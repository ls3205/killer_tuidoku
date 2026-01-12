use crate::{AppState, SolState};

pub fn solve(state: &mut AppState) -> [[u8; 9]; 9] {
    let mut table = state.board;

    if solve_sudoku(&mut table, &state.select_store) {
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

fn solve_sudoku(table: &mut [[u8; 9]; 9], cages: &Vec<(Vec<(u8, u8)>, u8)>) -> bool {
    if !table.as_flattened().contains(&0) {
        return true;
    }

    let (row, col) = find_empty(table);

    for n in 1..=9 {
        if is_valid(table, cages, row, col, n) {
            table[row][col] = n;

            if solve_sudoku(table, cages) {
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

fn is_valid(
    table: &[[u8; 9]; 9],
    cages: &Vec<(Vec<(u8, u8)>, u8)>,
    row: usize,
    col: usize,
    num: u8,
) -> bool {
    if let Some(group) = cages
        .iter()
        .find(|cage| cage.0.contains(&(row as u8, col as u8)))
    {
        let total: u8 = group
            .0
            .iter()
            .map(|(y, x)| table[*y as usize][*x as usize])
            .sum();

        if total + num > group.1 {
            return false;
        }
    }

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
