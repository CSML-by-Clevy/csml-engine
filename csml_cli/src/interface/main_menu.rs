use std::{error::Error, io::Stdout};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};

use crate::interface::{menu_widget::*, AppMain, AppState};

#[derive(Debug, Clone)]
pub struct MainMenu {
    pub options: Vec<MenuItem>,
}

impl MenuType for MainMenu {
    fn init() -> Box<dyn MenuType> {
        let main = MainMenu {
            options: vec![
                MenuItem::new(
                    "",
                    MenuComponent::Button{text: "csml init -- setup a new bot project".to_owned(), payload: None},
                    0,
                ),
                MenuItem::new(
                    "",
                    MenuComponent::Button{text: "csml run -- execute your bot script".to_owned(), payload: None},
                    0,
                ),
            ],
        };

        Box::new(main)
    }

    fn generate_menu(&mut self, _menu: &Vec<MenuItem>) -> Vec<MenuItem> {
        self.options.clone()
    }

    fn get_default_state() -> AppState {
        AppState::MainMenu
    }
}

pub fn draw_main(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut AppMain,
) -> Result<(), Box<dyn Error>> {
    // Draw Main UI
    terminal.draw(|f| {
        let size = f.size();

        // ######### Image
        // let chunks = Layout::default()
        // .constraints([Constraint::Percentage(100)].as_ref())
        // .split(
        //     Rect {
        //         x: 50,
        //         y: 0,
        //         width: 45,
        //         height: 30,
        //     }
        // );

        // let canvas = Canvas::default()
        // .block(
        //     Block::default()
        //     .borders(Borders::NONE),
        // )
        // .x_bounds([0.0, (app.logo.data.width() - 1) as f64])
        // .y_bounds([0.0, (app.logo.data.height() - 1) as f64])
        // .paint(|ctx| {
        //     for color in app.logo.color_data.keys() {
        //         if let Some(points) = app.logo.color_data.get(&color) {
        //             ctx.draw(
        //                 &Points {
        //                     coords: points,
        //                     color: Color::Rgb(color.0, color.1, color.2),
        //                 }
        //             )
        //         }
        //     }
        // });

        // f.render_widget(canvas, chunks[0]);

        // ######### Option menu

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3)].as_ref())
            .split(Rect {
                x: size.width / 4,
                y: size.height / 2,
                width: (size.width as f32 / 1.85) as u16,
                height: 6, //size.height / 3,
            });

        let menu = Menu::new(app.menu_state.menu.clone(), app.input.as_ref())
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(menu, chunks[0], &mut app.menu_state);
    })?;

    Ok(())
}
