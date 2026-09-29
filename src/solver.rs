use crate::board::Board;

const ALL_DIGITS: u16 = 0b1_1111_1111;

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

                if value == 0 {
                    continue;
                }

                state.place(row, col, value);
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

pub fn solve(board: &mut Board) -> bool {
    let mut state = SolverState::new(board);

    solve_recursive(board, &mut state)
}

fn solve_recursive(board: &mut Board, state: &mut SolverState) -> bool {
    // Propagate forced moves before making a branching decision.
    loop {
        let mut forced_move = None;

        'search: for row in 0..9 {
            for col in 0..9 {
                if !board.is_empty(row, col) {
                    continue;
                }

                let mask = state.candidate_mask(row, col);
                let count = mask.count_ones();

                // No legal value means this branch is impossible.
                if count == 0 {
                    return false;
                }

                // Exactly one possible value.
                if count == 1 {
                    let value = (mask.trailing_zeros() + 1) as u8;
                    forced_move = Some((row, col, value));
                    break 'search;
                }
            }
        }

        let Some((row, col, value)) = forced_move else {
            break;
        };

        board.set(row, col, value);
        state.place(row, col, value);
    }

    // No forced moves remain. Use MRV to choose a branching cell.
    let Some(((row, col), mut mask)) = find_best_empty(board, state) else {
        return true;
    };

    while mask != 0 {
        let bit = mask.trailing_zeros();
        let value = (bit + 1) as u8;

        // Remove the candidate we're about to try.
        mask &= mask - 1;

        board.set(row, col, value);
        state.place(row, col, value);

        if solve_recursive(board, state) {
            return true;
        }

        state.remove(row, col, value);
        board.clear(row, col);
    }

    false
}

fn find_best_empty(
    board: &Board,
    state: &SolverState,
) -> Option<((usize, usize), u16)> {
    let mut best_cell = None;
    let mut best_mask = 0;
    let mut fewest_candidates = 10;

    for row in 0..9 {
        for col in 0..9 {
            if !board.is_empty(row, col) {
                continue;
            }

            let mask = state.candidate_mask(row, col);
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