use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use sudoku_engine::board::Board;
use sudoku_engine::solver::{solve};

mod common;

use common::puzzles::{easy_puzzles, hard_puzzles, medium_puzzles};

fn benchmark_puzzles(puzzles: &[Board]) {
    for puzzle in puzzles {
        let mut board = puzzle.clone();

        black_box(solve(&mut board));
    }
}

fn analyze_puzzles(name: &str, puzzles: &[Board]) {
    println!("\n{name} puzzles:");

    for (index, puzzle) in puzzles.iter().enumerate() {
        let mut with_hidden = puzzle.clone();
        let mut without_hidden = puzzle.clone();

        let hidden_profile = sudoku_engine::solver::profile(&mut with_hidden);

        let no_hidden_profile = sudoku_engine::solver::profile_with_config(
            &mut without_hidden,
            sudoku_engine::solver::SolverConfig {
                use_hidden_singles: false,
            },
        );

        println!("Puzzle {}:", index + 1);

        println!(
            "hidden=true  → clues={}, recursive={}, forced={}, hidden={}, branches={}, backtracks={}",
            hidden_profile.clues,
            hidden_profile.recursive_calls,
            hidden_profile.forced_moves,
            hidden_profile.hidden_singles,
            hidden_profile.branches,
            hidden_profile.backtracks,
        );

        println!(
            "hidden=false → clues={}, recursive={}, forced={}, hidden={}, branches={}, backtracks={}",
            no_hidden_profile.clues,
            no_hidden_profile.recursive_calls,
            no_hidden_profile.forced_moves,
            no_hidden_profile.hidden_singles,
            no_hidden_profile.branches,
            no_hidden_profile.backtracks,
        );
    }
}
fn benchmark_solver(c: &mut Criterion) {
    let easy = easy_puzzles();
    let medium = medium_puzzles();
    let hard = hard_puzzles();

    analyze_puzzles("Easy", &easy);
    analyze_puzzles("Medium", &medium);
    analyze_puzzles("Hard", &hard);

    let mut group = c.benchmark_group("solver");

    group.bench_function("clone", |b| {
        b.iter(|| {
            for puzzle in &hard {
                black_box(puzzle.clone());
            }
        });
    });

    group.bench_function("easy", |b| {
        b.iter(|| {
            benchmark_puzzles(black_box(&easy));
        });
    });

    group.bench_function("medium", |b| {
        b.iter(|| {
            benchmark_puzzles(black_box(&medium));
        });
    });

    group.bench_function("hard", |b| {
        b.iter(|| {
            benchmark_puzzles(black_box(&hard));
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_solver);
criterion_main!(benches);
