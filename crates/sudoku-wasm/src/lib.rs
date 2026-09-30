use wasm_bindgen::prelude::*;

use sudoku_engine::{
    board::Board,
    solver::{SolverConfig, SolverStats, solve, solve_with_config, solve_with_stats_config},
};

#[wasm_bindgen]
pub struct SudokuGame {
    #[wasm_bindgen(skip)]
    board: Board,

    #[wasm_bindgen(skip)]
    givens: [bool; 81],
}

#[wasm_bindgen]
impl SudokuGame {
    // --------------------------------------------------
    // Construction
    // --------------------------------------------------

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            board: Board::empty(),
            givens: [false; 81],
        }
    }

    pub fn base() -> Self {
        Self {
            board: Board::base(),
            givens: [true; 81],
        }
    }

    pub fn random() -> Self {
        Self {
            board: Board::random(),
            givens: [true; 81],
        }
    }

    // --------------------------------------------------
    // Cell access
    // --------------------------------------------------

    pub fn get_cell(&self, row: usize, col: usize) -> u8 {
        self.board.get(row, col)
    }

    pub fn set_cell(&mut self, row: usize, col: usize, value: u8) {
        self.board.set(row, col, value);
    }

    pub fn clear_cell(&mut self, row: usize, col: usize) {
        self.board.clear(row, col);
    }

    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.board.is_empty(row, col)
    }

    // --------------------------------------------------
    // Game state
    // --------------------------------------------------

    pub fn is_given(&self, row: usize, col: usize) -> bool {
        self.givens[row * 9 + col]
    }

    pub fn make_move(&mut self, row: usize, col: usize, value: u8) -> bool {
        let index = row * 9 + col;

        // Given cells cannot be changed.
        if self.givens[index] {
            return false;
        }

        // A move must obey Sudoku rules.
        if !self.board.can_place(row, col, value) {
            return false;
        }

        self.board.set(row, col, value);

        true
    }

    pub fn clear_move(&mut self, row: usize, col: usize) -> bool {
        let index = row * 9 + col;

        // Given cells cannot be cleared.
        if self.givens[index] {
            return false;
        }

        self.board.clear(row, col);

        true
    }

    // --------------------------------------------------
    // Board access
    // --------------------------------------------------

    pub fn get_board(&self) -> Vec<u8> {
        let mut cells = Vec::with_capacity(81);

        for row in 0..9 {
            for col in 0..9 {
                cells.push(self.board.get(row, col));
            }
        }

        cells
    }

    // --------------------------------------------------
    // Validation
    // --------------------------------------------------

    pub fn can_place(&self, row: usize, col: usize, value: u8) -> bool {
        self.board.can_place(row, col, value)
    }

    pub fn find_empty(&self) -> Vec<usize> {
        match self.board.find_empty() {
            Some((row, col)) => vec![row, col],
            None => Vec::new(),
        }
    }

    // --------------------------------------------------
    // Board transformations
    // --------------------------------------------------

    pub fn swap_digits(&mut self, a: u8, b: u8) {
        self.board.swap_digits(a, b);
    }

    pub fn swap_rows(&mut self, row_a: usize, row_b: usize) {
        self.board.swap_rows(row_a, row_b);
    }

    pub fn swap_columns(&mut self, col_a: usize, col_b: usize) {
        self.board.swap_columns(col_a, col_b);
    }

    pub fn swap_bands(&mut self, band_a: usize, band_b: usize) {
        self.board.swap_bands(band_a, band_b);
    }

    pub fn swap_stacks(&mut self, stack_a: usize, stack_b: usize) {
        self.board.swap_stacks(stack_a, stack_b);
    }

    // --------------------------------------------------
    // Solver
    // --------------------------------------------------

    pub fn solve(&mut self) -> bool {
        solve(&mut self.board)
    }

    pub fn solve_with_config(&mut self, use_hidden_singles: bool) -> bool {
        let config = SolverConfig { use_hidden_singles };

        solve_with_config(&mut self.board, config)
    }

    pub fn solve_with_stats(&mut self) -> Vec<u32> {
        let stats = solve_with_stats_config(&mut self.board, SolverConfig::default());

        stats_to_vec(stats)
    }

    pub fn solve_with_stats_config(&mut self, use_hidden_singles: bool) -> Vec<u32> {
        let config = SolverConfig { use_hidden_singles };

        let stats = solve_with_stats_config(&mut self.board, config);

        stats_to_vec(stats)
    }
}

fn stats_to_vec(stats: SolverStats) -> Vec<u32> {
    vec![
        stats.recursive_calls as u32,
        stats.forced_moves as u32,
        stats.hidden_singles as u32,
        stats.branches as u32,
        stats.backtracks as u32,
    ]
}
