use wasm_bindgen::prelude::*;

use sudoku_engine::{
    board::Board,
    generator::{generate_with_difficulty, Difficulty},
    solver::{
        solve,
        solve_with_config,
        solve_with_stats_config,
        SolverConfig,
        SolverStats,
    },
};

#[wasm_bindgen]
pub struct SudokuGame {
    board: Board,
    givens: [bool; 81],
    solution: Option<Board>,
}
impl Default for SudokuGame {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl SudokuGame {
    // --------------------------------------------------
    // Constructors
    // --------------------------------------------------


    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut board = Board::base();

        for row in 0..9 {
            for col in 0..9 {
                board.clear(row, col);
            }
        }

        Self {
            board,
            givens: [false; 81],
            solution: None,
        }
    }

    pub fn base() -> Self {
        let board = Board::base();

        Self {
            board: board.clone(),
            givens: [true; 81],
            solution: Some(board),
        }
    }

    pub fn random() -> Self {
        let board = Board::random();

        Self {
            board: board.clone(),
            givens: [true; 81],
            solution: Some(board),
        }
    }

    pub fn generate(difficulty: u8) -> Option<Self> {
        let difficulty = match difficulty {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            2 => Difficulty::Hard,
            _ => return None,
        };

        let puzzle = generate_with_difficulty(difficulty)?;

        // The generated puzzle is guaranteed to have a solution,
        // but solve a clone so that the original puzzle remains intact.
        let mut solution = puzzle.clone();

        if !solve(&mut solution) {
            return None;
        }

        let mut givens = [false; 81];

        for row in 0..9 {
            for col in 0..9 {
                if puzzle.get(row, col) != 0 {
                    givens[row * 9 + col] = true;
                }
            }
        }

        Some(Self {
            board: puzzle,
            givens,
            solution: Some(solution),
        })
    }


    #[wasm_bindgen]
    pub fn from_puzzle(puzzle: Vec<u8>) -> Option<Self> {
        if puzzle.len() != 81 {
            return None;
        }

        let mut board = Board::base();

        for row in 0..9 {
            for col in 0..9 {
                board.clear(row, col);
            }
        }

        for row in 0..9 {
            for col in 0..9 {
                let index = row * 9 + col;
                let value = puzzle[index];

                if value > 9 {
                    return None;
                }

                if value != 0 {
                    if !board.can_place(row, col, value) {
                        return None;
                    }

                    board.set(row, col, value);
                }
            }
        }

        let givens: [bool; 81] = puzzle
            .iter()
            .map(|&value| value != 0)
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;

        let mut solution = board.clone();

        if !solve(&mut solution) {
            return None;
        }

        Some(Self {
            board,
            givens,
            solution: Some(solution),
        })
    }

    // --------------------------------------------------
    // Board access
    // --------------------------------------------------

    pub fn get_cell(&self, row: usize, col: usize) -> u8 {
        if row >= 9 || col >= 9 {
            return 0;
        }

        self.board.get(row, col)
    }

    pub fn get_board(&self) -> Vec<u8> {
        let mut cells = Vec::with_capacity(81);

        for row in 0..9 {
            for col in 0..9 {
                cells.push(self.board.get(row, col));
            }
        }

        cells
    }

    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        if row >= 9 || col >= 9 {
            return false;
        }

        self.board.is_empty(row, col)
    }

    pub fn is_given(&self, row: usize, col: usize) -> bool {
        if row >= 9 || col >= 9 {
            return false;
        }

        self.givens[row * 9 + col]
    }

    // --------------------------------------------------
    // Gameplay
    // --------------------------------------------------

    pub fn make_move(&mut self, row: usize, col: usize, value: u8) -> bool {
        if row >= 9 || col >= 9 {
            return false;
        }

        if self.is_given(row, col) {
            return false;
        }

        if !(1..=9).contains(&value) {
            return false;
        }

        let Some(solution) = &self.solution else {
            return false;
        };

        // Only allow the correct solution value.
        if solution.get(row, col) != value {
            return false;
        }

        // Keep the normal Sudoku legality check as well.
        if !self.board.can_place(row, col, value) {
            return false;
        }

        self.board.set(row, col, value);

        true
    }

    pub fn clear_move(&mut self, row: usize, col: usize) -> bool {
        if row >= 9 || col >= 9 {
            return false;
        }

        if self.is_given(row, col) {
            return false;
        }

        if self.board.is_empty(row, col) {
            return false;
        }

        self.board.clear(row, col);

        true
    }

    pub fn reset(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                if !self.givens[row * 9 + col] {
                    self.board.clear(row, col);
                }
            }
        }
    }

    // --------------------------------------------------
    // Game state
    // --------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.board.find_empty().is_none()
    }

    pub fn is_solved(&self) -> bool {
        let Some(solution) = &self.solution else {
            return false;
        };

        for row in 0..9 {
            for col in 0..9 {
                if self.board.get(row, col) != solution.get(row, col) {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_correct(&self, row: usize, col: usize, value: u8) -> bool {
        if row >= 9 || col >= 9 {
            return false;
        }

        let Some(solution) = &self.solution else {
            return false;
        };

        solution.get(row, col) == value
    }

    // --------------------------------------------------
    // Sudoku helpers
    // --------------------------------------------------

    pub fn can_place(&self, row: usize, col: usize, value: u8) -> bool {
        if row >= 9 || col >= 9 {
            return false;
        }

        if !(1..=9).contains(&value) {
            return false;
        }

        self.board.can_place(row, col, value)
    }

    pub fn find_empty(&self) -> Option<Vec<usize>> {
        self.board
            .find_empty()
            .map(|(row, col)| vec![row, col])
    }

    // --------------------------------------------------
    // Solver
    // --------------------------------------------------

    pub fn solve(&mut self) -> bool {
        solve(&mut self.board)
    }

    pub fn solve_with_config(
        &mut self,
        use_hidden_singles: bool,
    ) -> bool {
        let config = SolverConfig {
            use_hidden_singles,
        };

        solve_with_config(&mut self.board, config)
    }

    pub fn solve_with_stats(&mut self) -> Vec<usize> {
        let config = SolverConfig {
            use_hidden_singles: true,
        };

        let stats = solve_with_stats_config(&mut self.board, config);

        Self::stats_to_vec(&stats)
    }

    pub fn solve_with_stats_config(
        &mut self,
        use_hidden_singles: bool,
    ) -> Vec<usize> {
        let config = SolverConfig {
            use_hidden_singles,
        };

        let stats = solve_with_stats_config(&mut self.board, config);

        Self::stats_to_vec(&stats)
    }

    fn stats_to_vec(stats: &SolverStats) -> Vec<usize> {
        vec![
            stats.recursive_calls,
            stats.forced_moves,
            stats.hidden_singles,
            stats.branches,
            stats.backtracks,
        ]
    }
}