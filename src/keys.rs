use ratatui::crossterm::event::{self, Event, KeyEvent};

use crate::AppState;
use crate::SolState;
use crate::solver;

fn update_highlight(state: &mut AppState) {
    let sel_cell = state.table_state.selected_cell().unwrap();

    if let Some(active_cage) = state
        .select_store
        .iter()
        .find(|cage| cage.0.contains(&(sel_cell.0 as u8, sel_cell.1 as u8)))
    {
        state.highlighted_set = active_cage.clone();
    } else {
        state.highlighted_set.0.clear();
        state.highlighted_set.1 = 0;
    }
}

pub fn handler(state: &mut AppState) -> bool {
    if let Ok(Event::Key(k)) = event::read() {
        if state.select_state {
            if state.select_input_state {
                return handle_select_input(k, state);
            } else {
                return handle_select(k, state);
            }
        }

        if state.confirm_delete {
            return handle_delete(k, state);
        }

        return handle_default(k, state);
    }

    false
}

fn handle_default(k: KeyEvent, state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Esc => return true,
        event::KeyCode::Enter => {
            state.sol_state = SolState::LOADING;
            state.sol = solver::solve(state);
        }
        event::KeyCode::Char(c) if ('1'..='9').contains(&c) => {
            if !(state.sol_state == SolState::NULL) {
                state.sol_state = SolState::NULL;
                state.sol = [[0; 9]; 9];
            }

            state.board[state.cursor_loc.y as usize][state.cursor_loc.x as usize] =
                c.to_digit(10).unwrap() as u8;
        }
        event::KeyCode::Char(c) => match c {
            'j' => {
                state.cursor_loc.y = if state.cursor_loc.y < 8 {
                    state.cursor_loc.y + 1
                } else {
                    state.cursor_loc.y
                };

                state.table_state.select(Some(state.cursor_loc.y as usize));
                update_highlight(state);
            }
            'k' => {
                state.cursor_loc.y = if state.cursor_loc.y > 0 {
                    state.cursor_loc.y - 1
                } else {
                    state.cursor_loc.y
                };

                state.table_state.select(Some(state.cursor_loc.y as usize));
                update_highlight(state);
            }
            'h' => {
                state.cursor_loc.x = if state.cursor_loc.x > 0 {
                    state.cursor_loc.x - 1
                } else {
                    state.cursor_loc.x
                };

                state
                    .table_state
                    .select_column(Some(state.cursor_loc.x as usize));
                update_highlight(state);
            }
            'l' => {
                state.cursor_loc.x = if state.cursor_loc.x < 8 {
                    state.cursor_loc.x + 1
                } else {
                    state.cursor_loc.x
                };

                state
                    .table_state
                    .select_column(Some(state.cursor_loc.x as usize));
                update_highlight(state);
            }
            'x' => {
                state.board[state.cursor_loc.y as usize][state.cursor_loc.x as usize] = 0;
            }
            'v' => {
                if let Some((row, col)) = state.table_state.selected_cell()
                    && !state
                        .select_store
                        .iter()
                        .any(|cage| cage.0.contains(&(row as u8, col as u8)))
                {
                    state.select_state = true;
                    state.select_vec.push((row as u8, col as u8));
                }
            }
            'd' => {
                if state.select_store.contains(&state.highlighted_set) {
                    state.confirm_delete = true;
                }
            }
            'e' => {
                if let Some(idx) = state
                    .select_store
                    .iter()
                    .position(|cage| *cage == state.highlighted_set)
                {
                    let copy = state.select_store[idx].clone();

                    state.select_vec = copy.0.clone();
                    state.select_input_val = copy.1.clone().to_string();
                    state.select_state = true;

                    state.is_editing = true;
                    state.editing_clone = copy;

                    state.select_store.remove(idx);
                    update_highlight(state);
                }
            }
            _ => {}
        },
        _ => {}
    }

    false
}

