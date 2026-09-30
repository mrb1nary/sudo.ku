use crate::board::Board;
use crate::solver::{CandidateCache, Trail, Undo};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Technique {
    NakedSingle,
    HiddenSingle,
    NakedPair,
    NakedTriple,
    NakedQuad,
    LockedCandidates,
    XWing,
    Swordfish,
    Guess,
}

impl Default for Technique {
    fn default() -> Self {
        Self::NakedSingle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn from_profile(profile: &SolverProfile) -> Self {
        match profile.hardest_technique {
            Technique::NakedSingle => Self::Easy,
            Technique::HiddenSingle
            | Technique::NakedPair
            | Technique::NakedTriple
            | Technique::NakedQuad
            | Technique::LockedCandidates
            | Technique::XWing
            | Technique::Swordfish => Self::Medium,
            Technique::Guess => Self::Hard,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SolverProfile {
    pub clues: usize,
    pub recursive_calls: usize,
    pub forced_moves: usize,
    pub hidden_singles: usize,
    pub branches: usize,
    pub backtracks: usize,
    pub hardest_technique: Technique,
}

impl SolverProfile {
    pub fn difficulty(&self) -> Difficulty {
        Difficulty::from_profile(self)
    }
}

// -----------------------------------------------------------------------------
// Naked Single
// -----------------------------------------------------------------------------

pub(crate) fn find_naked_single(
    board: &Board,
    candidates: &CandidateCache,
) -> Option<(usize, usize, u8)> {
    for row in 0..9 {
        for col in 0..9 {
            if board.is_empty(row, col) {
                let mask = candidates[row][col];
                if mask.count_ones() == 1 {
                    return Some((row, col, mask_to_value(mask)));
                }
            }
        }
    }
    None
}

// -----------------------------------------------------------------------------
// Hidden Single
// -----------------------------------------------------------------------------

pub(crate) fn find_hidden_single(
    board: &Board,
    candidates: &CandidateCache,
) -> Option<(usize, usize, u8)> {
    for row in 0..9 {
        if let Some(result) = find_hidden_single_in_row(board, candidates, row) {
            return Some(result);
        }
    }

    for col in 0..9 {
        if let Some(result) = find_hidden_single_in_column(board, candidates, col) {
            return Some(result);
        }
    }

    for box_row in 0..3 {
        for box_col in 0..3 {
            if let Some(result) = find_hidden_single_in_box(
                board,
                candidates,
                box_row,
                box_col,
            ) {
                return Some(result);
            }
        }
    }

    None
}

fn find_hidden_single_in_row(
    board: &Board,
    candidates: &CandidateCache,
    row: usize,
) -> Option<(usize, usize, u8)> {
    let mut locations = [None; 9];
    let mut counts = [0u8; 9];

    for col in 0..9 {
        if board.is_empty(row, col) {
            record_candidate_locations(candidates[row][col], col, &mut locations, &mut counts);
        }
    }

    find_location(locations, counts, |col| (row, col))
}

fn find_hidden_single_in_column(
    board: &Board,
    candidates: &CandidateCache,
    col: usize,
) -> Option<(usize, usize, u8)> {
    let mut locations = [None; 9];
    let mut counts = [0u8; 9];

    for row in 0..9 {
        if board.is_empty(row, col) {
            record_candidate_locations(candidates[row][col], row, &mut locations, &mut counts);
        }
    }

    find_location(locations, counts, |row| (row, col))
}

fn find_hidden_single_in_box(
    board: &Board,
    candidates: &CandidateCache,
    box_row: usize,
    box_col: usize,
) -> Option<(usize, usize, u8)> {
    let mut locations = [None; 9];
    let mut counts = [0u8; 9];
    let start_row = box_row * 3;
    let start_col = box_col * 3;

    for row in start_row..start_row + 3 {
        for col in start_col..start_col + 3 {
            if board.is_empty(row, col) {
                record_candidate_locations(
                    candidates[row][col],
                    row * 9 + col,
                    &mut locations,
                    &mut counts,
                );
            }
        }
    }

    find_location(locations, counts, |location| (location / 9, location % 9))
}

fn record_candidate_locations(
    mask: u16,
    location: usize,
    locations: &mut [Option<usize>; 9],
    counts: &mut [u8; 9],
) {
    for digit in 0..9 {
        if mask & (1 << digit) != 0 {
            counts[digit] += 1;
            if counts[digit] == 1 {
                locations[digit] = Some(location);
            }
        }
    }
}

fn find_location(
    locations: [Option<usize>; 9],
    counts: [u8; 9],
    make_location: impl Fn(usize) -> (usize, usize),
) -> Option<(usize, usize, u8)> {
    for digit in 0..9 {
        if counts[digit] == 1 {
            let location = locations[digit].unwrap();
            let (row, col) = make_location(location);
            return Some((row, col, digit as u8 + 1));
        }
    }
    None
}

// -----------------------------------------------------------------------------
// Naked Pair / Triple / Quad
// -----------------------------------------------------------------------------

pub(crate) fn find_naked_pair(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    for cells in all_units() {
        if let Some((pair_cells, pair_mask)) = find_naked_n_in_unit(board, candidates, &cells, 2) {
            if eliminate_naked_n_from_unit(
                board,
                candidates,
                trail,
                &cells,
                &pair_cells,
                pair_mask,
            ) {
                return true;
            }
        }
    }
    false
}

pub(crate) fn find_naked_triple(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    for cells in all_units() {
        if let Some((triple_cells, triple_mask)) = find_naked_n_in_unit(board, candidates, &cells, 3) {
            if eliminate_naked_n_from_unit(
                board,
                candidates,
                trail,
                &cells,
                &triple_cells,
                triple_mask,
            ) {
                return true;
            }
        }
    }
    false
}

pub(crate) fn find_naked_quad(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    for cells in all_units() {
        if let Some((quad_cells, quad_mask)) = find_naked_n_in_unit(board, candidates, &cells, 4) {
            if eliminate_naked_n_from_unit(
                board,
                candidates,
                trail,
                &cells,
                &quad_cells,
                quad_mask,
            ) {
                return true;
            }
        }
    }
    false
}

fn find_naked_n_in_unit(
    board: &Board,
    candidates: &CandidateCache,
    cells: &[(usize, usize)],
    n: usize,
) -> Option<(Vec<usize>, u16)> {
    let eligible: Vec<(usize, u16)> = cells
        .iter()
        .filter_map(|&(row, col)| {
            if board.is_empty(row, col) {
                let mask = candidates[row][col];
                let count = mask.count_ones() as usize;
                if (2..=n).contains(&count) {
                    return Some((row * 9 + col, mask));
                }
            }
            None
        })
        .collect();

    let mut chosen = Vec::with_capacity(n);
    find_naked_combination(&eligible, n, 0, &mut chosen, 0)
}

fn find_naked_combination(
    eligible: &[(usize, u16)],
    n: usize,
    start: usize,
    chosen: &mut Vec<usize>,
    mask: u16,
) -> Option<(Vec<usize>, u16)> {
    if chosen.len() == n {
        if mask.count_ones() == n as u32 {
            return Some((chosen.clone(), mask));
        }
        return None;
    }

    for index in start..eligible.len() {
        let next_mask = mask | eligible[index].1;
        if next_mask.count_ones() > n as u32 {
            continue;
        }

        chosen.push(eligible[index].0);
        if let Some(result) = find_naked_combination(
            eligible,
            n,
            index + 1,
            chosen,
            next_mask,
        ) {
            return Some(result);
        }
        chosen.pop();
    }

    None
}

fn eliminate_naked_n_from_unit(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
    cells: &[(usize, usize)],
    pattern_cells: &[usize],
    pattern_mask: u16,
) -> bool {
    let mut changed = false;

    for &(row, col) in cells {
        let index = row * 9 + col;
        if pattern_cells.contains(&index) || !board.is_empty(row, col) {
            continue;
        }

        let previous = candidates[row][col];
        let next = previous & !pattern_mask;
        if previous == next {
            continue;
        }

        trail.push(Undo::Candidate { row, col, previous });
        candidates[row][col] = next;
        changed = true;
    }

    changed
}

// -----------------------------------------------------------------------------
// Locked Candidates
//
// Includes both:
//   - Pointing: box -> row/column
//   - Claiming: row/column -> box
// -----------------------------------------------------------------------------

pub(crate) fn find_locked_candidates(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    find_pointing(board, candidates, trail) || find_claiming(board, candidates, trail)
}

fn find_pointing(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    for box_row in 0..3 {
        for box_col in 0..3 {
            let start_row = box_row * 3;
            let start_col = box_col * 3;

            for digit in 0..9 {
                let bit = 1u16 << digit;
                let mut locations = Vec::new();

                for row in start_row..start_row + 3 {
                    for col in start_col..start_col + 3 {
                        if board.is_empty(row, col) && candidates[row][col] & bit != 0 {
                            locations.push((row, col));
                        }
                    }
                }

                if locations.len() < 2 {
                    continue;
                }

                let same_row = locations.iter().all(|&(row, _)| row == locations[0].0);
                if same_row {
                    let row = locations[0].0;
                    for col in 0..9 {
                        if (col / 3 == box_col) || !board.is_empty(row, col) {
                            continue;
                        }
                        if remove_candidate(candidates, trail, row, col, bit) {
                            return true;
                        }
                    }
                }

                let same_col = locations.iter().all(|&(_, col)| col == locations[0].1);
                if same_col {
                    let col = locations[0].1;
                    for row in 0..9 {
                        if (row / 3 == box_row) || !board.is_empty(row, col) {
                            continue;
                        }
                        if remove_candidate(candidates, trail, row, col, bit) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

fn find_claiming(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    for row in 0..9 {
        for digit in 0..9 {
            let bit = 1u16 << digit;
            let cols: Vec<usize> = (0..9)
                .filter(|&col| board.is_empty(row, col) && candidates[row][col] & bit != 0)
                .collect();

            if cols.len() < 2 {
                continue;
            }

            let box_col = cols[0] / 3;
            if cols.iter().all(|&col| col / 3 == box_col) {
                let box_row = row / 3;
                for box_r in box_row * 3..box_row * 3 + 3 {
                    for box_c in box_col * 3..box_col * 3 + 3 {
                        if box_r == row || !board.is_empty(box_r, box_c) {
                            continue;
                        }
                        if remove_candidate(candidates, trail, box_r, box_c, bit) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    for col in 0..9 {
        for digit in 0..9 {
            let bit = 1u16 << digit;
            let rows: Vec<usize> = (0..9)
                .filter(|&row| board.is_empty(row, col) && candidates[row][col] & bit != 0)
                .collect();

            if rows.len() < 2 {
                continue;
            }

            let box_row = rows[0] / 3;
            if rows.iter().all(|&row| row / 3 == box_row) {
                let box_col = col / 3;
                for box_r in box_row * 3..box_row * 3 + 3 {
                    for box_c in box_col * 3..box_col * 3 + 3 {
                        if box_c == col || !board.is_empty(box_r, box_c) {
                            continue;
                        }
                        if remove_candidate(candidates, trail, box_r, box_c, bit) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

// -----------------------------------------------------------------------------
// X-Wing
// -----------------------------------------------------------------------------

pub(crate) fn find_x_wing(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    find_x_wing_rows(board, candidates, trail) || find_x_wing_columns(board, candidates, trail)
}

fn find_x_wing_rows(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    for digit in 0..9 {
        let bit = 1u16 << digit;
        let mut row_masks = [0u16; 9];

        for row in 0..9 {
            let mut mask = 0u16;
            for col in 0..9 {
                if board.is_empty(row, col) && candidates[row][col] & bit != 0 {
                    mask |= 1 << col;
                }
            }
            if mask.count_ones() == 2 {
                row_masks[row] = mask;
            }
        }

        for row_a in 0..9 {
            if row_masks[row_a] == 0 {
                continue;
            }
            for row_b in row_a + 1..9 {
                if row_masks[row_a] != row_masks[row_b] || row_masks[row_b] == 0 {
                    continue;
                }

                for row in 0..9 {
                    if row == row_a || row == row_b {
                        continue;
                    }
                    for col in 0..9 {
                        if row_masks[row_a] & (1 << col) != 0
                            && remove_candidate(candidates, trail, row, col, bit)
                        {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

fn find_x_wing_columns(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    for digit in 0..9 {
        let bit = 1u16 << digit;
        let mut col_masks = [0u16; 9];

        for col in 0..9 {
            let mut mask = 0u16;
            for row in 0..9 {
                if board.is_empty(row, col) && candidates[row][col] & bit != 0 {
                    mask |= 1 << row;
                }
            }
            if mask.count_ones() == 2 {
                col_masks[col] = mask;
            }
        }

        for col_a in 0..9 {
            if col_masks[col_a] == 0 {
                continue;
            }
            for col_b in col_a + 1..9 {
                if col_masks[col_a] != col_masks[col_b] || col_masks[col_b] == 0 {
                    continue;
                }

                for col in 0..9 {
                    if col == col_a || col == col_b {
                        continue;
                    }
                    for row in 0..9 {
                        if col_masks[col_a] & (1 << row) != 0
                            && remove_candidate(candidates, trail, row, col, bit)
                        {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

// -----------------------------------------------------------------------------
// Swordfish
// -----------------------------------------------------------------------------

pub(crate) fn find_swordfish(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    find_swordfish_rows(board, candidates, trail) || find_swordfish_columns(board, candidates, trail)
}

fn find_swordfish_rows(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    for digit in 0..9 {
        let bit = 1u16 << digit;
        let mut masks = [0u16; 9];

        for row in 0..9 {
            let mut mask = 0u16;
            for col in 0..9 {
                if board.is_empty(row, col) && candidates[row][col] & bit != 0 {
                    mask |= 1 << col;
                }
            }
            let count = mask.count_ones();
            if (2..=3).contains(&count) {
                masks[row] = mask;
            }
        }

        for a in 0..9 {
            if masks[a] == 0 {
                continue;
            }
            for b in a + 1..9 {
                if masks[b] == 0 {
                    continue;
                }
                for c in b + 1..9 {
                    if masks[c] == 0 {
                        continue;
                    }

                    let union = masks[a] | masks[b] | masks[c];
                    if union.count_ones() != 3 {
                        continue;
                    }

                    for row in 0..9 {
                        if row == a || row == b || row == c {
                            continue;
                        }
                        for col in 0..9 {
                            if union & (1 << col) != 0
                                && remove_candidate(candidates, trail, row, col, bit)
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

fn find_swordfish_columns(
    board: &Board,
    candidates: &mut CandidateCache,
    trail: &mut Trail,
) -> bool {
    for digit in 0..9 {
        let bit = 1u16 << digit;
        let mut masks = [0u16; 9];

        for col in 0..9 {
            let mut mask = 0u16;
            for row in 0..9 {
                if board.is_empty(row, col) && candidates[row][col] & bit != 0 {
                    mask |= 1 << row;
                }
            }
            let count = mask.count_ones();
            if (2..=3).contains(&count) {
                masks[col] = mask;
            }
        }

        for a in 0..9 {
            if masks[a] == 0 {
                continue;
            }
            for b in a + 1..9 {
                if masks[b] == 0 {
                    continue;
                }
                for c in b + 1..9 {
                    if masks[c] == 0 {
                        continue;
                    }

                    let union = masks[a] | masks[b] | masks[c];
                    if union.count_ones() != 3 {
                        continue;
                    }

                    for col in 0..9 {
                        if col == a || col == b || col == c {
                            continue;
                        }
                        for row in 0..9 {
                            if union & (1 << row) != 0
                                && remove_candidate(candidates, trail, row, col, bit)
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

// -----------------------------------------------------------------------------
// Shared helpers
// -----------------------------------------------------------------------------

fn all_units() -> Vec<Vec<(usize, usize)>> {
    let mut units = Vec::with_capacity(27);

    for row in 0..9 {
        units.push((0..9).map(|col| (row, col)).collect());
    }

    for col in 0..9 {
        units.push((0..9).map(|row| (row, col)).collect());
    }

    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut cells = Vec::with_capacity(9);
            for row in box_row * 3..box_row * 3 + 3 {
                for col in box_col * 3..box_col * 3 + 3 {
                    cells.push((row, col));
                }
            }
            units.push(cells);
        }
    }

    units
}

#[inline]
fn remove_candidate(
    candidates: &mut CandidateCache,
    trail: &mut Trail,
    row: usize,
    col: usize,
    bit: u16,
) -> bool {
    let previous = candidates[row][col];
    let next = previous & !bit;

    if previous == next {
        return false;
    }

    trail.push(Undo::Candidate { row, col, previous });
    candidates[row][col] = next;
    true
}

#[inline]
fn mask_to_value(mask: u16) -> u8 {
    mask.trailing_zeros() as u8 + 1
}
