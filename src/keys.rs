use ratatui::crossterm::event::{self, Event, KeyEvent};

use crate::AppState;
use crate::SolState;
use crate::solver;

pub fn handler(state: &mut AppState) -> bool {
    if let Ok(Event::Key(k)) = event::read() {
        return handle_default(k, state);
    }

    false
}

fn handle_default(k: KeyEvent, state: &mut AppState) -> bool {
    match k.code {
        event::KeyCode::Esc => return true,
        event::KeyCode::Enter => {
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
            }
            'k' => {
                state.cursor_loc.y = if state.cursor_loc.y > 0 {
                    state.cursor_loc.y - 1
                } else {
                    state.cursor_loc.y
                };

                state.table_state.select(Some(state.cursor_loc.y as usize));
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
            }
            'x' => {
                state.board[state.cursor_loc.y as usize][state.cursor_loc.x as usize] = 0;
            }
            _ => {}
        },
        _ => {}
    }

    false
}
