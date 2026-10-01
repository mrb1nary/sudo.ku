<script lang="ts">
    import { initializeSudoku, SudokuGame } from "./sudoku";
    import SudokuBoard, { type ServerMove } from "./SudokuBoard.svelte";
    import MultiplayerGame from "./MultiplayerGame.svelte";

    type BoardMode = "shared" | "separate";

    interface RoomSettings {
        board_mode: BoardMode;
        difficulty: number;
        max_mistakes: number;
        timed: boolean;
        time_limit_minutes: number | null;
    }

    interface PlayerScore {
        player_id: number;
        score: number;
        mistakes_used: number;
        mistakes_allowed: number;
    }

    type GameResult = "win" | "loss" | "draw" | null;
    type EndReason = "mistakes" | "timeout" | "completed" | null;

    const defaultSettings: RoomSettings = {
        board_mode: "shared",
        difficulty: 1,
        max_mistakes: 3,
        timed: false,
        time_limit_minutes: null,
    };

    let roomCode = $state("");
    let createdRoomCode = $state("");
    let playerName = $state("");
    let joinName = $state("");
    let status = $state("");
    let connected = $state(false);
    let socket = $state<WebSocket | null>(null);

    let puzzle = $state<number[] | null>(null);
    let multiplayerGame = $state<SudokuGame | undefined>(undefined);
    let syncVersion = $state(0);

    let rejectedCell = $state<number | null>(null);
    let rejectedValue = $state<number | null>(null);

    let serverMoves = $state(new Map<number, ServerMove>());

    let boardMode = $state<BoardMode>(defaultSettings.board_mode);
    let difficulty = $state(defaultSettings.difficulty);
    let maxMistakes = $state(defaultSettings.max_mistakes);
    let timed = $state(defaultSettings.timed);
    let timeLimitMinutes = $state(15);
    let settingsOpen = $state(false);

    let playerOneName = $state("");
    let playerTwoName = $state("");
    let playerOneId = $state<number | null>(null);
    let playerTwoId = $state<number | null>(null);
    let yourPlayerId = $state<number | null>(null);

    let scores = $state(new Map<number, PlayerScore>());
    let mistakesUsed = $state(0);
    let mistakesAllowed = $state(defaultSettings.max_mistakes);
    let remainingSeconds = $state<number | null>(null);

    let timerId: ReturnType<typeof setInterval> | null = null;
    let gameResult = $state<GameResult>(null);
    let endReason = $state<EndReason>(null);
    let gameEnded = $state(false);

    function getSettings(): RoomSettings {
        return {
            board_mode: boardMode,
            difficulty,
            max_mistakes: maxMistakes,
            timed,
            time_limit_minutes: timed ? timeLimitMinutes : null,
        };
    }

    function applySettings(settings: RoomSettings) {
        boardMode = settings.board_mode;
        difficulty = settings.difficulty;
        maxMistakes = settings.max_mistakes;
        timed = settings.timed;

        if (settings.time_limit_minutes !== null) {
            timeLimitMinutes = settings.time_limit_minutes;
        }
    }

    function applyScores(nextScores: PlayerScore[]) {
        scores = new Map(
            nextScores.map((score) => [score.player_id, score]),
        );

        const ownScore = yourPlayerId === null
            ? undefined
            : scores.get(yourPlayerId);

        if (ownScore) {
            mistakesUsed = ownScore.mistakes_used;
            mistakesAllowed = ownScore.mistakes_allowed;
        }
    }

    function setServerMove(
        row: number,
        col: number,
        value: number,
        playerId: number,
        playerSlot: number,
    ) {
        const cell = row * 9 + col;
        serverMoves = new Map(serverMoves);
        serverMoves.set(cell, {
            value,
            playerId,
            playerSlot,
        });
    }

    function removeServerMove(row: number, col: number) {
        const cell = row * 9 + col;
        serverMoves = new Map(serverMoves);
        serverMoves.delete(cell);
    }

    async function connect(): Promise<WebSocket> {
        if (socket?.readyState === WebSocket.OPEN) {
            return socket;
        }

        return new Promise((resolve, reject) => {
            const wsUrl = import.meta.env.VITE_WS_URL ?? "ws://127.0.0.1:3000/ws";
            const ws = new WebSocket(wsUrl);
            socket = ws;

            ws.onopen = () => {
                connected = true;
                status = "Connected to server";
                resolve(ws);
            };

            ws.onmessage = async (event) => {
                const message = JSON.parse(event.data);

                switch (message.type) {
                    case "room_created":
                        createdRoomCode = message.room_code;
                        roomCode = message.room_code;
                        applySettings(message.settings);
                        yourPlayerId = message.player_id;
                        status = `Room created: ${message.room_code}`;
                        break;

                    case "room_joined":
                        roomCode = message.room_code;
                        yourPlayerId = message.player_id;
                        status = `Joined room: ${message.room_code}`;
                        break;

                    case "player_joined":
                        status = `${message.player_name} joined the room`;
                        break;

                    case "game_started":
                        puzzle = message.puzzle;
                        await initializeSudoku();

                        multiplayerGame = SudokuGame.from_puzzle(message.puzzle);

                        if (!multiplayerGame) {
                            status = "Failed to load puzzle";
                            break;
                        }

                        applySettings(message.settings);

                        playerOneName = message.player_one_name;
                        playerTwoName = message.player_two_name;
                        playerOneId = message.player_one_id;
                        playerTwoId = message.player_two_id;
                        yourPlayerId = message.your_player_id;

                        remainingSeconds = message.remaining_seconds;
                        serverMoves = new Map();
                        rejectedCell = null;
                        rejectedValue = null;
                        gameResult = null;
                        endReason = null;
                        gameEnded = false;

                        applyScores(message.scores);
                        startTimer();

                        status = `${playerOneName} vs ${playerTwoName}`;
                        syncVersion += 1;
                        break;

                    case "move_accepted":
                        setServerMove(
                            message.row,
                            message.col,
                            message.value,
                            message.player_id,
                            message.player_slot,
                        );

                        rejectedCell = null;
                        rejectedValue = null;

                        mistakesUsed = message.mistakes_used;
                        mistakesAllowed = message.mistakes_allowed;

                        scores = new Map(scores);
                        scores.set(message.player_id, {
                            player_id: message.player_id,
                            score: message.score,
                            mistakes_used: message.mistakes_used,
                            mistakes_allowed: message.mistakes_allowed,
                        });

                        syncVersion += 1;
                        status = "Move accepted";
                        break;

                    case "move_rejected":
                        rejectedCell = message.row * 9 + message.col;
                        rejectedValue = message.value;

                        if (message.player_id === yourPlayerId) {
                            mistakesUsed = message.mistakes_used;
                            mistakesAllowed = message.mistakes_allowed;
                        }

                        scores = new Map(scores);
                        scores.set(message.player_id, {
                            player_id: message.player_id,
                            score: message.score,
                            mistakes_used: message.mistakes_used,
                            mistakes_allowed: message.mistakes_allowed,
                        });

                        status = "Incorrect move";
                        syncVersion += 1;
                        break;

                    case "move_cleared":
                        removeServerMove(message.row, message.col);
                        rejectedCell = null;
                        rejectedValue = null;
                        syncVersion += 1;
                        status = "Move cleared";
                        break;

                    case "score_updated":
                        scores = new Map(scores);
                        scores.set(message.player_id, {
                            player_id: message.player_id,
                            score: message.score,
                            mistakes_used: message.mistakes_used,
                            mistakes_allowed: message.mistakes_allowed,
                        });

                        if (message.player_id === yourPlayerId) {
                            mistakesUsed = message.mistakes_used;
                            mistakesAllowed = message.mistakes_allowed;
                        }
                        break;

                    case "game_finished":
                        applyScores(message.scores);

                        if (message.winner_id === null) {
                            finishGame("draw", "completed");
                            status = "🤝 Game ended in a draw";
                        } else if (message.winner_id === yourPlayerId) {
                            finishGame("win", "completed");
                            status = "🏆 You win!";
                        } else {
                            finishGame("loss", "completed");
                            status = "😔 You lose";
                        }
                        break;

                    case "game_timed_out":
                        applyScores(message.scores);
                        finishGame("draw", "timeout");
                        status = "⏱️ Time's up!";
                        break;

                    case "mistake_limit_reached":
                        applyScores(message.scores);

                        finishGame(
                            message.you_lost ? "loss" : "win",
                            "mistakes",
                        );

                        status = message.you_lost
                            ? "You reached the mistake limit"
                            : `${message.loser_name} reached the mistake limit`;

                        break;

                    case "error":
                        status = message.message;
                        break;

                    default:
                        console.warn("Unknown server message:", message);
                }
            };

            ws.onclose = () => {
                connected = false;
                socket = null;
                stopTimer();
                status = "Disconnected from server";
            };

            ws.onerror = () => {
                status = "WebSocket connection failed";
                reject(new Error("WebSocket connection failed"));
            };
        });
    }

    async function createRoom() {
        const name = playerName.trim();

        if (!name) {
            status = "Enter your name first";
            return;
        }

        try {
            const ws = await connect();

            ws.send(
                JSON.stringify({
                    type: "create_room",
                    name,
                    settings: getSettings(),
                }),
            );
        } catch {
            // Connection error is already shown.
        }
    }

    async function joinRoom() {
        const code = roomCode.trim().toUpperCase();
        const name = joinName.trim();

        if (!name) {
            status = "Enter your name first";
            return;
        }

        if (!code) {
            status = "Enter a room code";
            return;
        }

        try {
            const ws = await connect();

            ws.send(
                JSON.stringify({
                    type: "join_room",
                    room_code: code,
                    name,
                }),
            );
        } catch {
            // Connection error is already shown.
        }
    }

    function sendMove(row: number, col: number, value: number) {
        if (!socket || socket.readyState !== WebSocket.OPEN) {
            status = "Not connected to server";
            return;
        }

        if (gameEnded) return;

        rejectedCell = null;
        rejectedValue = null;

        socket.send(
            JSON.stringify({
                type: "make_move",
                row,
                col,
                value,
            }),
        );
    }

    function sendClear(row: number, col: number) {
        if (!socket || socket.readyState !== WebSocket.OPEN) {
            status = "Not connected to server";
            return;
        }

        if (gameEnded) return;

        rejectedCell = null;
        rejectedValue = null;

        socket.send(
            JSON.stringify({
                type: "clear_move",
                row,
                col,
            }),
        );
    }

    function startTimer() {
        stopTimer();

        if (remainingSeconds === null) return;

        timerId = setInterval(() => {
            if (remainingSeconds === null) {
                stopTimer();
                return;
            }

            if (remainingSeconds <= 0) {
                remainingSeconds = 0;
                stopTimer();
                return;
            }

            remainingSeconds -= 1;
        }, 1000);
    }

    function stopTimer() {
        if (timerId !== null) {
            clearInterval(timerId);
            timerId = null;
        }
    }

    function finishGame(
        result: Exclude<GameResult, null>,
        reason: Exclude<EndReason, null>,
    ) {
        gameResult = result;
        endReason = reason;
        gameEnded = true;
        stopTimer();
    }

    function formatTime(seconds: number | null): string {
        if (seconds === null) return "∞";

        const minutes = Math.floor(seconds / 60);
        const remaining = seconds % 60;

        return `${minutes}:${remaining.toString().padStart(2, "0")}`;
    }

    function playerColor(id: number | null): string {
        if (id === playerOneId) return "player-one";
        if (id === playerTwoId) return "player-two";
        return "";
    }

    function playerLabel(id: number | null): string {
        if (id === playerOneId) return playerOneName;
        if (id === playerTwoId) return playerTwoName;
        return "";
    }
