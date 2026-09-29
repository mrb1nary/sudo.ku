use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use sudoku_engine::solver::solve;

mod puzzles;

fn benchmark_solver(c: &mut Criterion) {
    let puzzles = puzzles::benchmark_puzzles();

    c.bench_function("solve_10_puzzles", |b| {
        b.iter(|| {
            for puzzle in &puzzles {
                let mut board = puzzle.clone();

                solve(black_box(&mut board));
            }
        });
    });
}

criterion_group!(benches, benchmark_solver);
criterion_main!(benches);