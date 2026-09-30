use sudoku_engine::board::Board;
use sudoku_engine::solver::{profile, Technique};

fn board_from_string(puzzle: &str) -> Board {
    let puzzle: String = puzzle.chars().filter(|c| !c.is_whitespace()).collect();

    assert_eq!(puzzle.len(), 81);

    let mut board = Board::empty();

    for (index, byte) in puzzle.bytes().enumerate() {
        let value = byte - b'0';

        if value != 0 {
            board.set(index / 9, index % 9, value);
        }
    }

    board
}

fn assert_valid_solution(board: &Board) {
    for row in 0..9 {
        for col in 0..9 {
            let value = board.get(row, col);

            assert!(
                (1..=9).contains(&value),
                "empty/invalid value at r{}c{}: {}",
                row + 1,
                col + 1,
                value
            );
        }
    }

    for row in 0..9 {
        let mut seen = [false; 10];

        for col in 0..9 {
            let value = board.get(row, col) as usize;

            assert!(!seen[value], "duplicate {} in row {}", value, row + 1);

            seen[value] = true;
        }

        assert!(seen[1..].iter().all(|&present| present));
    }

    for col in 0..9 {
        let mut seen = [false; 10];

        for row in 0..9 {
            let value = board.get(row, col) as usize;

            assert!(!seen[value], "duplicate {} in column {}", value, col + 1);

            seen[value] = true;
        }

        assert!(seen[1..].iter().all(|&present| present));
    }

    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut seen = [false; 10];

            for row in box_row * 3..box_row * 3 + 3 {
                for col in box_col * 3..box_col * 3 + 3 {
                    let value = board.get(row, col) as usize;

                    assert!(
                        !seen[value],
                        "duplicate {} in box ({}, {})",
                        value, box_row, box_col
                    );

                    seen[value] = true;
                }
            }

            assert!(seen[1..].iter().all(|&present| present));
        }
    }
}

fn assert_technique(puzzle: &str, expected: Technique) {
    let mut board = board_from_string(puzzle);
    let profile = profile(&mut board);

    assert_eq!(
        profile.hardest_technique, expected,
        "expected {:?}, got {:?}\nprofile: {profile:#?}",
        expected,
        profile.hardest_technique
    );

    assert_eq!(profile.branches, 0);
    assert_eq!(profile.backtracks, 0);

    assert_valid_solution(&board);
}

#[test]
fn naked_pair() {
    assert_technique(
        "\
        208016090\
        060000080\
        000025000\
        001308405\
        040060000\
        003000000\
        400000000\
        020001060\
        030780000",
        Technique::NakedPair,
    );
}

#[test]
fn naked_triple() {
    assert_technique(
        "\
        002010700\
        408000600\
        000302000\
        000000950\
        506031000\
        040006020\
        009000000\
        000780000\
        000124870",
        Technique::NakedTriple,
    );
}

#[test]
fn naked_quad() {
    assert_technique(
        "\
        000001000\
        080030090\
        300067081\
        104020700\
        060014039\
        000000050\
        800100000\
        020005000\
        403090000",
        Technique::NakedQuad,
    );
}

#[test]
fn locked_candidates() {
    assert_technique(
        "\
        000002005\
        000901320\
        000040600\
        005004701\
        060200940\
        000000000\
        300000000\
        091600003\
        000085000",
        Technique::LockedCandidates,
    );
}

#[test]
fn x_wing() {
    assert_technique(
        "\
        007290410\
        000000000\
        004080029\
        000860000\
        940000300\
        800042000\
        030000040\
        010000800\
        000409073",
        Technique::XWing,
    );
}

#[test]
fn swordfish() {
    assert_technique(
        "\
        000090300\
        090000005\
        076800002\
        000000008\
        063050700\
        900020000\
        000289600\
        000000004\
        010500089",
        Technique::Swordfish,
    );
}