</script>

{#if multiplayerGame}
    <MultiplayerGame
        game={multiplayerGame}
        boardMode={boardMode}
        playerOneName={playerOneName}
        playerTwoName={playerTwoName}
        playerOneId={playerOneId}
        playerTwoId={playerTwoId}
        yourPlayerId={yourPlayerId}
        scores={scores}
        mistakesUsed={mistakesUsed}
        mistakesAllowed={mistakesAllowed}
        remainingSeconds={remainingSeconds}
        gameEnded={gameEnded}
        gameResult={gameResult}
        endReason={endReason}
        status={status}
        syncVersion={syncVersion}
        serverMoves={serverMoves}
        rejectedCell={rejectedCell}
        rejectedValue={rejectedValue}
        onMove={sendMove}
        onClear={sendClear}
        formatTime={formatTime}
    />
{:else}
    <div class="multiplayer">
        <h2>Multiplayer</h2>

        <p class:connected class:disconnected={!connected}>
            {connected ? "● Connected" : "○ Disconnected"}
        </p>

        <section class="create-section">
            <div class="section-title">
                <h3>Create a Room</h3>

                <button
                    class="settings-button"
                    type="button"
                    aria-label="Game settings"
                    aria-expanded={settingsOpen}
                    onclick={() => (settingsOpen = !settingsOpen)}
                >
                    ⚙
                </button>
            </div>

            <label>
                <span>Your name</span>
                <input
                    bind:value={playerName}
                    maxlength="24"
                    placeholder="Enter your name"
                    onkeydown={(event) => {
                        if (event.key === "Enter") createRoom();
                    }}
                />
            </label>

            {#if settingsOpen}
                <div class="settings">
                    <div class="settings-header">
                        <strong>Game Settings</strong>

                        <button
                            type="button"
                            class="close-settings"
                            onclick={() => (settingsOpen = false)}
                        >
                            ×
                        </button>
                    </div>

                    <label>
                        <span>Board</span>
                        <select bind:value={boardMode}>
                            <option value="shared">Shared Board</option>
                            <option value="separate">Separate Boards</option>
                        </select>
                    </label>

                    <p class="setting-help">
                        {#if boardMode === "shared"}
                            Both players see and edit the same board.
                        {:else}
                            Both players solve independently. First to finish wins.
                        {/if}
                    </p>

                    <label>
                        <span>Difficulty</span>
                        <select bind:value={difficulty}>
                            <option value={0}>Easy</option>
                            <option value={1}>Medium</option>
                            <option value={2}>Hard</option>
                        </select>
                    </label>

                    <label>
                        <span>Maximum mistakes</span>
                        <select bind:value={maxMistakes}>
                            <option value={0}>0 — Instant loss</option>
                            <option value={1}>1</option>
                            <option value={2}>2</option>
                            <option value={3}>3</option>
                            <option value={4}>4</option>
                            <option value={5}>5</option>
                            <option value={10}>10</option>
                        </select>
                    </label>

                    <label>
                        <span>Timer</span>
                        <select
                            value={timed ? "timed" : "untimed"}
                            onchange={(event) => {
                                timed = event.currentTarget.value === "timed";
                                if (timed && !timeLimitMinutes) {
                                    timeLimitMinutes = 15;
                                }
                            }}
                        >
                            <option value="untimed">Untimed</option>
                            <option value="timed">Timed</option>
                        </select>
                    </label>

                    {#if timed}
                        <label>
                            <span>Time limit</span>
                            <select bind:value={timeLimitMinutes}>
                                <option value={5}>5 minutes</option>
                                <option value={10}>10 minutes</option>
                                <option value={15}>15 minutes</option>
                                <option value={30}>30 minutes</option>
                                <option value={45}>45 minutes</option>
                                <option value={60}>60 minutes</option>
                            </select>
                        </label>
                    {/if}
                </div>
            {/if}

            <button
                type="button"
                class="primary-button"
                onclick={createRoom}
                disabled={connected && !!createdRoomCode}
            >
                Create Room
            </button>
        </section>

        {#if createdRoomCode}
            <div class="room">
                <span>Room Code</span>
                <strong>{createdRoomCode}</strong>
                <p>Share this code with your opponent.</p>
            </div>
        {/if}

        <div class="divider"><span>OR</span></div>

        <section class="join-section">
            <h3>Join a Room</h3>

            <label>
                <span>Your name</span>
                <input
                    bind:value={joinName}
                    maxlength="24"
                    placeholder="Enter your name"
                    onkeydown={(event) => {
                        if (event.key === "Enter") joinRoom();
                    }}
                />
            </label>

            <label>
                <span>Room code</span>
                <input
                    bind:value={roomCode}
                    maxlength="6"
                    placeholder="ABC123"
                    onkeydown={(event) => {
                        if (event.key === "Enter") joinRoom();
                    }}
                />
            </label>

            <button
                type="button"
                class="primary-button"
                onclick={joinRoom}
                disabled={!joinName.trim() || !roomCode.trim()}
            >
                Join Room
            </button>
        </section>

        {#if status}
            <p class="status">{status}</p>
        {/if}
    </div>
{/if}

<style>
    .multiplayer {
        width: min(90vw, 600px);
        margin: 3rem auto;
        text-align: center;
    }

    h2 { margin: 0 0 0.5rem; }
    h3 { margin: 0; font-size: 1rem; }

    .connected,
    .disconnected {
        margin: 0 0 2rem;
    }

    .connected { color: var(--success); }
    .disconnected { color: var(--text-muted); }

    .create-section,
    .join-section {
        display: flex;
        flex-direction: column;
        gap: 0.8rem;
        padding: 1.25rem;
        border: 1px solid var(--border);
        border-radius: 12px;
        background: var(--surface);
        text-align: left;
    }

    .section-title {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .settings-button {
        width: 36px;
        height: 36px;
        padding: 0;
        border: 1px solid var(--border);
        border-radius: 8px;
        background: var(--button-bg);
        color: var(--button-text);
        font-size: 1.1rem;
        cursor: pointer;
    }

    .settings-button:hover { background: var(--button-hover); }

    label {
        display: flex;
        flex-direction: column;
        gap: 0.35rem;
    }

    label span {
        color: var(--text-muted);
        font-size: 0.85rem;
    }

    input,
    select {
        width: 100%;
        padding: 0.75rem 0.8rem;
        border: 1px solid var(--border);
        border-radius: 8px;
        background: var(--input-bg);
        color: var(--text);
        font: inherit;
    }

    input:focus,
    select:focus {
        border-color: var(--accent);
        outline: none;
    }

    .primary-button {
        width: 100%;
        padding: 0.8rem 1rem;
        border: 1px solid var(--accent);
        border-radius: 8px;
        background: var(--accent);
        color: #fff;
        font: inherit;
        font-weight: 650;
        cursor: pointer;
    }

    .primary-button:hover:not(:disabled) {
        background: var(--accent-hover);
    }

    .primary-button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .settings {
        display: flex;
        flex-direction: column;
        gap: 0.8rem;
        padding: 1rem;
        border: 1px solid var(--border);
        border-radius: 10px;
        background: var(--surface-elevated);
    }

    .settings-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .close-settings {
        width: 28px;
        height: 28px;
        padding: 0;
        border: none;
        background: transparent;
        color: var(--text-muted);
        font-size: 1.4rem;
        cursor: pointer;
    }

    .setting-help {
        margin: -0.3rem 0 0.2rem;
        color: var(--text-muted);
        font-size: 0.8rem;
        line-height: 1.4;
    }

    .room {
        margin-top: 1rem;
        padding: 1.25rem;
        border: 1px solid var(--border);
        border-radius: 12px;
        background: var(--surface);
    }

    .room span {
        display: block;
        color: var(--text-muted);
        font-size: 0.85rem;
        margin-bottom: 0.35rem;
    }

    .room strong {
        display: block;
        font-size: 2rem;
        letter-spacing: 0.25rem;
    }

    .room p {
        margin: 0.5rem 0 0;
        color: var(--text-muted);
        font-size: 0.9rem;
    }

    .divider {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin: 1.5rem 0;
        color: var(--text-subtle);
    }

    .divider::before,
    .divider::after {
        content: "";
        flex: 1;
        height: 1px;
        background: var(--border);
    }

    .players {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1.5rem;
        margin-top: 1.5rem;
        padding: 0.9rem;
        border: 1px solid var(--border);
        border-radius: 10px;
        background: var(--surface);
        font-weight: 650;
    }

    .player {
        display: flex;
        align-items: center;
        gap: 0.45rem;
    }

    .player-dot {
        width: 9px;
        height: 9px;
        border-radius: 50%;
    }

    .player-one-dot { background: #60a5fa; }
    .player-two-dot { background: #fb923c; }

    .players strong {
        color: var(--accent);
        font-size: 0.8rem;
    }

    .scoreboard {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 0.75rem;
        margin-top: 0.75rem;
    }

    .score-card {
        display: flex;
        flex-direction: column;
        gap: 0.15rem;
        padding: 0.75rem;
        border: 1px solid var(--border);
        border-radius: 10px;
        background: var(--surface);
    }

    .score-card strong {
        font-size: 1.4rem;
    }

    .score-card span {
        color: var(--text-muted);
        font-size: 0.75rem;
    }

    .score-name {
        font-size: 0.85rem;
        font-weight: 700;
    }

    .score-name.player-one,
    .player-one-text { color: #60a5fa; }

    .score-name.player-two,
    .player-two-text { color: #fb923c; }

    .legend {
        display: flex;
        justify-content: center;
        gap: 1rem;
        margin: 0.75rem 0 0;
        font-size: 0.8rem;
    }

    .timer {
        margin-top: 1rem;
        font-size: 1.15rem;
        font-weight: 700;
        color: var(--accent);
    }

    .status {
        margin-top: 1rem;
        color: var(--text-muted);
    }

    .result {
        display: flex;
        flex-direction: column;
        gap: 0.35rem;
        margin-top: 1rem;
        padding: 1rem;
        border: 1px solid var(--border);
        border-radius: 10px;
        background: var(--surface);
    }

    .result strong { font-size: 1.25rem; }
    .result span { color: var(--text-muted); font-size: 0.9rem; }
    .result.win strong { color: var(--success); }
    .result.loss strong { color: var(--error); }
    .result.draw strong { color: var(--accent); }

    .game { margin-top: 2rem; }

    @media (max-width: 480px) {
        .multiplayer {
            width: min(calc(100% - 2rem), 600px);
            margin-top: 2rem;
        }

        .players {
            gap: 0.6rem;
            font-size: 0.9rem;
        }

        .scoreboard {
            grid-template-columns: 1fr 1fr;
        }
    }
</style>
