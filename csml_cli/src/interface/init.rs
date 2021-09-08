use std::{error::Error, io::Stdout};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
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
            .split(f.size());

        let menu = Menu::new(app.menu_state.menu.clone(), app.input.as_ref())
            .block(Block::default().borders(Borders::ALL).title("Init"))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(menu, chunks[0], &mut app.menu_state);
    })?;

    Ok(())
}
