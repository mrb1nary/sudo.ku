use sudoku_engine::{
    board::Board,
    generator::generate,
    solver::{SolverConfig, solve_with_stats, solve_with_stats_config},
};

fn count_clues(board: &Board) -> usize {
    let mut clues = 0;

    for row in 0..9 {
        for col in 0..9 {
            if board.get(row, col) != 0 {
                clues += 1;
            }
        }
    }

    clues
}

fn main() {
    const SAMPLES: usize = 1_000;

    let mut clue_counts = Vec::with_capacity(SAMPLES);
    let mut hidden_single_counts = Vec::with_capacity(SAMPLES);

    let mut branch_counts = Vec::with_capacity(SAMPLES);
    let mut backtrack_counts = Vec::with_capacity(SAMPLES);
    let mut recursive_call_counts = Vec::with_capacity(SAMPLES);

    for _ in 0..SAMPLES {
        // Generate one puzzle.
        let puzzle = generate();

        // Keep the same puzzle for both solver configurations.
        let mut with_hidden = puzzle.clone();
        let mut without_hidden = puzzle;

        let clues = count_clues(&with_hidden);

        let stats_with_hidden = solve_with_stats(&mut with_hidden);

        let stats_without_hidden = solve_with_stats_config(
            &mut without_hidden,
            SolverConfig {
                use_hidden_singles: false,
            },
        );

        clue_counts.push(clues);
        hidden_single_counts.push(stats_with_hidden.hidden_singles);

        branch_counts.push(stats_without_hidden.branches);
        backtrack_counts.push(stats_without_hidden.backtracks);
        recursive_call_counts.push(stats_without_hidden.recursive_calls);
    }

    print_stats("Clues", &clue_counts);
    print_stats("Hidden singles", &hidden_single_counts);
    print_stats("Branches (no HS)", &branch_counts);
    print_stats("Backtracks (no HS)", &backtrack_counts);
    print_stats("Recursive calls (no HS)", &recursive_call_counts);
    print_branch_distribution(&branch_counts);

    print_percentiles("Clues", &clue_counts);
    print_percentiles("Hidden singles", &hidden_single_counts);
    print_percentiles("Branches (no HS)", &branch_counts);
    print_percentiles("Backtracks (no HS)", &backtrack_counts);
    print_percentiles("Recursive calls (no HS)", &recursive_call_counts);
}

fn percentile(values: &[usize], percentile: f64) -> usize {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();

    let index = ((sorted.len() - 1) as f64 * percentile).round() as usize;

    sorted[index]
}

fn print_percentiles(name: &str, values: &[usize]) {
    println!("\n{name}:");

    println!("  P50  = {}", percentile(values, 0.50));
    println!("  P75  = {}", percentile(values, 0.75));
    println!("  P90  = {}", percentile(values, 0.90));
    println!("  P95  = {}", percentile(values, 0.95));
    println!("  P99  = {}", percentile(values, 0.99));
    println!("  P100 = {}", percentile(values, 1.00));
}
fn print_stats(name: &str, values: &[usize]) {
    let min = *values.iter().min().unwrap();
    let max = *values.iter().max().unwrap();

    let sum: usize = values.iter().sum();
    let average = sum as f64 / values.len() as f64;

    println!("{name:<24} min={min:<4} max={max:<4} avg={average:.2}");
}

fn print_branch_distribution(values: &[usize]) {
    let buckets = [
        ("0", 0, 0),
        ("1-10", 1, 10),
        ("11-50", 11, 50),
        ("51-100", 51, 100),
        ("101-500", 101, 500),
        ("501+", 501, usize::MAX),
    ];

    println!("\nBranch distribution:");

    for (name, min, max) in buckets {
        let count = values
            .iter()
            .filter(|&&value| value >= min && value <= max)
            .count();

        println!("  {name:<8} {count:>4}");
    }
}
