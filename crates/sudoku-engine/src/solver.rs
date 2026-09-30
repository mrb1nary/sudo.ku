use crate::board::Board;

const ALL_DIGITS: u16 = 0b1_1111_1111;

type CandidateCache = [[u16; 9]; 9];

#[derive(Debug, Default)]
pub struct SolverStats {
    pub recursive_calls: usize,
    pub forced_moves: usize,
    pub hidden_singles: usize,
    pub branches: usize,
    pub backtracks: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct SolverConfig {
    pub use_hidden_singles: bool,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            use_hidden_singles: true,
        }
    }
}

#[derive(Clone)]
pub struct SolverState {
    row_masks: [u16; 9],
    col_masks: [u16; 9],
    box_masks: [u16; 9],
}

impl SolverState {
    pub fn from_board(board: &Board) -> Option<Self> {
        let mut state = Self {
            row_masks: [0; 9],
            col_masks: [0; 9],
            box_masks: [0; 9],
        };

        for row in 0..9 {
            for col in 0..9 {
                let value = board.get(row, col);

                if value == 0 {
                    continue;
                }

                if !(1..=9).contains(&value) {
                    return None;
                }

                let bit = digit_bit(value);
                let box_index = box_index(row, col);

                // A duplicate in any unit makes the starting puzzle invalid.
                if state.row_masks[row] & bit != 0
                    || state.col_masks[col] & bit != 0
                    || state.box_masks[box_index] & bit != 0
                {
                    return None;
                }

                state.place(row, col, value);
            }
        }

        Some(state)
    }

    #[inline]
    fn place(&mut self, row: usize, col: usize, value: u8) {
        let bit = digit_bit(value);
        let box_index = box_index(row, col);

        self.row_masks[row] |= bit;
        self.col_masks[col] |= bit;
        self.box_masks[box_index] |= bit;
    }

    #[inline]
    fn candidate_mask(&self, row: usize, col: usize) -> u16 {
        let used =
            self.row_masks[row]
                | self.col_masks[col]
                | self.box_masks[box_index(row, col)];

        ALL_DIGITS & !used
    }
}

enum Undo {
    BoardCell {
        row: usize,
        col: usize,
    },
    RowMask {
        row: usize,
        previous: u16,
    },
    ColMask {
        col: usize,
        previous: u16,
    },
    BoxMask {
        index: usize,
        previous: u16,
    },
    Candidate {
        row: usize,
        col: usize,
        previous: u16,
    },
}

struct Trail {
    entries: Vec<Undo>,
}

