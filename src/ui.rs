use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    prelude::Widget,
    style::{Color, Style, Stylize},
    text::{Span, ToSpan},
    widgets::{Block, Padding, Paragraph, Row, Table},
};

use crate::{AppState, SolState};

fn keybindinator(
    binds: Vec<(String, String)>,
    c1: Color,
    c2: Color,
) -> ratatui::prelude::Line<'static> {
    let mut out = ratatui::prelude::Line::from(vec![" ".to_span()]);

    for (k, v) in binds {
        out.push_span(Span::from(k).fg(c1));
        out.push_span(" ".to_span());
        out.push_span(Span::from(v).fg(c2));
        out.push_span(" ".to_span());
    }

    out
}

pub fn render(frame: &mut Frame, state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(
            " killer_tuidoku "
                .to_span()
                .into_centered_line()
                .fg(Color::White),
        )
        .title_bottom(
            keybindinator(
                vec![
                    (String::from("Up"), String::from("[k]")),
                    (String::from("Down"), String::from("[j]")),
                    (String::from("Left"), String::from("[h]")),
                    (String::from("Right"), String::from("[l]")),
                    (String::from("Clear Cell"), String::from("[x]")),
                    (String::from("Write Cell"), String::from("[1-9]")),
                    (String::from("Select Mode"), String::from("[v]")),
                    (String::from("Edit Cage"), String::from("[e]")),
                    (String::from("Delete Cage"), String::from("[d]")),
                    (String::from("Solve"), String::from("[Enter]")),
                ],
                Color::Green,
                Color::Yellow,
            )
            .alignment(ratatui::layout::HorizontalAlignment::Center),
        )
        .fg(Color::default())
        .render(border_area, frame.buffer_mut());

    render_board(frame, state);

    if state.select_input_state {
        render_cage_input(frame, state);
    }

    if state.confirm_delete {
        render_confirm_delete(frame);
    }
}

