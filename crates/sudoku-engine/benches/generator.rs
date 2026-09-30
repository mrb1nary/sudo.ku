use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use sudoku_engine::generator::{generate, generate_with_difficulty};
use sudoku_engine::solver::Difficulty;

fn benchmark_generate(c: &mut Criterion) {
    c.bench_function("generator/generate", |b| {
        b.iter(|| {
            black_box(generate());
        });
    });
}

fn benchmark_generate_easy(c: &mut Criterion) {
    c.bench_function("generator/generate_easy", |b| {
        b.iter(|| {
            black_box(generate_with_difficulty(Difficulty::Easy));
        });
    });
}

fn benchmark_generate_medium(c: &mut Criterion) {
    c.bench_function("generator/generate_medium", |b| {
        b.iter(|| {
            black_box(generate_with_difficulty(Difficulty::Medium));
        });
    });
}

fn benchmark_generate_hard(c: &mut Criterion) {
    c.bench_function("generator/generate_hard", |b| {
        b.iter(|| {
            black_box(generate_with_difficulty(Difficulty::Hard));
        });
    });
}

criterion_group!(
    benches,
    benchmark_generate,
    benchmark_generate_easy,
    benchmark_generate_medium,
    benchmark_generate_hard
);

criterion_main!(benches);