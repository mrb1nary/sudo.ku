use sudoku_engine::board::Board;
use sudoku_engine::solver::{
    solve,
    solve_with_stats,
    solve_with_stats_config,
    SolverConfig,
};

fn board_from_string(puzzle: &str) -> Board {
    let puzzle: String = puzzle
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    assert_eq!(puzzle.len(), 81, "puzzle must contain exactly 81 cells");

    let mut board = Board::empty();

    for (index, byte) in puzzle.bytes().enumerate() {
        assert!(
            byte.is_ascii_digit(),
            "invalid puzzle character at index {index}: {:?}",
            byte as char
        );

        let value = byte - b'0';

        if value != 0 {
            assert!(
                (1..=9).contains(&value),
                "invalid Sudoku value at index {index}: {value}"
            );

            board.set(index / 9, index % 9, value);
        }
    }

    board
}

fn assert_solved(board: &Board) {
    assert!(board.is_solved(), "board should be a solved Sudoku");

    for row in 0..9 {
        for col in 0..9 {
            assert!(
                !board.is_empty(row, col),
                "empty cell at r{}c{}",
                row + 1,
                col + 1
            );
        }
    }
}

fn inspect_stats(name: &str, puzzle: &str) {
    let mut board = board_from_string(puzzle);

    let stats = solve_with_stats(&mut board);

    println!("\n{name}");
    println!("{stats:#?}");

    assert_solved(&board);
}

fn assert_valid_solution(board: &Board) {
    // Every cell must contain a digit.
    for row in 0..9 {
        for col in 0..9 {
            let value = board.get(row, col);

            assert!(
                (1..=9).contains(&value),
                "empty/invalid value at r{}c{}: {}",
                row + 1,
                col + 1,
                value
            );
        }
    }

    // Every row must contain 1..=9 exactly once.
    for row in 0..9 {
        let mut seen = [false; 10];

        for col in 0..9 {
            let value = board.get(row, col) as usize;

            assert!(
                !seen[value],
                "duplicate {} in row {}",
                value,
                row + 1
            );

            seen[value] = true;
        }

        assert!(seen[1..].iter().all(|&present| present));
    }

    // Every column must contain 1..=9 exactly once.
    for col in 0..9 {
        let mut seen = [false; 10];

        for row in 0..9 {
            let value = board.get(row, col) as usize;

            assert!(
                !seen[value],
                "duplicate {} in column {}",
                value,
                col + 1
            );

            seen[value] = true;
        }

        assert!(seen[1..].iter().all(|&present| present));
    }

    // Every 3x3 box must contain 1..=9 exactly once.
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut seen = [false; 10];

            for row in box_row * 3..box_row * 3 + 3 {
                for col in box_col * 3..box_col * 3 + 3 {
                    let value = board.get(row, col) as usize;

                    assert!(
                        !seen[value],
                        "duplicate {} in box ({}, {})",
                        value,
                        box_row,
                        box_col
                    );

                    seen[value] = true;
                }
            }

            assert!(seen[1..].iter().all(|&present| present));
        }
    }
}

#[test]
fn inspect_solver_stats() {
    inspect_stats(
        "easy",
        "\
        530070000\
        600195000\
        098000060\
        800060003\
        400803001\
        700020006\
        060000280\
        000419005\
        000080079",
    );

    inspect_stats(
        "medium",
        "\
        003020600\
        900305001\
        001806400\
        008102900\
        700000008\
        006708200\
        002609500\
        800203009\
        005010300",
    );

    inspect_stats(
        "hard",
        "\
        005300000\
        800000020\
        070010500\
        400005300\
        010070006\
        003200080\
        060500009\
        004000030\
        000009700",
    );
}

#[test]
fn incremental_cache_survives_backtracking() {
    let mut board = board_from_string(
        "\
        005300000\
        800000020\
        070010500\
        400005300\
        010070006\
        003200080\
        060500009\
        004000030\
        000009700",
    );

    let stats = solve_with_stats_config(
        &mut board,
        SolverConfig {
            use_hidden_singles: false,
        },
    );

    println!("{stats:#?}");

    assert!(stats.branches > 0);
    assert!(stats.backtracks > 0);

    assert_solved(&board);
    assert_valid_solution(&board);
}

#[test]
fn solve_returns_true_for_solvable_puzzle() {
    let mut board = board_from_string(
        "\
        530070000\
        600195000\
        098000060\
        800060003\
        400803001\
        700020006\
        060000280\
        000419005\
        000080079",
    );

    assert!(solve(&mut board));
    assert_solved(&board);
}

#[test]
fn solve_rejects_invalid_board() {
    let mut board = Board::empty();

    board.set(0, 0, 5);
    board.set(0, 1, 5);

    assert!(!solve(&mut board));
    assert!(!board.is_solved());
}

#[test]
fn solve_leaves_unsolvable_board_unsolved() {
    let mut board = board_from_string(
        "\
        530570000\
        600195000\
        098000060\
        800060003\
        400803001\
        700020006\
        060000280\
        000419005\
        000080079",
    );

    assert!(!solve(&mut board));
    assert!(!board.is_solved());
}
