use color_eyre::eyre::Result;
use ratatui::{DefaultTerminal, widgets::TableState};

use crate::{board::init_board, ui::render};

mod board;
mod keys;
mod solver;
mod ui;

#[derive(Default, Debug)]
pub struct AppState {
    board: [[u8; 9]; 9],
    sol: [[u8; 9]; 9],
    sol_state: SolState,
    cursor_loc: BoardLoc,
    table_state: TableState,
    select_state: bool,
    select_input_state: bool,
    select_input_val: String,
    select_vec: Vec<(u8, u8)>,
    select_store: Vec<(Vec<(u8, u8)>, u8)>,
    highlighted_set: Vec<(u8, u8)>,
}

#[derive(Default, Debug, PartialEq)]
pub enum SolState {
    FOUND,
    NOTFOUND,
    #[default]
    NULL,
}

#[derive(Default, Debug)]
struct BoardLoc {
    x: u8,
    y: u8,
}

fn main() -> Result<()> {
    let mut state = AppState {
        board: init_board(),
        sol: init_board(),
        ..Default::default()
    };

    let term = ratatui::init();

    state.table_state.select(Some(0_usize));
    state.table_state.select_column(Some(0_usize));

    let res = run(term, &mut state);

    ratatui::restore();

    res
}

fn run(mut term: DefaultTerminal, state: &mut AppState) -> Result<()> {
    loop {
        // rendering
        term.draw(|f| render(f, state))?;
        // input handling
        if keys::handler(state) {
            break;
        };
    }
    Ok(())
}
