use rand::{rng, seq::SliceRandom};

pub use crate::{
    board::Board,
    solver::{count_solutions, solve_with_difficulty, Difficulty},
};
use crate::solver::profile;

const BEGINNER_MAX_ATTEMPTS: usize = 200;
const EASY_MAX_ATTEMPTS: usize = 200;
const MEDIUM_MAX_ATTEMPTS: usize = 200;
const HARD_MAX_ATTEMPTS: usize = 1_000;
const EXPERT_MAX_ATTEMPTS: usize = 5_000;

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
        Difficulty::Beginner => BEGINNER_MAX_ATTEMPTS,
        Difficulty::Easy => EASY_MAX_ATTEMPTS,
        Difficulty::Medium => MEDIUM_MAX_ATTEMPTS,
        Difficulty::Hard => HARD_MAX_ATTEMPTS,
        Difficulty::Expert => EXPERT_MAX_ATTEMPTS,
    };

    for _ in 0..max_attempts {
        let puzzle = generate();

        if required_difficulty(&puzzle) == Some(difficulty) {
            return Some(puzzle);
        }
    }

    None
}

fn required_difficulty(board: &Board) -> Option<Difficulty> {
    for difficulty in [
        Difficulty::Beginner,
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Expert,
    ] {
        let mut candidate = board.clone();

        if solve_with_difficulty(&mut candidate, difficulty) {
            return Some(difficulty);
        }
    }

    None
}


#[test]
fn inspect_generated_difficulty_distribution() {
    let mut counts = [0usize; 6];

    for _ in 0..5000 {
        let puzzle = generate();

        match required_difficulty_for_test(&puzzle) {
            Some(Difficulty::Beginner) => counts[0] += 1,
            Some(Difficulty::Easy) => counts[1] += 1,
            Some(Difficulty::Medium) => counts[2] += 1,

            Some(Difficulty::Hard) => {
                counts[3] += 1;

                let mut board = puzzle.clone();
                let stats = profile(&mut board);

                let clues = (0..9)
                    .flat_map(|row| (0..9).map(move |col| (row, col)))
                    .filter(|&(row, col)| puzzle.get(row, col) != 0)
                    .count();

                println!(
                    "\n========== HARD PUZZLE ==========\n\
                     {}\n\
                     clues={}\n\
                     hardest_technique={:?}\n\
                     hidden_singles={}\n\
                     branches={}\n\
                     backtracks={}\n\
                     =================================\n",
                    puzzle,
                    clues,
                    stats.hardest_technique,
                    stats.hidden_singles,
                    stats.branches,
                    stats.backtracks,
                );
            }

            Some(Difficulty::Expert) => counts[4] += 1,
            None => counts[5] += 1,
        }
    }

    println!("\nGenerated difficulty distribution:");
    println!("None:              {}", counts[5]);
    println!("Beginner:          {}", counts[0]);
    println!("Easy:              {}", counts[1]);
    println!("Medium:            {}", counts[2]);
    println!("Hard:              {}", counts[3]);
    println!("Expert:            {}", counts[4]);
}

fn required_difficulty_for_test(
    board: &Board,
) -> Option<Difficulty> {
    for difficulty in [
        Difficulty::Beginner,
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Expert,
    ] {
        let mut candidate = board.clone();

        if solve_with_difficulty(
            &mut candidate,
            difficulty,
        ) {
            return Some(difficulty);
        }
    }

    None
}


#[test]
fn find_hard_puzzle() {
    for attempt in 1..=10_000 {
        let puzzle = generate();

        if required_difficulty_for_test(&puzzle) == Some(Difficulty::Hard) {
            let mut profile_board = puzzle.clone();
            let stats = profile(&mut profile_board);

            let clues = (0..9)
                .flat_map(|row| (0..9).map(move |col| (row, col)))
                .filter(|&(row, col)| puzzle.get(row, col) != 0)
                .count();

            println!(
                "\n========== HARD PUZZLE FOUND ==========\n\
                 attempt={}\n\
                 {}\n\
                 clues={}\n\
                 hardest_technique={:?}\n\
                 hidden_singles={}\n\
                 branches={}\n\
                 backtracks={}\n\
                 =====================================\n",
                attempt,
                puzzle,
                clues,
                stats.hardest_technique,
                stats.hidden_singles,
                stats.branches,
                stats.backtracks,
            );

            return;
        }

        if attempt % 1000 == 0 {
            println!("Searched {} generated puzzles...", attempt);
        }
    }

    panic!("No Hard puzzle found in 10,000 generated puzzles");
}