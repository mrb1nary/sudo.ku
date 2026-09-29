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

    fn candidate_mask(&self, row: usize, col: usize) -> u16 {
        let box_index = (row / 3) * 3 + (col / 3);

        let used = self.row_masks[row]
            | self.col_masks[col]
            | self.box_masks[box_index];

        ALL_DIGITS & !used
    }

    fn place(&mut self, row: usize, col: usize, value: u8) {
        let bit = 1 << (value - 1);
        let box_index = (row / 3) * 3 + (col / 3);

        self.row_masks[row] |= bit;
        self.col_masks[col] |= bit;
        self.box_masks[box_index] |= bit;
    }

    fn remove(&mut self, row: usize, col: usize, value: u8) {
        let bit = 1 << (value - 1);
        let box_index = (row / 3) * 3 + (col / 3);

        self.row_masks[row] &= !bit;
        self.col_masks[col] &= !bit;
        self.box_masks[box_index] &= !bit;
    }
}

type CandidateCache = [[u16; 9]; 9];

pub fn solve(board: &mut Board) -> bool {
    let mut state = SolverState::new(board);

    solve_recursive(board, &mut state, None)
}

pub fn solve_with_stats(board: &mut Board) -> SolverStats {
    let mut state = SolverState::new(board);
    let mut stats = SolverStats::default();

    solve_recursive(board, &mut state, Some(&mut stats));

    stats
}

fn solve_recursive(
    board: &mut Board,
    state: &mut SolverState,
    mut stats: Option<&mut SolverStats>,
) -> bool {
    if let Some(stats) = stats.as_deref_mut() {
        stats.recursive_calls += 1;
    }

    loop {
        let candidates = build_candidate_cache(board, state);

        if let Some((row, col, value)) =
            find_naked_single(board, &candidates)
        {
            board.set(row, col, value);
            state.place(row, col, value);

            if let Some(stats) = stats.as_deref_mut() {
                stats.forced_moves += 1;
            }

            continue;
        }

        if let Some((row, col, value)) =
            find_hidden_single(board, &candidates)
        {
            board.set(row, col, value);
            state.place(row, col, value);

            if let Some(stats) = stats.as_deref_mut() {
                stats.forced_moves += 1;
                stats.hidden_singles += 1;
            }

            continue;
        }

        break;
    }

    let candidates = build_candidate_cache(board, state);

    let Some(((row, col), mut mask)) =
        find_best_empty(board, &candidates)
    else {
        return true;
    };

    while mask != 0 {
        let bit = mask.trailing_zeros();
        let value = (bit + 1) as u8;

        mask &= mask - 1;

        if let Some(stats) = stats.as_deref_mut() {
            stats.branches += 1;
        }

        board.set(row, col, value);
        state.place(row, col, value);

        if solve_recursive(board, state, stats.as_deref_mut()) {
            return true;
        }

        state.remove(row, col, value);
        board.clear(row, col);

        if let Some(stats) = stats.as_deref_mut() {
            stats.backtracks += 1;
        }
    }

    false
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
                let value =
                    (mask.trailing_zeros() + 1) as u8;

                return Some((row, col, value));
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
        if !board.is_empty(row, col) {
            continue;
        }

        let mask = candidates[row][col];

        for digit in 0..9 {
            if mask & (1 << digit) != 0 {
                if locations[digit].is_some() {
                    locations[digit] = None;
                } else {
                    locations[digit] = Some(col);
                }
            }
        }
    }

    for digit in 0..9 {
        if let Some(col) = locations[digit] {
            return Some((row, col, digit as u8 + 1));
        }
    }

    None
}

fn find_hidden_single_in_column(
    board: &Board,
    candidates: &CandidateCache,
    col: usize,
) -> Option<(usize, usize, u8)> {
    let mut locations = [None; 9];

    for row in 0..9 {
        if !board.is_empty(row, col) {
            continue;
        }

        let mask = candidates[row][col];

        for digit in 0..9 {
            if mask & (1 << digit) != 0 {
                if locations[digit].is_some() {
                    locations[digit] = None;
                } else {
                    locations[digit] = Some(row);
                }
            }
        }
    }

    for digit in 0..9 {
        if let Some(row) = locations[digit] {
            return Some((row, col, digit as u8 + 1));
        }
    }

    None
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
            if !board.is_empty(row, col) {
                continue;
            }

            let mask = candidates[row][col];

            for digit in 0..9 {
                if mask & (1 << digit) != 0 {
                    if locations[digit].is_some() {
                        locations[digit] = None;
                    } else {
                        locations[digit] = Some((row, col));
                    }
                }
            }
        }
    }

    for digit in 0..9 {
        if let Some((row, col)) = locations[digit] {
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