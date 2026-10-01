use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use rand::{distr::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sudoku_engine::{
    board::Board,
    generator::{generate_with_difficulty, Difficulty},
    solver::solve,
};
use tokio::sync::{mpsc, RwLock};
use tower_http::cors::CorsLayer;

type Rooms = Arc<RwLock<HashMap<String, Room>>>;

const DEFAULT_DIFFICULTY: u8 = 1;
const DEFAULT_MAX_MISTAKES: u8 = 3;
const DEFAULT_TIMED: bool = false;
const DEFAULT_TIME_LIMIT_MINUTES: Option<u16> = None;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum BoardMode {
    Shared,
    Separate,
}

impl Default for BoardMode {
    fn default() -> Self {
        Self::Shared
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RoomSettings {
    board_mode: BoardMode,
    difficulty: u8,
    max_mistakes: u8,
    timed: bool,
    time_limit_minutes: Option<u16>,
}

impl Default for RoomSettings {
    fn default() -> Self {
        Self {
            board_mode: BoardMode::Shared,
            difficulty: DEFAULT_DIFFICULTY,
            max_mistakes: DEFAULT_MAX_MISTAKES,
            timed: DEFAULT_TIMED,
            time_limit_minutes: DEFAULT_TIME_LIMIT_MINUTES,
        }
    }
}

impl RoomSettings {
    fn validate(&self) -> Result<(), String> {
        if self.difficulty > 2 {
            return Err("difficulty must be 0 (Easy), 1 (Medium), or 2 (Hard)".into());
        }

        if self.max_mistakes > 10 {
            return Err("max mistakes must be between 0 and 10".into());
        }

        if self.timed {
            let Some(minutes) = self.time_limit_minutes else {
                return Err("timed games require a time limit".into());
            };

            if !matches!(minutes, 5 | 10 | 15 | 30 | 45 | 60) {
                return Err(
                    "time limit must be 5, 10, 15, 30, 45, or 60 minutes".into(),
                );
            }
        } else if self.time_limit_minutes.is_some() {
            return Err("untimed games cannot have a time limit".into());
        }

        Ok(())
    }

    fn difficulty(&self) -> Difficulty {
        match self.difficulty {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            2 => Difficulty::Hard,
            _ => Difficulty::Medium,
        }
    }
}

#[derive(Clone)]
struct Player {
    id: u64,
    name: String,
    tx: mpsc::UnboundedSender<ServerMessage>,
}

struct PlayerState {
    board: Board,
    mistakes: u8,
    score: i32,
}

struct Room {
    players: Vec<Player>,
    puzzle: Board,
    solution: Board,
    shared_board: Board,
    player_states: HashMap<u64, PlayerState>,
    settings: RoomSettings,
    started_at: Instant,
    finished: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMessage {
    CreateRoom {
        name: String,
        settings: RoomSettings,
    },
    JoinRoom {
        room_code: String,
        name: String,
    },
    MakeMove {
        row: usize,
        col: usize,
        value: u8,
    },
    ClearMove {
        row: usize,
        col: usize,
    },
}

#[derive(Debug, Clone, Serialize)]
struct PlayerScore {
    player_id: u64,
    score: i32,
    mistakes_used: u8,
    mistakes_allowed: u8,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMessage {
    RoomCreated {
        room_code: String,
        settings: RoomSettings,
        player_name: String,
        player_id: u64,
    },
    RoomJoined {
        room_code: String,
        player_name: String,
        player_id: u64,
    },
    PlayerJoined {
        player_name: String,
    },
    GameStarted {
        puzzle: Vec<u8>,
        settings: RoomSettings,
        remaining_seconds: Option<u64>,
        player_one_name: String,
        player_two_name: String,
        player_one_id: u64,
        player_two_id: u64,
        your_player_id: u64,
        scores: Vec<PlayerScore>,
    },
    MoveAccepted {
        row: usize,
        col: usize,
        value: u8,
        board: Vec<u8>,
        player_id: u64,
        player_slot: u8,
        score: i32,
        mistakes_used: u8,
        mistakes_allowed: u8,
    },
    MoveRejected {
        row: usize,
        col: usize,
        value: u8,
        player_id: u64,
        score: i32,
        mistakes_used: u8,
        mistakes_allowed: u8,
    },
    MoveCleared {
        row: usize,
        col: usize,
        board: Vec<u8>,
        player_id: u64,
    },
    ScoreUpdated {
        player_id: u64,
        score: i32,
        mistakes_used: u8,
        mistakes_allowed: u8,
    },
    GameFinished {
        winner_id: Option<u64>,
        scores: Vec<PlayerScore>,
    },
    GameTimedOut {
        scores: Vec<PlayerScore>,
    },
    MistakeLimitReached {
        mistakes_allowed: u8,
        you_lost: bool,
        loser_name: String,
        scores: Vec<PlayerScore>,
    },
    Error {
        message: String,
    },
}

#[derive(Clone)]
struct AppState {
    rooms: Rooms,
    next_player_id: Arc<AtomicU64>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        rooms: Arc::new(RwLock::new(HashMap::new())),
        next_player_id: Arc::new(AtomicU64::new(1)),
    };

    let app = Router::new()
        .route("/", get(health))
        .route("/ws", get(websocket_handler))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());

    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind server");

    println!("sudoku-server listening on {addr}");

    println!("sudoku-server listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

async fn health() -> &'static str {
    "sudo.ku server is running"
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let player_id = state
        .next_player_id
        .fetch_add(1, Ordering::Relaxed);

    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();

    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let json = serde_json::to_string(&message)
                .expect("failed to serialize server message");

            if sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(message)) = receiver.next().await {
        let Message::Text(text) = message else {
            continue;
        };

        let response = match serde_json::from_str::<ClientMessage>(&text) {
            Ok(ClientMessage::CreateRoom { name, settings }) => {
                Some(create_room(&state, player_id, tx.clone(), name, settings).await)
            }
            Ok(ClientMessage::JoinRoom { room_code, name }) => {
                Some(join_room(&state, player_id, tx.clone(), room_code, name).await)
            }
            Ok(ClientMessage::MakeMove { row, col, value }) => {
                make_move(&state, player_id, row, col, value).await
            }
            Ok(ClientMessage::ClearMove { row, col }) => {
                clear_move(&state, player_id, row, col).await
            }
            Err(error) => Some(ServerMessage::Error {
                message: format!("invalid message: {error}"),
            }),
        };

        if let Some(response) = response {
            let _ = tx.send(response);
        }
    }

    remove_player(&state, player_id).await;
    send_task.abort();
}

async fn create_room(
    state: &AppState,
    player_id: u64,
    player_tx: mpsc::UnboundedSender<ServerMessage>,
    name: String,
    settings: RoomSettings,
) -> ServerMessage {
    let name = name.trim().to_string();

    if name.is_empty() {
        return ServerMessage::Error {
            message: "player name cannot be empty".into(),
        };
    }

    if name.len() > 24 {
        return ServerMessage::Error {
            message: "player name must be 24 characters or fewer".into(),
        };
    }

    if let Err(error) = settings.validate() {
        return ServerMessage::Error { message: error };
    }

    let mut rooms = state.rooms.write().await;

    let room_code = loop {
        let code = generate_room_code();
        if !rooms.contains_key(&code) {
            break code;
        }
    };

    let puzzle = match generate_with_difficulty(settings.difficulty()) {
        Some(puzzle) => puzzle,
        None => {
            return ServerMessage::Error {
                message: "failed to generate puzzle".into(),
            };
        }
    };

    let mut solution = puzzle.clone();

    if !solve(&mut solution) {
        return ServerMessage::Error {
            message: "failed to solve generated puzzle".into(),
        };
    }

    let mut player_states = HashMap::new();
    player_states.insert(
        player_id,
        PlayerState {
            board: puzzle.clone(),
            mistakes: 0,
            score: 0,
        },
    );

    rooms.insert(
        room_code.clone(),
        Room {
            players: vec![Player {
                id: player_id,
                name: name.clone(),
                tx: player_tx,
            }],
            puzzle: puzzle.clone(),
            solution,
            shared_board: puzzle,
            player_states,
            settings: settings.clone(),
            started_at: Instant::now(),
            finished: false,
        },
    );

    println!("created room: {room_code} by player {player_id} ({name})");

    ServerMessage::RoomCreated {
        room_code,
        settings,
        player_name: name,
        player_id,
    }
}

async fn join_room(
    state: &AppState,
    player_id: u64,
    player_tx: mpsc::UnboundedSender<ServerMessage>,
    room_code: String,
    name: String,
) -> ServerMessage {
    let room_code = room_code.trim().to_uppercase();
    let name = name.trim().to_string();

    if name.is_empty() {
        return ServerMessage::Error {
            message: "player name cannot be empty".into(),
        };
    }

    if name.len() > 24 {
        return ServerMessage::Error {
            message: "player name must be 24 characters or fewer".into(),
        };
    }

    let mut rooms = state.rooms.write().await;

    let Some(room) = rooms.get_mut(&room_code) else {
        return ServerMessage::Error {
            message: "room not found".into(),
        };
    };

    if room.players.len() >= 2 {
        return ServerMessage::Error {
            message: "room is full".into(),
        };
    }

    if room.finished {
        return ServerMessage::Error {
            message: "game has already finished".into(),
        };
    }

    let player_one = room.players[0].clone();

    room.players.push(Player {
        id: player_id,
        name: name.clone(),
        tx: player_tx,
    });

    room.player_states.insert(
        player_id,
        PlayerState {
            board: room.puzzle.clone(),
            mistakes: 0,
            score: 0,
        },
    );

    let _ = player_one.tx.send(ServerMessage::PlayerJoined {
        player_name: name.clone(),
    });

    let player_one_name = room.players[0].name.clone();
    let player_two_name = room.players[1].name.clone();
    let player_one_id = room.players[0].id;
    let player_two_id = room.players[1].id;
    let remaining_seconds = room_remaining_seconds(room);
    let scores = player_scores(room);

    for player in &room.players {
        let _ = player.tx.send(ServerMessage::GameStarted {
            puzzle: board_to_vec(&room.puzzle),
            settings: room.settings.clone(),
            remaining_seconds,
            player_one_name: player_one_name.clone(),
            player_two_name: player_two_name.clone(),
            player_one_id,
            player_two_id,
            your_player_id: player.id,
            scores: scores.clone(),
        });
    }

    println!(
        "game started in room: {room_code} ({player_one_name} vs {player_two_name})"
    );

    ServerMessage::RoomJoined {
        room_code,
        player_name: player_two_name,
        player_id,
    }
}

async fn make_move(
    state: &AppState,
    player_id: u64,
    row: usize,
    col: usize,
    value: u8,
) -> Option<ServerMessage> {
    if row >= 9 || col >= 9 || !(1..=9).contains(&value) {
        return Some(ServerMessage::Error {
            message: "invalid move".into(),
        });
    }

    let mut rooms = state.rooms.write().await;

    let Some(room) = find_room_for_player(&mut rooms, player_id) else {
        return Some(ServerMessage::Error {
            message: "player is not in a room".into(),
        });
    };

    if room.finished {
        return Some(ServerMessage::Error {
            message: "game has finished".into(),
        });
    }

    if game_has_timed_out(room) {
        room.finished = true;
        let scores = player_scores(room);
        broadcast(&room.players, ServerMessage::GameTimedOut { scores });
        return None;
    }

    let settings = room.settings.clone();
    let player_slot = room
        .players
        .iter()
        .position(|player| player.id == player_id)
        .map(|index| (index + 1) as u8)
        .unwrap_or(1);
    let mistakes_used = room.player_states.get(&player_id)?.mistakes;
    let score = room.player_states.get(&player_id)?.score;
    let mistakes_allowed = settings.max_mistakes;

    let board_value = match settings.board_mode {
        BoardMode::Shared => room.shared_board.get(row, col),
        BoardMode::Separate => room.player_states.get(&player_id)?.board.get(row, col),
    };

    if board_value != 0 {
        return Some(ServerMessage::MoveRejected {
            row,
            col,
            value,
            player_id,
            score,
            mistakes_used,
            mistakes_allowed,
        });
    }

    if room.solution.get(row, col) != value {
        let new_mistakes = mistakes_used.saturating_add(1);
        let new_score = score - 5;

        if let Some(player_state) = room.player_states.get_mut(&player_id) {
            player_state.mistakes = new_mistakes;
            player_state.score = new_score;
        }

        let rejection = ServerMessage::MoveRejected {
            row,
            col,
            value,
            player_id,
            score: new_score,
            mistakes_used: new_mistakes,
            mistakes_allowed,
        };

        broadcast(
            &room.players,
            ServerMessage::ScoreUpdated {
                player_id,
                score: new_score,
                mistakes_used: new_mistakes,
                mistakes_allowed,
            },
        );

        if new_mistakes >= mistakes_allowed {
            room.finished = true;

            let loser_name = room
                .players
                .iter()
                .find(|player| player.id == player_id)
                .map(|player| player.name.clone())
                .unwrap_or_else(|| "Player".into());

            let scores = player_scores(room);

            for player in &room.players {
                let _ = player.tx.send(ServerMessage::MistakeLimitReached {
                    mistakes_allowed,
                    you_lost: player.id == player_id,
                    loser_name: loser_name.clone(),
                    scores: scores.clone(),
                });
            }

            return Some(rejection);
        }

        return Some(rejection);
    }

    let can_place = match settings.board_mode {
        BoardMode::Shared => room.shared_board.can_place(row, col, value),
        BoardMode::Separate => room
            .player_states
            .get(&player_id)?
            .board
            .can_place(row, col, value),
    };

    if !can_place {
        return Some(ServerMessage::MoveRejected {
            row,
            col,
            value,
            player_id,
            score,
            mistakes_used,
            mistakes_allowed,
        });
    }

    if settings.board_mode == BoardMode::Shared {
        room.shared_board.set(row, col, value);
    } else {
        room.player_states.get_mut(&player_id)?.board.set(row, col, value);
    }

    let new_score = score + 10;

    if let Some(player_state) = room.player_states.get_mut(&player_id) {
        player_state.score = new_score;
    }

    let board_vec = match settings.board_mode {
        BoardMode::Shared => board_to_vec(&room.shared_board),
        BoardMode::Separate => board_to_vec(&room.player_states.get(&player_id)?.board),
    };

    let accepted = ServerMessage::MoveAccepted {
        row,
        col,
        value,
        board: board_vec,
        player_id,
        player_slot,
        score: new_score,
        mistakes_used,
        mistakes_allowed,
    };

    if settings.board_mode == BoardMode::Shared {
        broadcast(&room.players, accepted);
    } else {
        if let Some(player) = room.players.iter().find(|p| p.id == player_id) {
            let _ = player.tx.send(accepted);
        }
    }

    broadcast(
        &room.players,
        ServerMessage::ScoreUpdated {
            player_id,
            score: new_score,
            mistakes_used,
            mistakes_allowed,
        },
    );

    let completed = match settings.board_mode {
        BoardMode::Shared => is_board_complete(&room.shared_board),
        BoardMode::Separate => {
            is_board_complete(&room.player_states.get(&player_id)?.board)
        }
    };

    if completed {
        let completion_bonus = 50;

        if let Some(player_state) = room.player_states.get_mut(&player_id) {
            player_state.score += completion_bonus;
        }

        let final_score = room.player_states.get(&player_id)?.score;

        broadcast(
            &room.players,
            ServerMessage::ScoreUpdated {
                player_id,
                score: final_score,
                mistakes_used,
                mistakes_allowed,
            },
        );

        room.finished = true;

        let winner_id = if settings.board_mode == BoardMode::Separate {
            Some(player_id)
        } else {
            let scores = player_scores(room);
            let max_score = scores.iter().map(|s| s.score).max().unwrap_or(0);
            let leaders: Vec<_> = scores
                .iter()
                .filter(|s| s.score == max_score)
                .collect();

            if leaders.len() == 1 {
                Some(leaders[0].player_id)
            } else {
                None
            }
        };

        broadcast(
            &room.players,
            ServerMessage::GameFinished {
                winner_id,
                scores: player_scores(room),
            },
        );
    }

    None
}

async fn clear_move(
    state: &AppState,
    player_id: u64,
    row: usize,
    col: usize,
) -> Option<ServerMessage> {
    if row >= 9 || col >= 9 {
        return Some(ServerMessage::Error {
            message: "invalid cell".into(),
        });
    }

    let mut rooms = state.rooms.write().await;

    let Some(room) = find_room_for_player(&mut rooms, player_id) else {
        return Some(ServerMessage::Error {
            message: "player is not in a room".into(),
        });
    };

    if room.finished {
        return Some(ServerMessage::Error {
            message: "game has finished".into(),
        });
    }

    if game_has_timed_out(room) {
        room.finished = true;
        let scores = player_scores(room);
        broadcast(&room.players, ServerMessage::GameTimedOut { scores });
        return None;
    }

    let settings = room.settings.clone();

    let board = match settings.board_mode {
        BoardMode::Shared => &room.shared_board,
        BoardMode::Separate => &room.player_states.get(&player_id)?.board,
    };

    if room.solution.get(row, col) == board.get(row, col) {
        return Some(ServerMessage::Error {
            message: "correct moves cannot be cleared".into(),
        });
    }

    Some(ServerMessage::Error {
        message: "there is no incorrect move to clear".into(),
    })
}

fn find_room_for_player<'a>(
    rooms: &'a mut HashMap<String, Room>,
    player_id: u64,
) -> Option<&'a mut Room> {
    rooms
        .values_mut()
        .find(|room| room.players.iter().any(|player| player.id == player_id))
}

fn broadcast(players: &[Player], message: ServerMessage) {
    for player in players {
        let _ = player.tx.send(message.clone());
    }
}

async fn remove_player(state: &AppState, player_id: u64) {
    let mut rooms = state.rooms.write().await;
    let mut empty_rooms = Vec::new();

    for (room_code, room) in rooms.iter_mut() {
        if !room.players.iter().any(|player| player.id == player_id) {
            continue;
        }

        room.players.retain(|player| player.id != player_id);
        room.player_states.remove(&player_id);

        if room.players.is_empty() {
            empty_rooms.push(room_code.clone());
        } else {
            let _ = room.players[0].tx.send(ServerMessage::Error {
                message: "opponent disconnected".into(),
            });
        }
    }

    for room_code in empty_rooms {
        rooms.remove(&room_code);
    }
}

fn game_has_timed_out(room: &Room) -> bool {
    let Some(limit_minutes) = room.settings.time_limit_minutes else {
        return false;
    };

    room.started_at.elapsed() >= Duration::from_secs(u64::from(limit_minutes) * 60)
}

fn room_remaining_seconds(room: &Room) -> Option<u64> {
    let limit_minutes = room.settings.time_limit_minutes?;
    let total_seconds = u64::from(limit_minutes) * 60;
    let elapsed = room.started_at.elapsed().as_secs();
    Some(total_seconds.saturating_sub(elapsed))
}

fn player_scores(room: &Room) -> Vec<PlayerScore> {
    room.players
        .iter()
        .map(|player| {
            let state = room
                .player_states
                .get(&player.id)
                .expect("player state must exist");

            PlayerScore {
                player_id: player.id,
                score: state.score,
                mistakes_used: state.mistakes,
                mistakes_allowed: room.settings.max_mistakes,
            }
        })
        .collect()
}

fn is_board_complete(board: &Board) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if board.get(row, col) == 0 {
                return false;
            }
        }
    }

    true
}

fn board_to_vec(board: &Board) -> Vec<u8> {
    let mut cells = Vec::with_capacity(81);

    for row in 0..9 {
        for col in 0..9 {
            cells.push(board.get(row, col));
        }
    }

    cells
}

fn generate_room_code() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}
