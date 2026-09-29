use sudoku_engine::board::Board;
use sudoku_engine::solver::{solve, solve_with_stats};

fn board_from_string(puzzle: &str) -> Board {
    let puzzle: String = puzzle
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    assert_eq!(puzzle.len(), 81);

    let mut board = Board::empty();

    for (index, byte) in puzzle.bytes().enumerate() {
        let value = byte - b'0';

        if value != 0 {
            board.set(index / 9, index % 9, value);
        }
    }

    board
}

fn inspect_stats(name: &str, puzzle: &str) {
    let mut board = board_from_string(puzzle);

    let stats = solve_with_stats(&mut board);

    println!("\n{name}");
    println!("{stats:#?}");
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