    use rand::{Rng, RngExt, rng};
    use std::fmt;

    #[derive(Clone)]
    pub struct Board {
        cells: [u8; 81],
    }
    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for row in 0..9 {
                for col in 0..9 {
                    write!(f, "{} ", self.get(row, col))?;
                }

                writeln!(f)?;
            }

            Ok(())
        }
    }
    impl Board {
        pub fn empty() -> Self {
            Self { cells: [0; 81] }
        }

        pub fn get(&self, row: usize, col: usize) -> u8 {
            self.cells[row * 9 + col]
        }

        pub fn set(&mut self, row: usize, col: usize, value: u8) {
            self.cells[row * 9 + col] = value;
        }

        pub fn base() -> Self {
            let mut board = Self::empty();

            for row in 0..9 {
                for col in 0..9 {
                    let value = (row * 3 + row / 3 + col) % 9 + 1;
                    board.set(row, col, value as u8);
                }
            }
            board
        }

        pub fn swap_digits(&mut self, a: u8, b: u8) {
            for cell in &mut self.cells {
                if *cell == a {
                    *cell = b;
                } else if *cell == b {
                    *cell = a;
                }
            }
        }

        pub fn swap_rows(&mut self, row_a: usize, row_b: usize) {
            for col in 0..9 {
                let index_a = row_a * 9 + col;
                let index_b = row_b * 9 + col;

                self.cells.swap(index_a, index_b);
            }
        }

        pub fn swap_columns(&mut self, col_a: usize, col_b: usize) {
            for row in 0..9 {
                let index_a = row * 9 + col_a;
                let index_b = row * 9 + col_b;

                self.cells.swap(index_a, index_b);
            }
        }

        pub fn swap_bands(&mut self, band_a: usize, band_b: usize) {
            for offset in 0..3 {
                let row_a = band_a * 3 + offset;
                let row_b = band_b * 3 + offset;

                self.swap_rows(row_a, row_b);
            }
        }

        pub fn swap_stacks(&mut self, stack_a: usize, stack_b: usize) {
            for offset in 0..3 {
                let col_a = stack_a * 3 + offset;
                let col_b = stack_b * 3 + offset;

                self.swap_columns(col_a, col_b);
            }
        }

        pub fn random_swap_digits(&mut self, rng: &mut impl Rng) {
            let a = rng.random_range(1..=9);
            let mut b = rng.random_range(1..=9);

            while a == b {
                b = rng.random_range(1..=9);
            }

            self.swap_digits(a, b);
        }

        pub fn random_swap_rows(&mut self, rng: &mut impl Rng) {
            let band = rng.random_range(0..3);

            let row_a = band * 3 + rng.random_range(0..3);

            let mut row_b = band * 3 + rng.random_range(0..3);

            while row_a == row_b {
                row_b = band * 3 + rng.random_range(0..3);
            }

            self.swap_rows(row_a, row_b);
        }

        pub fn random_swap_bands(&mut self, rng: &mut impl Rng) {
            let band_a = rng.random_range(0..3);

            let mut band_b = rng.random_range(0..3);

            while band_a == band_b {
                band_b = rng.random_range(0..3);
            }

            self.swap_bands(band_a, band_b);
        }

        pub fn random_swap_columns(&mut self, rng: &mut impl Rng) {
            let stack = rng.random_range(0..3);

            let col_a = stack * 3 + rng.random_range(0..3);

            let mut col_b = stack * 3 + rng.random_range(0..3);

            while col_a == col_b {
                col_b = stack * 3 + rng.random_range(0..3);
            }

            self.swap_columns(col_a, col_b);
        }

        pub fn random_swap_stacks(&mut self, rng: &mut impl Rng) {
            let stack_a = rng.random_range(0..3);

            let mut stack_b = rng.random_range(0..3);

            while stack_a == stack_b {
                stack_b = rng.random_range(0..3);
            }

            self.swap_stacks(stack_a, stack_b);
        }

        pub fn random() -> Self {
            let mut board = Self::base();
            let mut rng = rng();

            for _ in 0..20 {
                board.random_swap_digits(&mut rng);
                board.random_swap_rows(&mut rng);
                board.random_swap_bands(&mut rng);
                board.random_swap_columns(&mut rng);
                board.random_swap_stacks(&mut rng);
            }

            board
        }

        pub fn clear(&mut self, row: usize, col: usize) {
            self.set(row, col, 0);
        }

        pub fn is_empty(&self, row: usize, col: usize) -> bool {
            self.get(row, col) == 0
        }

        pub fn can_place(&self, row: usize, col: usize, value: u8) -> bool {
            // Check row
            for current_col in 0..9 {
                if self.get(row, current_col) == value {
                    return false;
                }
            }

            // Check column
            for current_row in 0..9 {
                if self.get(current_row, col) == value {
                    return false;
                }
            }

            // Check 3x3 box
            let box_row = (row / 3) * 3;
            let box_col = (col / 3) * 3;

            for current_row in box_row..box_row + 3 {
                for current_col in box_col..box_col + 3 {
                    if self.get(current_row, current_col) == value {
                        return false;
                    }
                }
            }

            true
        }

        pub fn find_empty(&self) -> Option<(usize, usize)> {
            for row in 0..9 {
                for col in 0..9 {
                    if self.is_empty(row, col) {
                        return Some((row, col));
                    }
                }
            }

            None
        }
    }
