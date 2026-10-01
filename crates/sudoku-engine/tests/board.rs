mod tests {
    use rand::{RngExt, rng};
    use sudoku_engine::board::Board;

    #[test]
    fn empty_board() {
        let mut board = Board::empty();

        board.set(5, 5, 7);

        assert_eq!(board.get(5, 5), 7);
    }

    #[test]
    fn base_board() {
        let mut rng = rng();

        let number = rng.random_range(1..=9);

        println!("Random number: {}", number);

        let board = Board::base();

        assert_eq!(board.get(0, 0), 1);
        assert_eq!(board.get(0, 8), 9);
        assert_eq!(board.get(1, 0), 4);
        assert_eq!(board.get(1, 8), 3);
        assert_eq!(board.get(8, 0), 9);
        assert_eq!(board.get(8, 8), 8);
    }

    #[test]
    fn swap_digits() {
        let mut board = Board::base();

        board.swap_digits(1, 5);

        assert_eq!(board.get(0, 0), 5);
        assert_eq!(board.get(0, 4), 1);
    }

    #[test]
    fn swap_rows() {
        let mut board = Board::base();

        board.swap_rows(0, 2);

        assert_eq!(board.get(0, 0), 7);
        assert_eq!(board.get(2, 0), 1);
    }

    #[test]
    fn swap_columns() {
        let mut board = Board::base();

        board.swap_columns(0, 2);

        assert_eq!(board.get(0, 0), 3);
        assert_eq!(board.get(0, 2), 1);
    }

    #[test]
    fn swap_bands() {
        let mut board = Board::base();

        board.swap_bands(0, 2);

        assert_eq!(board.get(0, 0), 3);
        assert_eq!(board.get(2, 0), 9);

        assert_eq!(board.get(6, 0), 1);
        assert_eq!(board.get(8, 0), 7);
    }

    #[test]
    fn swap_stacks() {
        let mut board = Board::base();

        board.swap_stacks(0, 2);

        assert_eq!(board.get(0, 0), 7);
        assert_eq!(board.get(0, 2), 9);

        assert_eq!(board.get(0, 6), 1);
        assert_eq!(board.get(0, 8), 3);
    }

    #[test]
    fn random_swap_digits() {
        let mut board = Board::base();
        let mut rng = rng();

        println!("Before:\n{}", board);

        board.random_swap_digits(&mut rng);

        println!("After:\n{}", board);
    }

    #[test]
    fn random_swap_rows() {
        let mut board = Board::base();
        let mut rng = rng();

        println!("Before:\n{}", board);

        board.random_swap_rows(&mut rng);

        println!("After:\n{}", board);
    }

    #[test]
    fn random_swap_bands() {
        let mut board = Board::base();
        let mut rng = rng();

        println!("Before:\n{}", board);

        board.random_swap_bands(&mut rng);

        println!("After:\n{}", board);
    }

    #[test]
    fn random_swap_columns() {
        let mut board = Board::base();
        let mut rng = rng();

        println!("Before:\n{}", board);

        board.random_swap_columns(&mut rng);

        println!("After:\n{}", board);
    }

    #[test]
    fn random_swap_stacks() {
        let mut board = Board::base();
        let mut rng = rng();

        println!("Before:\n{}", board);

        board.random_swap_stacks(&mut rng);

        println!("After:\n{}", board);
    }

    #[test]
    fn random_board() {
        let board = Board::random();

        println!("{}", board);
    }

    #[test]
    fn clear_cell() {
        let mut board = Board::base();

        board.clear(4, 7);

        assert_eq!(board.get(4, 7), 0);
    }

    #[test]
    fn cell_is_empty() {
        let mut board = Board::base();

        assert!(!board.is_empty(4, 7));

        board.clear(4, 7);

        assert!(board.is_empty(4, 7));
    }

    #[test]
    fn can_place_checks_row() {
        let board = Board::base();

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
}
