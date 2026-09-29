use crate::board::Board;

pub fn solve(board: &mut Board) -> bool {
    let Some(((row, col), mut mask)) = find_best_empty(board) else {
        return true;
    };

    while mask != 0 {
        let bit = mask.trailing_zeros();
        let value = bit + 1;

        // Remove the lowest set bit.
        mask &= mask - 1;

        board.set(row, col, value as u8);

        if solve(board) {
            return true;
        }

        board.clear(row, col);
    }

    false
}

fn find_best_empty(board: &Board) -> Option<((usize, usize), u16)> {
    let mut best_cell = None;
    let mut best_mask = 0;
    let mut fewest_candidates = 10;

    for row in 0..9 {
        for col in 0..9 {
            if !board.is_empty(row, col) {
                continue;
            }

            let mask = candidate_mask(board, row, col);
            let count = mask.count_ones();

            if count < fewest_candidates {
                fewest_candidates = count;
                best_cell = Some((row, col));
                best_mask = mask;

                if count == 1 {
                    return best_cell.map(|cell| (cell, best_mask));
                }
            }
        }
    }

    best_cell.map(|cell| (cell, best_mask))
}

fn candidate_mask(board: &Board, row: usize, col: usize) -> u16 {
    let mut mask = 0;

    for value in 1..=9 {
        if board.can_place(row, col, value) {
            mask |= 1 << (value - 1);
        }
    }

    mask
}