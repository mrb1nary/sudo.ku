use sudoku_engine::board::Board;
use sudoku_engine::solver::solve;

#[test]
fn solve_complete_board() {
    let mut board = Board::base();

    assert!(solve(&mut board));
}

#[test]
fn solve_places_a_value() {
    let mut board = Board::base();

    board.clear(4, 7);

    assert!(solve(&mut board));

    assert!(!board.is_empty(4, 7));
}

#[test]
fn solve_puzzle() {
    let mut board = Board::base();

    board.clear(0, 0);
    board.clear(1, 1);
    board.clear(2, 2);
    board.clear(3, 3);
    board.clear(4, 4);
    board.clear(5, 5);
    board.clear(6, 6);
    board.clear(7, 7);
    board.clear(8, 8);

    assert!(solve(&mut board));

    assert_eq!(board.get(0, 0), 1);
    assert_eq!(board.get(1, 1), 5);
    assert_eq!(board.get(2, 2), 9);
    assert_eq!(board.get(3, 3), 5);
    assert_eq!(board.get(4, 4), 9);
    assert_eq!(board.get(5, 5), 4);
    assert_eq!(board.get(6, 6), 9);
    assert_eq!(board.get(7, 7), 4);
    assert_eq!(board.get(8, 8), 8);
}