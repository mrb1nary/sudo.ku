use crate::board::Board;

const ALL_DIGITS: u16 = 0b1_1111_1111;

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

type CandidateCache = [[u16; 9]; 9];

#[derive(Clone)]
struct SolverState {
    row_masks: [u16; 9],
    col_masks: [u16; 9],
    box_masks: [u16; 9],
}

impl SolverState {
    fn new(board: &Board) -> Self {
        let mut state = Self {
            row_masks: [0; 9],
            col_masks: [0; 9],
            box_masks: [0; 9],
        };

        for row in 0..9 {
            for col in 0..9 {
                let value = board.get(row, col);

                if value != 0 {
                    state.place(row, col, value);
                }
            }
        }

        state
    }

    fn place(&mut self, row: usize, col: usize, value: u8) {
        let bit = digit_bit(value);
        let box_index = box_index(row, col);

        self.row_masks[row] |= bit;
        self.col_masks[col] |= bit;
        self.box_masks[box_index] |= bit;
    }

    fn remove(&mut self, row: usize, col: usize, value: u8) {
        let bit = digit_bit(value);
        let box_index = box_index(row, col);

        self.row_masks[row] &= !bit;
        self.col_masks[col] &= !bit;
        self.box_masks[box_index] &= !bit;
    }

    fn candidate_mask(&self, row: usize, col: usize) -> u16 {
        let used = self.row_masks[row]
            | self.col_masks[col]
            | self.box_masks[box_index(row, col)];

        ALL_DIGITS & !used
    }
}

pub fn solve(board: &mut Board) -> bool {
    solve_with_config(board, SolverConfig::default())
}

pub fn solve_with_config(board: &mut Board, config: SolverConfig) -> bool {
    let mut state = SolverState::new(board);

    solve_recursive(board, &mut state, config, None)
}

pub fn solve_with_stats(board: &mut Board) -> SolverStats {
    solve_with_stats_config(board, SolverConfig::default())
}

pub fn solve_with_stats_config(
    board: &mut Board,
    config: SolverConfig,
) -> SolverStats {
    let mut state = SolverState::new(board);
    let mut stats = SolverStats::default();

    solve_recursive(
        board,
        &mut state,
        config,
        Some(&mut stats),
    );

    stats
}

