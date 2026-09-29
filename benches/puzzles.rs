use sudoku_engine::board::Board;

pub fn puzzle_from_rows(rows: [&str; 9]) -> Board {
    let mut board = Board::empty();

    for (row, line) in rows.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                let value = ch.to_digit(10).unwrap() as u8;
                board.set(row, col, value);
            }
        }
    }

    board
}

pub fn benchmark_puzzles() -> Vec<Board> {
    vec![
        puzzle_from_rows([
            "8.57..3.4",
            "7.2.3.51.",
            "4.9.5.6.7",
            "1.34.57.9",
            "9.671.85.",
            "25.976.31",
            "5.416.9.3",
            "37.51.286",
            "6.23.7145",
        ]),

        puzzle_from_rows([
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]),

        // Add the remaining puzzles here...
    ]
}