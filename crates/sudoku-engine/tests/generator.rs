use sudoku_engine::board::Board;
use sudoku_engine::generator::*;
use sudoku_engine::solver::{SolverConfig, profile, profile_with_config, solve_with_stats, solve_with_stats_config, Difficulty};
use sudoku_engine::{generator::generate, solver::count_solutions};


pub fn generate_with_difficulty(
    difficulty: Difficulty,
    max_attempts: usize,
) -> Option<Board> {
    for _ in 0..max_attempts {
        let puzzle = generate();

        let mut board = puzzle.clone();
        let profile = profile(&mut board);

        if profile.difficulty() == difficulty {
            return Some(puzzle);
        }
    }

    None
}

#[test]
fn generator_can_target_all_difficulties() {
    use sudoku_engine::{
        generator::generate_with_difficulty,
        solver::{profile, Difficulty},
    };

    for difficulty in [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
    ] {
        let puzzle = generate_with_difficulty(difficulty)
            .unwrap_or_else(|| {
                panic!(
                    "failed to generate a {:?} puzzle within the configured retry limit",
                    difficulty
                )
            });

        let mut board = puzzle.clone();
        let stats = profile(&mut board);

        assert_eq!(stats.difficulty(), difficulty);

        println!(
            "\nGenerated {:?} puzzle:\n{}\n\
             hardest_technique={:?}\n\
             hidden_singles={}\n\
             branches={}\n\
             backtracks={}",
            difficulty,
            puzzle,
            stats.hardest_technique,
            stats.hidden_singles,
            stats.branches,
            stats.backtracks,
        );
    }
}

#[test]
fn generated_puzzle_has_one_solution() {
    let mut puzzle = generate();

    let solutions = count_solutions(&mut puzzle, 2);

    assert_eq!(solutions, 1);
}

#[test]
fn generated_puzzle_has_valid_clue_count() {
    let puzzle = generate();

    let mut clues = 0;

    for row in 0..9 {
        for col in 0..9 {
            if puzzle.get(row, col) != 0 {
                clues += 1;
            }
        }
    }

    println!("Generated puzzle has {clues} clues");

    assert!(clues > 0);
    assert!(clues < 81);
}

