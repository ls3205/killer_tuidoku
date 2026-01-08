use crate::{AppState, SolState};

pub fn solve(state: &mut AppState) -> [[u8; 9]; 9] {
    let mut table = state.board;

    if solve_sudoku(&mut table) {
        state.sol_state = SolState::FOUND;
    } else {
        state.sol_state = SolState::NOTFOUND;
    }

    if state.sol_state == SolState::FOUND {
        table
    } else {
        [[0; 9]; 9]
    }
}

fn solve_sudoku(table: &mut [[u8; 9]; 9]) -> bool {
    if !table.as_flattened().contains(&0) {
        return true;
    }

    let (row, col) = find_empty(table);

    for n in 1..=9 {
        if is_valid(table, row, col, n) {
            table[row][col] = n;

            if solve_sudoku(table) {
                return true;
            }

            table[row][col] = 0;
        }
    }

    false
}

fn find_empty(table: &[[u8; 9]; 9]) -> (usize, usize) {
    for (y, row) in table.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val == 0 {
                return (y, x);
            }
        }
    }

    (0, 0)
}

fn is_valid(table: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    if table[row].contains(&num) {
        return false;
    }

    if table.iter().any(|row| row[col] == num) {
        return false;
    }

    let (start_row, start_col) = ((row / 3) * 3, (col / 3) * 3);

    if table[start_row..=start_row + 2]
        .iter()
        .flat_map(|row| &row[start_col..=start_col + 2])
        .any(|&val| val == num)
    {
        return false;
    }

    true
}