fn solve_recursive(
    board: &mut Board,
    state: &mut SolverState,
    config: SolverConfig,
    mut stats: Option<&mut SolverStats>,
) -> bool {
    record_recursive_call(&mut stats);

    let mut candidates = build_candidate_cache(board, state);

    // Constraint propagation.
    loop {
        if let Some((row, col, value)) =
            find_naked_single(board, &candidates)
        {
            place_value(
                board,
                state,
                &mut candidates,
                row,
                col,
                value,
            );

            record_forced_move(&mut stats);
            continue;
        }

        if config.use_hidden_singles {
            if let Some((row, col, value)) =
                find_hidden_single(board, &candidates)
            {
                place_value(
                    board,
                    state,
                    &mut candidates,
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

    // No empty cells means the puzzle is solved.
    let Some(((row, col), candidate_mask)) =
        find_best_empty(board, &candidates)
    else {
        return board_is_complete(board);
    };

    // No candidates means this branch is impossible.
    if candidate_mask == 0 {
        return false;
    }

    // Try each candidate.
    let mut remaining = candidate_mask;

    while remaining != 0 {
        let value = next_value(&mut remaining);

        record_branch(&mut stats);

        // Snapshot the complete search state before branching.
        let previous_board = board.clone();
        let previous_state = state.clone();
        let previous_candidates = candidates;

        board.set(row, col, value);
        state.place(row, col, value);

        if solve_recursive(
            board,
            state,
            config,
            stats.as_deref_mut(),
        ) {
            return true;
        }

        // Restore the complete state after a failed branch.
        *board = previous_board;
        *state = previous_state;
        candidates = previous_candidates;

        record_backtrack(&mut stats);
    }

    false
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
                candidates[row][col] =
                    state.candidate_mask(row, col);
            }
        }
    }

    candidates
}

fn place_value(
    board: &mut Board,
    state: &mut SolverState,
    candidates: &mut CandidateCache,
    row: usize,
    col: usize,
    value: u8,
) {
    board.set(row, col, value);
    state.place(row, col, value);

    let bit = digit_bit(value);

    // Remove the value from the row.
    for peer_col in 0..9 {
        if peer_col != col && board.is_empty(row, peer_col) {
            candidates[row][peer_col] &= !bit;
        }
    }

    // Remove the value from the column.
    for peer_row in 0..9 {
        if peer_row != row && board.is_empty(peer_row, col) {
            candidates[peer_row][col] &= !bit;
        }
    }

    // Remove the value from the box.
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;

    for peer_row in start_row..start_row + 3 {
        for peer_col in start_col..start_col + 3 {
            if (peer_row != row || peer_col != col)
                && board.is_empty(peer_row, peer_col)
            {
                candidates[peer_row][peer_col] &= !bit;
            }
        }
    }

    candidates[row][col] = 0;
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
                return Some((
                    row,
                    col,
                    mask_to_value(mask),
                ));
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

    for col in 0..9 {
        if board.is_empty(row, col) {
            record_candidate_locations(
                candidates[row][col],
                col,
                &mut locations,
            );
        }
    }

    find_location(locations, |col| (row, col))
}

fn find_hidden_single_in_column(
    board: &Board,
    candidates: &CandidateCache,
    col: usize,
) -> Option<(usize, usize, u8)> {
    let mut locations = [None; 9];

    for row in 0..9 {
        if board.is_empty(row, col) {
            record_candidate_locations(
                candidates[row][col],
                row,
                &mut locations,
            );
        }
    }

    find_location(locations, |row| (row, col))
}

fn find_hidden_single_in_box(
    board: &Board,
    candidates: &CandidateCache,
    box_row: usize,
    box_col: usize,
) -> Option<(usize, usize, u8)> {
    let mut locations = [None; 9];

    let start_row = box_row * 3;
    let start_col = box_col * 3;

    for row in start_row..start_row + 3 {
        for col in start_col..start_col + 3 {
            if board.is_empty(row, col) {
                record_candidate_locations(
                    candidates[row][col],
                    row * 9 + col,
                    &mut locations,
                );
            }
        }
    }

    for digit in 0..9 {
        if let Some(location) = locations[digit] {
            return Some((
                location / 9,
                location % 9,
                digit as u8 + 1,
            ));
        }
    }

    None
}

fn record_candidate_locations(
    mask: u16,
    location: usize,
    locations: &mut [Option<usize>; 9],
) {
    for digit in 0..9 {
        if mask & (1 << digit) == 0 {
            continue;
        }

        locations[digit] = match locations[digit] {
            None => Some(location),
            Some(_) => None,
        };
    }
}

fn find_location(
    locations: [Option<usize>; 9],
    make_location: impl Fn(usize) -> (usize, usize),
) -> Option<(usize, usize, u8)> {
    for digit in 0..9 {
        if let Some(location) = locations[digit] {
            let (row, col) = make_location(location);

            return Some((
                row,
                col,
                digit as u8 + 1,
            ));
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

fn next_value(mask: &mut u16) -> u8 {
    let bit = mask.trailing_zeros();

    *mask &= *mask - 1;

    bit as u8 + 1
}

fn mask_to_value(mask: u16) -> u8 {
    mask.trailing_zeros() as u8 + 1
}

fn digit_bit(value: u8) -> u16 {
    1 << (value - 1)
}

fn box_index(row: usize, col: usize) -> usize {
    (row / 3) * 3 + (col / 3)
}

fn record_recursive_call(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.recursive_calls += 1;
    }
}

fn record_forced_move(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.forced_moves += 1;
    }
}

fn record_hidden_single(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.hidden_singles += 1;
    }
}

fn record_branch(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.branches += 1;
    }
}

fn record_backtrack(stats: &mut Option<&mut SolverStats>) {
    if let Some(stats) = stats.as_deref_mut() {
        stats.backtracks += 1;
    }
}