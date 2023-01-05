use std::{error::Error, io::Stdout};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};

use crate::{
    interface::{menu_widget::*, AppState},
    run::search_csml_bot_folders,
};

use crate::interface::{
    menu_widget::{MenuComponent, MenuItem},
    AppSelectBot,
};

#[derive(Debug, Clone)]
pub struct SelectBotMenu {
    pub options: Vec<MenuItem>,
}

impl MenuType for SelectBotMenu {
    fn init() -> Box<dyn MenuType> {
        let list = search_csml_bot_folders();
        let mut options = list
            .iter()
            .map(|(path, bot)| {
                let bot_component = MenuComponent::Bot {
                    path_info: path.to_owned(),
                    bot: bot.to_owned(),
                    text: format!(" {}({})", bot.name, path),
                };
                MenuItem::new("", bot_component, 0)
            })
            .collect::<Vec<MenuItem>>();

        options.push(MenuItem::new(
            "",
            MenuComponent::Button {
                text: "[create new bot]".to_owned(),
                payload: None,
            },
            0,
        ));

        let select_bot_menu = SelectBotMenu { options };

        Box::new(select_bot_menu)
    }

    fn generate_menu(&mut self) -> Vec<MenuItem> {
        self.options.clone()
    }

    fn get_default_state() -> AppState {
        AppState::Selecting
    }
}

pub fn draw_select_bot(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut AppSelectBot,
) -> Result<(), Box<dyn Error>> {
    // Draw Main UI
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(3)].as_ref())
            .split(f.size());

        let menu = Menu::new(app.menu_state.menu.clone(), app.input.as_ref())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Select your bot"),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(menu, chunks[0], &mut app.menu_state);
    })?;

    Ok(())
}