impl Trail {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    #[inline]
    fn checkpoint(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    fn push(&mut self, undo: Undo) {
        self.entries.push(undo);
    }

    fn undo_to(
        &mut self,
        checkpoint: usize,
        board: &mut Board,
        state: &mut SolverState,
        candidates: &mut CandidateCache,
    ) {
        while self.entries.len() > checkpoint {
            match self.entries.pop().unwrap() {
                Undo::BoardCell { row, col } => {
                    board.clear(row, col);
                }
                Undo::RowMask { row, previous } => {
                    state.row_masks[row] = previous;
                }
                Undo::ColMask { col, previous } => {
                    state.col_masks[col] = previous;
                }
                Undo::BoxMask { index, previous } => {
                    state.box_masks[index] = previous;
                }
                Undo::Candidate {
                    row,
                    col,
                    previous,
                } => {
                    candidates[row][col] = previous;
                }
            }
        }
    }
}

pub fn solve(board: &mut Board) -> bool {
    solve_with_config(board, SolverConfig::default())
}

fn count_recursive(
    board: &mut Board,
    state: &mut SolverState,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
    config: SolverConfig,
    limit: usize,
) -> usize {
    let checkpoint = trail.checkpoint();

    // Constraint propagation.
    loop {
        if board_is_complete(board) {
            trail.undo_to(checkpoint, board, state, candidates);
            return 1;
        }

        if let Some((row, col, value)) = find_naked_single(board, candidates) {
            place_value(
                board,
                state,
                candidates,
                trail,
                row,
                col,
                value,
            );

            continue;
        }

        if config.use_hidden_singles {
            if let Some((row, col, value)) =
                find_hidden_single(board, candidates)
            {
                place_value(
                    board,
                    state,
                    candidates,
                    trail,
                    row,
                    col,
                    value,
                );

                continue;
            }
        }

        break;
    }

    let Some(((row, col), mut remaining)) =
        find_best_empty(board, candidates)
    else {
        trail.undo_to(checkpoint, board, state, candidates);
        return 1;
    };

    // Contradiction.
    if remaining == 0 {
        trail.undo_to(checkpoint, board, state, candidates);
        return 0;
    }

    let mut solutions = 0;

    while remaining != 0 && solutions < limit {
        let value = next_value(&mut remaining);
        let branch_checkpoint = trail.checkpoint();

        place_value(
            board,
            state,
            candidates,
            trail,
            row,
            col,
            value,
        );

        solutions += count_recursive(
            board,
            state,
            candidates,
            trail,
            config,
            limit - solutions,
        );

        trail.undo_to(branch_checkpoint, board, state, candidates);
    }

    trail.undo_to(checkpoint, board, state, candidates);

    solutions
}

pub fn solve_with_config(board: &mut Board, config: SolverConfig) -> bool {
    let Some(mut state) = SolverState::from_board(board) else {
        return false;
    };

    let mut candidates = build_candidate_cache(board, &state);
    let mut trail = Trail::new();

    solve_recursive(
        board,
        &mut state,
        &mut candidates,
        &mut trail,
        config,
        None,
    )
}

pub fn count_solutions(board: &mut Board, limit: usize) -> usize {
    count_solutions_with_config(board, limit, SolverConfig::default())
}

pub fn count_solutions_with_config(
    board: &mut Board,
    limit: usize,
    config: SolverConfig,
) -> usize {
    if limit == 0 {
        return 0;
    }

    let Some(mut state) = SolverState::from_board(board) else {
        return 0;
    };

    let mut candidates = build_candidate_cache(board, &state);
    let mut trail = Trail::new();

    count_recursive(
        board,
        &mut state,
        &mut candidates,
        &mut trail,
        config,
        limit,
    )
}

pub fn solve_with_stats(board: &mut Board) -> SolverStats {
    solve_with_stats_config(board, SolverConfig::default())
}

pub fn solve_with_stats_config(
    board: &mut Board,
    config: SolverConfig,
) -> SolverStats {
    let mut stats = SolverStats::default();

    let Some(mut state) = SolverState::from_board(board) else {
        return stats;
    };

    let mut candidates = build_candidate_cache(board, &state);
    let mut trail = Trail::new();

    solve_recursive(
        board,
        &mut state,
        &mut candidates,
        &mut trail,
        config,
        Some(&mut stats),
    );

    stats
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SolverProfile {
    pub clues: usize,
    pub recursive_calls: usize,
    pub forced_moves: usize,
    pub hidden_singles: usize,
    pub branches: usize,
    pub backtracks: usize,
}

fn count_clues(board: &Board) -> usize {
    let mut clues = 0;

    for row in 0..9 {
        for col in 0..9 {
            if board.get(row, col) != 0 {
                clues += 1;
            }
        }
    }

    clues
}

fn solve_recursive(
    board: &mut Board,
    state: &mut SolverState,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
    config: SolverConfig,
    mut stats: Option<&mut SolverStats>,
) -> bool {
    record_recursive_call(&mut stats);

    let checkpoint = trail.checkpoint();

    // Constraint propagation.
    loop {
        if board_is_complete(board) {
            return true;
        }

        if let Some((row, col, value)) = find_naked_single(board, candidates) {
            place_value(
                board,
                state,
                candidates,
                trail,
                row,
                col,
                value,
            );

            record_forced_move(&mut stats);
            continue;
        }

        if config.use_hidden_singles {
            if let Some((row, col, value)) =
                find_hidden_single(board, candidates)
            {
                place_value(
                    board,
                    state,
                    candidates,
                    trail,
                    row,
                    col,
                    value,
                );

                record_forced_move(&mut stats);
                record_hidden_single(&mut stats);
                continue;
            }
        }

        break;
    }

    let Some(((row, col), mut remaining)) =
        find_best_empty(board, candidates)
    else {
        // No empty cells remain, so the board is complete.
        return true;
    };

    // An empty candidate set is a contradiction.
    if remaining == 0 {
        trail.undo_to(checkpoint, board, state, candidates);
        return false;
    }

    record_branch(&mut stats);

    while remaining != 0 {
        let value = next_value(&mut remaining);
        let branch_checkpoint = trail.checkpoint();

        place_value(
            board,
            state,
            candidates,
            trail,
            row,
            col,
            value,
        );

        if solve_recursive(
            board,
            state,
            candidates,
            trail,
            config,
            stats.as_deref_mut(),
        ) {
            return true;
        }

        trail.undo_to(branch_checkpoint, board, state, candidates);
        record_backtrack(&mut stats);
    }

    trail.undo_to(checkpoint, board, state, candidates);
    false
}

pub fn profile(board: &mut Board) -> SolverProfile {
    profile_with_config(board, SolverConfig::default())
}

pub fn profile_with_config(
    board: &mut Board,
    config: SolverConfig,
) -> SolverProfile {
    let clues = count_clues(board);

    let stats = solve_with_stats_config(board, config);

    SolverProfile {
        clues,
        recursive_calls: stats.recursive_calls,
        forced_moves: stats.forced_moves,
        hidden_singles: stats.hidden_singles,
        branches: stats.branches,
        backtracks: stats.backtracks,
    }
}

fn board_is_complete(board: &Board) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if board.is_empty(row, col) {
                return false;
            }
        }
    }

    true
}

