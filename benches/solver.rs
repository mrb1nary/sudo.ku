use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use sudoku_engine::board::Board;
use sudoku_engine::solver::solve;

mod common;

use common::puzzles::{easy_puzzles, hard_puzzles, medium_puzzles};

fn benchmark_puzzles(puzzles: &[Board]) {
    for puzzle in puzzles {
        let mut board = puzzle.clone();

        black_box(solve(&mut board));
    }
}

fn benchmark_solver(c: &mut Criterion) {
    let mut group = c.benchmark_group("solver");

    let easy = easy_puzzles();
    let medium = medium_puzzles();
    let hard = hard_puzzles();

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