fn render_board(frame: &mut Frame, state: &mut AppState) {
    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(6)
        .split(frame.area());

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::HeavyDoubleDashed)
        .fg(if state.select_state {
            Color::Blue
        } else {
            Color::Green
        })
        .title(
            (if state.select_state {
                if state.is_editing {
                    " Editing Cage "
                } else {
                    " Selecting Cage "
                }
            } else {
                " Input "
            })
            .to_span()
            .into_centered_line(),
        )
        .title_bottom(if state.highlighted_set.1 > 0 {
            (" Cage Total: ".to_span().fg(Color::Green)
                + state.highlighted_set.1.to_span().fg(Color::default())
                + " ".to_span())
            .alignment(ratatui::layout::HorizontalAlignment::Center)
        } else {
            "".to_span().into_centered_line()
        })
        .render(area[0], frame.buffer_mut());

    // i swear to you the hardest part about learning a new frontend tool is figuring out how to
    // style anything... say what you want about tailwind but i would take tailwind 10 million
    // times over this

    let board_area = area[0].centered(Constraint::Length(41), Constraint::Length(25));
    let board_lines_outer = area[0].centered(Constraint::Length(45), Constraint::Length(27));

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::default())
        .render(board_lines_outer, frame.buffer_mut());

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::default())
        .render(
            board_lines_outer.centered(Constraint::Percentage(100), Constraint::Length(9)),
            frame.buffer_mut(),
        );

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::default())
        .render(
            board_lines_outer.centered(Constraint::Length(15), Constraint::Percentage(100)),
            frame.buffer_mut(),
        );

    let table = Table::new(
        state.board.iter().enumerate().map(|(y, row)| {
            Row::new(
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| {
                        let content = if *cell == 0 {
                            "-".to_string()
                        } else {
                            cell.to_string()
                        };
                        content.bg(if state.select_vec.contains(&(y as u8, x as u8)) {
                            Color::Blue
                        } else if state.highlighted_set.0.contains(&(y as u8, x as u8)) {
                            Color::Yellow
                        } else if state.select_state
                            && state
                                .select_store
                                .iter()
                                .any(|cage| cage.0.contains(&(y as u8, x as u8)))
                        {
                            Color::Red
                        } else {
                            Color::default()
                        })
                    })
                    .collect::<Vec<_>>(),
            )
            .bottom_margin(2)
            .fg(Color::Green)
        }),
        &[Constraint::Length(1); 9],
    )
    .column_spacing(4)
    .cell_highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan));

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::LightDoubleDashed)
        .fg(Color::default())
        .title(" Solution ".to_span().into_centered_line())
        .title_bottom(match state.sol_state {
            SolState::NULL => "".to_span().into_centered_line(),
            SolState::LOADING => " Loading (This may take a little) "
                .to_span()
                .into_centered_line()
                .fg(Color::default()),
            SolState::FOUND => " Solution Found "
                .to_span()
                .into_centered_line()
                .fg(Color::Green),
            SolState::NOTFOUND => " No Solution Found "
                .to_span()
                .into_centered_line()
                .fg(Color::Red),
        })
        .render(area[1], frame.buffer_mut());

    let sol_area = area[1].centered(Constraint::Length(41), Constraint::Length(25));
    let sol_lines_outer = area[1].centered(Constraint::Length(45), Constraint::Length(27));

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::default())
        .render(sol_lines_outer, frame.buffer_mut());

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::default())
        .render(
            sol_lines_outer.centered(Constraint::Percentage(100), Constraint::Length(9)),
            frame.buffer_mut(),
        );

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::default())
        .render(
            sol_lines_outer.centered(Constraint::Length(15), Constraint::Percentage(100)),
            frame.buffer_mut(),
        );

    let sol = Table::new(
        state.sol.iter().map(|row| {
            Row::new(
                row.iter()
                    .map(|cell| {
                        if *cell == 0 {
                            "-".to_string()
                        } else {
                            cell.to_string()
                        }
                    })
                    .collect::<Vec<_>>(),
            )
            .bottom_margin(2)
            .fg(Color::Green)
        }),
        &[Constraint::Length(1); 9],
    )
    .column_spacing(4);

    frame.render_stateful_widget(table, board_area, &mut state.table_state);
    frame.render_widget(sol, sol_area);
}

fn render_cage_input(frame: &mut Frame, state: &mut AppState) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(100)])
        .flex(Flex::Center)
        .split(frame.area())[0]
        .centered(Constraint::Length(40), Constraint::Length(5));

    Paragraph::new(state.select_input_val.clone() + "|")
        .fg(Color::default())
        .block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .fg(Color::Green)
                .padding(Padding::uniform(1))
                .title(" Cage Total ".to_span().fg(Color::default()))
                .title_bottom(
                    keybindinator(
                        vec![
                            (String::from("Cancel"), String::from("[Esc]")),
                            (String::from("Submit"), String::from("[Enter]")),
                        ],
                        Color::Green,
                        Color::Yellow,
                    )
                    .alignment(Alignment::Center),
                ),
        )
        .render(area, frame.buffer_mut());
}

fn render_confirm_delete(frame: &mut Frame) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Percentage(100)])
        .flex(Flex::Center)
        .split(frame.area())[0]
        .centered(Constraint::Length(40), Constraint::Length(5));

    Paragraph::new("Confirm Cage Deletion".to_span().into_centered_line())
        .fg(Color::default())
        .block(
            Block::bordered()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .fg(Color::Green)
                .padding(Padding::uniform(1))
                .title(" Confirm ".to_span().fg(Color::default()))
                .title_bottom(
                    keybindinator(
                        vec![
                            (String::from("Cancel"), String::from("[Esc]")),
                            (String::from("Confirm"), String::from("[Enter]")),
                        ],
                        Color::Green,
                        Color::Yellow,
                    )
                    .alignment(Alignment::Center),
                ),
        )
        .render(area, frame.buffer_mut());
}