fn build_candidate_cache(
    board: &Board,
    state: &SolverState,
) -> CandidateCache {
    let mut candidates = [[0u16; 9]; 9];

    for row in 0..9 {
        for col in 0..9 {
            if board.is_empty(row, col) {
                candidates[row][col] = state.candidate_mask(row, col);
            }
        }
    }

    candidates
}

fn place_value(
    board: &mut Board,
    state: &mut SolverState,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
    row: usize,
    col: usize,
    value: u8,
) {
    trail.push(Undo::BoardCell { row, col });
    board.set(row, col, value);

    let bit = digit_bit(value);
    let box_index = box_index(row, col);

    trail.push(Undo::RowMask {
        row,
        previous: state.row_masks[row],
    });

    trail.push(Undo::ColMask {
        col,
        previous: state.col_masks[col],
    });

    trail.push(Undo::BoxMask {
        index: box_index,
        previous: state.box_masks[box_index],
    });

    state.place(row, col, value);

    // The placed cell is no longer empty.
    let previous = candidates[row][col];

    if previous != 0 {
        trail.push(Undo::Candidate {
            row,
            col,
            previous,
        });
    }

    candidates[row][col] = 0;

    // Remove the value from all peers.
    remove_candidate_from_row(
        candidates,
        board,
        trail,
        row,
        col,
        bit,
    );

    remove_candidate_from_column(
        candidates,
        board,
        trail,
        row,
        col,
        bit,
    );

    remove_candidate_from_box(
        candidates,
        board,
        trail,
        row,
        col,
        bit,
    );
}

fn remove_candidate_from_row(
    candidates: &mut CandidateCache,
    board: &Board,
    trail: &mut Trail,
    row: usize,
    col: usize,
    bit: u16,
) {
    for peer_col in 0..9 {
        if peer_col == col || !board.is_empty(row, peer_col) {
            continue;
        }

        remove_candidate(candidates, trail, row, peer_col, bit);
    }
}

fn remove_candidate_from_column(
    candidates: &mut CandidateCache,
    board: &Board,
    trail: &mut Trail,
    row: usize,
    col: usize,
    bit: u16,
) {
    for peer_row in 0..9 {
        if peer_row == row || !board.is_empty(peer_row, col) {
            continue;
        }

        remove_candidate(candidates, trail, peer_row, col, bit);
    }
}

fn remove_candidate_from_box(
    candidates: &mut CandidateCache,
    board: &Board,
    trail: &mut Trail,
    row: usize,
    col: usize,
    bit: u16,
) {
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;

    for peer_row in start_row..start_row + 3 {
        for peer_col in start_col..start_col + 3 {
            if (peer_row == row && peer_col == col)
                || !board.is_empty(peer_row, peer_col)
            {
                continue;
            }

            remove_candidate(
                candidates,
                trail,
                peer_row,
                peer_col,
                bit,
            );
        }
    }
}

#[inline]
fn remove_candidate(
    candidates: &mut CandidateCache,
    trail: &mut Trail,
    row: usize,
    col: usize,
    bit: u16,
) {
    let previous = candidates[row][col];
    let next = previous & !bit;

    if previous == next {
        return;
    }

    trail.push(Undo::Candidate {
        row,
        col,
        previous,
    });

    candidates[row][col] = next;
}

fn find_naked_single(
    board: &Board,
    candidates: &CandidateCache,
) -> Option<(usize, usize, u8)> {
    for row in 0..9 {
        for col in 0..9 {
            if !board.is_empty(row, col) {
                continue;
            }

            let mask = candidates[row][col];

            if mask.count_ones() == 1 {
                return Some((row, col, mask_to_value(mask)));
            }
        }
    }

    None
}

