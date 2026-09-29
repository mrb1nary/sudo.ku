#[cfg(test)]
mod tests {
    use super::*;
    use sudoku_engine::board::Board;

    #[test]
    fn empty_board() {
        let mut board = Board::empty();

        assert!(board.find_empty().is_some());
        assert!(board.is_valid());
        assert!(!board.is_solved());

        board.set(5, 5, 7);

        assert_eq!(board.get(5, 5), 7);
    }

    #[test]
    fn solved_board() {
        let board = Board::solved();

        assert_eq!(board.get(0, 0), 1);
        assert_eq!(board.get(0, 8), 9);
        assert_eq!(board.get(1, 0), 4);
        assert_eq!(board.get(1, 8), 3);
        assert_eq!(board.get(8, 0), 9);
        assert_eq!(board.get(8, 8), 8);

        assert!(board.is_valid());
        assert!(board.is_solved());
        assert_eq!(board.find_empty(), None);
    }

    #[test]
    fn random_solved_board_is_valid_solution() {
        for _ in 0..100 {
            let board = Board::random_solved();

            assert!(board.is_valid());
            assert!(board.is_solved());
            assert_eq!(board.find_empty(), None);
        }
    }

    #[test]
    fn clear_cell() {
        let mut board = Board::solved();

        board.clear(4, 7);

        assert_eq!(board.get(4, 7), 0);
        assert!(board.is_empty(4, 7));
        assert!(board.is_valid());
        assert!(!board.is_solved());
    }

    #[test]
    fn cell_is_empty() {
        let mut board = Board::solved();

        assert!(!board.is_empty(4, 7));

        board.clear(4, 7);

        assert!(board.is_empty(4, 7));
    }

    #[test]
    fn can_place_checks_row() {
        let board = Board::solved();

        assert!(!board.can_place(0, 4, 5));
        assert!(!board.can_place(0, 4, 1));
    }

    #[test]
    fn can_place_checks_column() {
        let mut board = Board::empty();

        board.set(0, 4, 5);

        assert!(!board.can_place(4, 4, 5));
        assert!(board.can_place(4, 4, 7));
    }

    #[test]
    fn can_place_checks_box() {
        let mut board = Board::empty();

        board.set(0, 0, 5);

        assert!(!board.can_place(2, 2, 5));
        assert!(board.can_place(2, 2, 7));
    }

    #[test]
    fn can_place_rejects_invalid_values() {
        let board = Board::empty();

        assert!(!board.can_place(0, 0, 0));
        assert!(!board.can_place(0, 0, 10));
        assert!(!board.can_place(0, 0, 255));
    }

    #[test]
    fn can_place_rejects_occupied_cell() {
        let mut board = Board::empty();

        board.set(0, 0, 5);

        assert!(!board.can_place(0, 0, 5));
        assert!(!board.can_place(0, 0, 7));
    }

    #[test]
    fn can_place_allows_legal_value() {
        let mut board = Board::empty();

        board.set(0, 0, 5);

        assert!(board.can_place(0, 1, 7));
    }

    #[test]
    fn valid_partial_board() {
        let mut board = Board::empty();

        board.set(0, 0, 5);
        board.set(1, 1, 3);
        board.set(4, 4, 7);
        board.set(8, 8, 9);

        assert!(board.is_valid());
        assert!(!board.is_solved());
    }

    #[test]
    fn invalid_row() {
        let mut board = Board::empty();

        board.set(0, 0, 5);
        board.set(0, 1, 5);

        assert!(!board.is_valid());
        assert!(!board.is_solved());
    }

    #[test]
    fn invalid_column() {
        let mut board = Board::empty();

        board.set(0, 0, 5);
        board.set(1, 0, 5);

        assert!(!board.is_valid());
        assert!(!board.is_solved());
    }

    #[test]
    fn invalid_box() {
        let mut board = Board::empty();

        board.set(0, 0, 5);
        board.set(1, 1, 5);

        assert!(!board.is_valid());
        assert!(!board.is_solved());
    }

    #[test]
    fn invalid_value_makes_board_invalid() {
        let mut board = Board::empty();

        board.set(0, 0, 10);

        assert!(!board.is_valid());
        assert!(!board.is_solved());
    }

    #[test]
    fn full_invalid_board_is_not_solved() {
        let mut board = Board::solved();

        board.set(0, 0, 2);

        assert_eq!(board.find_empty(), None);
        assert!(!board.is_valid());
        assert!(!board.is_solved());
    }

    #[test]
    fn find_empty_cell() {
        let mut board = Board::solved();

        assert_eq!(board.find_empty(), None);

        board.clear(4, 7);

        assert_eq!(board.find_empty(), Some((4, 7)));
    }

    #[test]
    fn find_empty_returns_first_empty_cell() {
        let mut board = Board::solved();

        board.clear(6, 6);
        board.clear(2, 3);
        board.clear(0, 8);

        assert_eq!(board.find_empty(), Some((0, 8)));
    }
}
