use std::{error::Error, io::Stdout};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};

use crate::interface::{menu_widget::*, AppInit};

pub fn draw_init(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut AppInit,
) -> Result<(), Box<dyn Error>> {
    // Draw Init UI
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(3)].as_ref())
            .split(
                Rect {
                    x: 40,
                    y: 0,
                    width: 100,
                    height: 30,
                }, // f.size()
            );

        // app.menu_state.

        let menu = Menu::new(app.menu_state.menu.clone(), app.input.as_ref())
            .block(Block::default().borders(Borders::ALL).title("Init"))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(menu, chunks[0], &mut app.menu_state);

        // f.set_cursor(
        //     // Put cursor past the end of the input text
        //     chunks[0].x + app.input.width() as u16 + 1 + 38,
        //     // Move one line down, from the border to the input line
        //     chunks[0].y + 1,
        // );
    })?;

    Ok(())
}
