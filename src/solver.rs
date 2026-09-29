use crate::board::Board;

pub fn solve(board: &mut Board) -> bool {
    let empty = find_best_empty(board);

    if empty.is_none() {
        return true;
    }

    let (row, col) = empty.unwrap();

    for value in 1..=9 {
        if board.can_place(row, col, value) {
            board.set(row, col, value);

            if solve(board) {
                return true;
            }

            board.clear(row, col);
        }
    }

    false
}

fn find_best_empty(board: &Board) -> Option<(usize, usize)> {
    let mut best_cell = None;
    let mut fewest_candidates = 10;

    for row in 0..9 {
        for col in 0..9 {
            if !board.is_empty(row, col) {
                continue;
            }

            let mut candidates = 0;

            for value in 1..=9 {
                if board.can_place(row, col, value) {
                    candidates += 1;
                }
            }

            if candidates < fewest_candidates {
                fewest_candidates = candidates;
                best_cell = Some((row, col));

                if candidates == 1 {
                    return best_cell;
                }
            }
        }
    }

    best_cell
}