fn handle_select(k: KeyEvent, state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Esc => {
            if state.is_editing {
                state.select_state = false;
                state.is_editing = false;

                state.select_vec.clear();
                state.select_input_val.clear();

                state.select_store.push(state.editing_clone.clone());
                state.editing_clone.0.clear();
                state.editing_clone.1 = 0;

                update_highlight(state);
            } else {
                state.select_state = false;
                state.select_vec.clear();
            }
        }
        event::KeyCode::Enter => {
            state.select_input_state = true;
        }
        event::KeyCode::Char(c) => match c {
            'j' => {
                state.cursor_loc.y = if state.cursor_loc.y < 8 {
                    state.cursor_loc.y + 1
                } else {
                    state.cursor_loc.y
                };

                state.table_state.select(Some(state.cursor_loc.y as usize));

                if let Some((row, col)) = state.table_state.selected_cell() {
                    let cell = (row as u8, col as u8);
                    if state.select_store.iter().any(|cage| cage.0.contains(&cell)) {
                        state.cursor_loc.y -= 1;
                        state.table_state.select(Some(state.cursor_loc.y as usize));
                    } else if !state.select_vec.contains(&cell) {
                        state.select_vec.push(cell);
                    }
                }
            }
            'k' => {
                state.cursor_loc.y = if state.cursor_loc.y > 0 {
                    state.cursor_loc.y - 1
                } else {
                    state.cursor_loc.y
                };

                state.table_state.select(Some(state.cursor_loc.y as usize));

                if let Some((row, col)) = state.table_state.selected_cell() {
                    let cell = (row as u8, col as u8);
                    if state.select_store.iter().any(|cage| cage.0.contains(&cell)) {
                        state.cursor_loc.y += 1;
                        state.table_state.select(Some(state.cursor_loc.y as usize));
                    } else if !state.select_vec.contains(&cell) {
                        state.select_vec.push(cell);
                    }
                }
            }
            'h' => {
                state.cursor_loc.x = if state.cursor_loc.x > 0 {
                    state.cursor_loc.x - 1
                } else {
                    state.cursor_loc.x
                };

                state
                    .table_state
                    .select_column(Some(state.cursor_loc.x as usize));

                if let Some((row, col)) = state.table_state.selected_cell() {
                    let cell = (row as u8, col as u8);
                    if state.select_store.iter().any(|cage| cage.0.contains(&cell)) {
                        state.cursor_loc.x += 1;
                        state
                            .table_state
                            .select_column(Some(state.cursor_loc.x as usize));
                    } else if !state.select_vec.contains(&cell) {
                        state.select_vec.push(cell);
                    }
                }
            }
            'l' => {
                state.cursor_loc.x = if state.cursor_loc.x < 8 {
                    state.cursor_loc.x + 1
                } else {
                    state.cursor_loc.x
                };

                state
                    .table_state
                    .select_column(Some(state.cursor_loc.x as usize));

                if let Some((row, col)) = state.table_state.selected_cell() {
                    let cell = (row as u8, col as u8);
                    if state.select_store.iter().any(|cage| cage.0.contains(&cell)) {
                        state.cursor_loc.x -= 1;
                        state
                            .table_state
                            .select_column(Some(state.cursor_loc.x as usize));
                    } else if !state.select_vec.contains(&cell) {
                        state.select_vec.push(cell);
                    }
                }
            }
            'd' => {
                if let Some((row, col)) = state.table_state.selected_cell() {
                    let cell = (row as u8, col as u8);
                    if let Some(idx) = state
                        .select_vec
                        .iter()
                        .position(|cell_store| *cell_store == cell)
                    {
                        if cell.0 > 0 && state.select_vec.contains(&(cell.0 - 1, cell.1)) {
                            state.select_vec.remove(idx);

                            state.cursor_loc.y -= 1;
                            state.table_state.select_cell(Some((
                                state.cursor_loc.y as usize,
                                state.cursor_loc.x as usize,
                            )));
                        } else if cell.1 > 0 && state.select_vec.contains(&(cell.0, cell.1 - 1)) {
                            state.select_vec.remove(idx);

                            state.cursor_loc.x -= 1;
                            state.table_state.select_cell(Some((
                                state.cursor_loc.y as usize,
                                state.cursor_loc.x as usize,
                            )));
                        } else if cell.0 < 8 && state.select_vec.contains(&(cell.0 + 1, cell.1)) {
                            state.select_vec.remove(idx);

                            state.cursor_loc.y += 1;
                            state.table_state.select_cell(Some((
                                state.cursor_loc.y as usize,
                                state.cursor_loc.x as usize,
                            )));
                        } else if cell.1 < 8 && state.select_vec.contains(&(cell.0, cell.1 + 1)) {
                            state.select_vec.remove(idx);

                            state.cursor_loc.x += 1;
                            state.table_state.select_cell(Some((
                                state.cursor_loc.y as usize,
                                state.cursor_loc.x as usize,
                            )));
                        }
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }

    false
}

fn handle_select_input(k: KeyEvent, state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Esc => {
            state.select_input_state = false;
            state.select_input_val.clear();
        }
        event::KeyCode::Enter => {
            if let Ok(n) = state.select_input_val.parse::<u8>()
                && n > 0
            {
                state.select_input_state = false;
                state.select_state = false;

                state.select_store.push((state.select_vec.clone(), n));
                state.highlighted_set = (state.select_vec.clone(), n);
                state.select_vec.clear();
                state.select_input_val.clear();
            }
        }
        event::KeyCode::Backspace => {
            state.select_input_val.pop();
        }
        event::KeyCode::Char(c) if c.is_ascii_digit() => {
            state.select_input_val.push(c);
        }
        _ => {}
    }

    false
}

fn handle_delete(k: KeyEvent, state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Esc => {
            state.confirm_delete = false;
        }
        event::KeyCode::Enter => {
            if let Some(idx) = state
                .select_store
                .iter()
                .position(|cage| *cage == state.highlighted_set)
            {
                state.select_store.remove(idx);
                state.confirm_delete = false;
                update_highlight(state);
            }
        }
        _ => {}
    }

    false
}
