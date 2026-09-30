use rand::{rng, seq::SliceRandom};

use crate::{
    board::Board,
    solver::{count_solutions, profile, Difficulty},
};

const EASY_MAX_ATTEMPTS: usize = 200;
const MEDIUM_MAX_ATTEMPTS: usize = 20;
const HARD_MAX_ATTEMPTS: usize = 20;

pub fn generate() -> Board {
    let mut board = Board::random();
    let mut rng = rng();

    let mut cells: Vec<usize> = (0..81).collect();
    cells.shuffle(&mut rng);

    for index in cells {
        let row = index / 9;
        let col = index % 9;

        let value = board.get(row, col);

        board.clear(row, col);

        let solutions = count_solutions(&mut board, 2);

        if solutions != 1 {
            board.set(row, col, value);
        }
    }

    board
}

pub fn generate_with_difficulty(difficulty: Difficulty) -> Option<Board> {
    let max_attempts = match difficulty {
        Difficulty::Easy => EASY_MAX_ATTEMPTS,
        Difficulty::Medium => MEDIUM_MAX_ATTEMPTS,
        Difficulty::Hard => HARD_MAX_ATTEMPTS,
    };

    for _ in 0..max_attempts {
        let puzzle = generate();

        let mut board = puzzle.clone();
        let stats = profile(&mut board);

        if stats.difficulty() == difficulty {
            return Some(puzzle);
        }
    }

    None
}