fn find_hidden_single(
    board: &Board,
    candidates: &CandidateCache,
) -> Option<(usize, usize, u8)> {
    for row in 0..9 {
        if let Some(result) =
            find_hidden_single_in_row(board, candidates, row)
        {
            return Some(result);
        }
    }

    for col in 0..9 {
        if let Some(result) =
            find_hidden_single_in_column(board, candidates, col)
        {
            return Some(result);
        }
    }

    for box_row in 0..3 {
        for box_col in 0..3 {
            if let Some(result) =
                find_hidden_single_in_box(
                    board,
                    candidates,
                    box_row,
                    box_col,
                )
            {
                return Some(result);
            }
        }
    }

    None
}

fn find_hidden_single_in_row(
    board: &Board,
    candidates: &CandidateCache,
    row: usize,
) -> Option<(usize, usize, u8)> {
    let mut locations = [None; 9];
    let mut counts = [0u8; 9];

    for col in 0..9 {
        if board.is_empty(row, col) {
            record_candidate_locations(
                candidates[row][col],
                col,
                &mut locations,
                &mut counts,
            );
        }
    }

    find_location(locations, counts, |col| (row, col))
}

fn find_hidden_single_in_column(
    board: &Board,
    candidates: &CandidateCache,
    col: usize,
) -> Option<(usize, usize, u8)> {
    let mut locations = [None; 9];
    let mut counts = [0u8; 9];

    for row in 0..9 {
        if board.is_empty(row, col) {
            record_candidate_locations(
                candidates[row][col],
                row,
                &mut locations,
                &mut counts,
            );
        }
    }

    find_location(locations, counts, |row| (row, col))
}

fn find_hidden_single_in_box(
    board: &Board,
    candidates: &CandidateCache,
    box_row: usize,
    box_col: usize,
) -> Option<(usize, usize, u8)> {
    let mut locations = [None; 9];
    let mut counts = [0u8; 9];

    let start_row = box_row * 3;
    let start_col = box_col * 3;

    for row in start_row..start_row + 3 {
        for col in start_col..start_col + 3 {
            if board.is_empty(row, col) {
                record_candidate_locations(
                    candidates[row][col],
                    row * 9 + col,
                    &mut locations,
                    &mut counts,
                );
            }
        }
    }

    find_location(locations, counts, |location| {
        (location / 9, location % 9)
    })
}

fn record_candidate_locations(
    mask: u16,
    location: usize,
    locations: &mut [Option<usize>; 9],
    counts: &mut [u8; 9],
) {
    for digit in 0..9 {
        if mask & (1 << digit) == 0 {
            continue;
        }

        counts[digit] += 1;

        if counts[digit] == 1 {
            locations[digit] = Some(location);
        }
    }
}

fn find_location(
    locations: [Option<usize>; 9],
    counts: [u8; 9],
    make_location: impl Fn(usize) -> (usize, usize),
) -> Option<(usize, usize, u8)> {
    for digit in 0..9 {
        if counts[digit] == 1 {
            let location = locations[digit].unwrap();
            let (row, col) = make_location(location);

            return Some((row, col, digit as u8 + 1));
        }
    }

    None
}

fn find_best_empty(
    board: &Board,
    candidates: &CandidateCache,
) -> Option<((usize, usize), u16)> {
    let mut best_cell = None;
    let mut best_mask = 0;
    let mut fewest_candidates = 10;

    for row in 0..9 {
        for col in 0..9 {
            if !board.is_empty(row, col) {
                continue;
            }

            let mask = candidates[row][col];
            let count = mask.count_ones();

            if count == 0 {
                return Some(((row, col), 0));
            }

            if count < fewest_candidates {
                fewest_candidates = count;
                best_cell = Some((row, col));
                best_mask = mask;

                if count == 1 {
                    break;
                }
            }
        }
    }

    best_cell.map(|cell| (cell, best_mask))
}

#[inline]
fn next_value(mask: &mut u16) -> u8 {
    let bit = mask.trailing_zeros();

    *mask &= *mask - 1;

    bit as u8 + 1
}

#[inline]
fn mask_to_value(mask: u16) -> u8 {
    mask.trailing_zeros() as u8 + 1
}

#[inline]
fn digit_bit(value: u8) -> u16 {
    1 << (value - 1)
}

#[inline]
fn box_index(row: usize, col: usize) -> usize {
    (row / 3) * 3 + (col / 3)
}

#[inline]
fn record_recursive_call(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.recursive_calls += 1;
    }
}

#[inline]
fn record_forced_move(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.forced_moves += 1;
    }
}

#[inline]
fn record_hidden_single(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.hidden_singles += 1;
    }
}

#[inline]
fn record_branch(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.branches += 1;
    }
}

#[inline]
fn record_backtrack(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.backtracks += 1;
    }
}