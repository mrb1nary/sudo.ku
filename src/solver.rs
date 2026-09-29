use crate::board::Board;

pub fn solve(board: &mut Board) -> bool {
    let empty = board.find_empty();

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