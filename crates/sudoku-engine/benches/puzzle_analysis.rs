mod common;

use common::puzzles::{easy_puzzles, hard_puzzles, medium_puzzles};

use sudoku_engine::solver::{SolverConfig, solve_with_stats, solve_with_stats_config};

fn analyze_group(name: &str, puzzles: Vec<sudoku_engine::board::Board>) {
    println!("\n{name} puzzles:");

    for (index, puzzle) in puzzles.into_iter().enumerate() {
        let mut with_hidden = puzzle.clone();
        let mut without_hidden = puzzle;

        let hidden_stats = solve_with_stats(&mut with_hidden);

        let no_hidden_stats = solve_with_stats_config(
            &mut without_hidden,
            SolverConfig {
                use_hidden_singles: false,
            },
        );

        println!(
            "  Puzzle {}:
    hidden=true  → recursive={}, forced={}, hidden={}, branches={}, backtracks={}
    hidden=false → recursive={}, forced={}, hidden={}, branches={}, backtracks={}",
            index + 1,
            hidden_stats.recursive_calls,
            hidden_stats.forced_moves,
            hidden_stats.hidden_singles,
            hidden_stats.branches,
            hidden_stats.backtracks,
            no_hidden_stats.recursive_calls,
            no_hidden_stats.forced_moves,
            no_hidden_stats.hidden_singles,
            no_hidden_stats.branches,
            no_hidden_stats.backtracks,
        );
    }
}

fn main() {
    analyze_group("Easy", easy_puzzles());
    analyze_group("Medium", medium_puzzles());
    analyze_group("Hard", hard_puzzles());
}
