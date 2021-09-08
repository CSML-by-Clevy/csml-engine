// pub mod ui_image;s
// use ui_image::ImageData;

pub mod chat_menu;
pub mod init;
pub mod main_menu;
pub mod select_bot;

pub mod chat_widget;
pub mod menu_widget;

use std::{
    error::Error,
    fs::File,
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use tui::{backend::CrosstermBackend, Terminal};

use csml_engine::data::{BotOpt, CsmlRequest};

use chat_menu::{draw_run, run_conversation};
use chat_widget::{RawMessage, StatefulList};
use init::draw_init;
use main_menu::{draw_main, MainMenu};
use menu_widget::*;
use select_bot::SelectBotMenu;



use self::select_bot::draw_select_bot;
use crate::init_package::init_with_env;
use crate::run::load_info;

fn load_metadata(directory_name: &str) -> Option<serde_json::Value> {
    let metadata_file = File::open(&format!("{}/metadata.yaml", directory_name)).ok()?;

    let metadata: serde_json::Value = serde_yaml::from_reader(metadata_file).ok()?;

    Some(metadata)
}

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AppState {
    // main
    MainMenu,
    // run
    RunEditing,
    RunNormal,
    // init && // run menu
    Normal,
    Editing,
    Selecting, // run menu
}

pub enum AppMode {
    Main(AppMain),
    Init(AppInit),
    SelectBot(AppSelectBot),
    Run(AppRun),
}

impl AppMode {
    fn init(path: &str) -> AppMode {
        AppMode::Main(AppMain::init(path))
    }
}

impl AppMode {
    pub fn get_state(&self) -> AppState {
        match &self {
            AppMode::Main(main) => main.menu_state.state.clone(),
            AppMode::Init(init) => init.menu_state.state.clone(),
            AppMode::SelectBot(select_bot_app) => select_bot_app.menu_state.state.clone(),
            AppMode::Run(run) => run.input_mode.clone(),
        }
    }
}

pub struct AppMain {
    /// Current value of the input box
    input: String,
    /// Current menu state
    menu_state: MenuState,
    // /// Logo data
    // logo: ImageData,
}

impl AppMain {
    fn init(_path: &str) -> AppMain {
        AppMain {
            input: String::new(),
            menu_state: MenuState::init::<MainMenu>(),
            // logo: ImageData::get_data(&format!("{}/assets/csml-logo.png", path))
        }
    }
}

pub struct AppInit {
    /// Current value of the input box
    input: String,
    /// Current menu state
    menu_state: MenuState,
}

impl Default for AppInit {
    fn default() -> AppInit {
        AppInit {
            input: String::new(),
            menu_state: MenuState::init::<InitMenu>(),
        }
    }
}

pub struct AppSelectBot {
    /// Current value of the input box
    input: String,
    /// Current menu state
    menu_state: MenuState,
}

impl Default for AppSelectBot {
    fn default() -> AppSelectBot {
        AppSelectBot {
            input: String::new(),
            menu_state: MenuState::init::<SelectBotMenu>(),
        }
    }
}

pub struct AppRun {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: AppState,
    /// History of recorded messages
    messages: StatefulList,

    scroll_x: u16,
    scroll_y: u16,
}

impl Default for AppRun {
    fn default() -> AppRun {
        AppRun {
            input: String::new(),
            input_mode: AppState::RunEditing,
            messages: StatefulList::new(),
            scroll_y: 0,
            scroll_x: 0,
        }
    }
}

pub enum StartUI {
    Main,
    Init,
    Run {
        bot_opt: Option<BotOpt>,
        request: Option<CsmlRequest>,
    },
}

pub fn csml_ui(start_ui: StartUI) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    // Terminal initialization
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create default app state
    let mut directory_name = ".".to_owned();
    // load_info
    let mut request = None;
    let mut bot_opt = None;
    let mut metadata = None;

    let mut app = match start_ui {
        StartUI::Main => AppMode::init(&directory_name),
        StartUI::Init => AppMode::Init(AppInit::default()),
        StartUI::Run {
            bot_opt: o,
            request: r,
        } => {
            request = r;
            bot_opt = o;
            AppMode::Run(AppRun::default())
        }
    };

    terminal.clear()?;

    // Setup event handlers
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let Some(CEvent::Key(key)) = event::read().ok(){
                    tx.send(Event::Input(key.code)).ok();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).ok();
                last_tick = Instant::now();
            }
        }
    });

    let mut exit;
    loop {
        match &mut app {
            AppMode::Main(main_app) => {
                draw_main(&mut terminal, main_app)?;
            }
            AppMode::Init(init_app) => {
                draw_init(&mut terminal, init_app)?;
            }
            AppMode::SelectBot(select_bot_app) => {
                draw_select_bot(&mut terminal, select_bot_app)?;
            }
            AppMode::Run(run_app) => {
                draw_run(&mut terminal, run_app)?;
            }
        };

        // Handle input
        if let Event::Input(input) = rx.recv()? {
            let state = app.get_state();

            exit = match &mut app {
                AppMode::Main(main) => match state {
                    AppState::MainMenu => handle_main_normal_mode(input, main)?,
                    _ => Exit::None,
                },
                AppMode::Init(init) => match state {
                    AppState::Selecting => handle_select_mode(input, init)?,
                    AppState::Editing => handle_editing_mode(input, init)?,
                    AppState::Normal => handle_normal_mode(
                        input,
                        init,
                        &mut bot_opt,
                        &mut metadata,
                        &mut directory_name,
                    )?,
                    _ => Exit::None,
                },
                AppMode::SelectBot(select_bot_app) => match state {
                    AppState::Selecting => handle_select_bot_mode(
                        input,
                        select_bot_app,
                        &mut bot_opt,
                        &mut metadata,
                        &mut directory_name,
                    )?,
                    _ => Exit::None,
                },
                AppMode::Run(run) => match state {
                    AppState::RunNormal => handle_run_normal_mode(
                        input,
                        run,
                        &mut bot_opt,
                        &mut metadata,
                        &directory_name,
                    )?,
                    AppState::RunEditing => {
                        handle_run_editing_mode(input, run, &bot_opt, &mut metadata, request)?
                    }
                    _ => Exit::None,
                },
            };

            match exit {
                Exit::Exit => {
                    disable_raw_mode()?;
                    terminal.hide_cursor()?;
                    terminal.flush()?;
                    terminal.show_cursor()?;
                    break;
                }
                Exit::ChangeAPP(new_app) => {
                    app = new_app;
                }
                Exit::None => {}
            };
        }

        request = None;
    }

    Ok(())
}

// ################################# HANDLERS
enum Exit {
    None,
    Exit,
    ChangeAPP(AppMode),
}

//############################# init menu

fn handle_normal_mode<'a>(
    input: KeyCode,
    app: &mut AppInit,
    bot_opt: &mut Option<BotOpt>,
    metadata: &mut Option<serde_json::Value>,
    directory_name: &mut String,
) -> Result<Exit, Box<dyn Error>> {
    match input {
        KeyCode::Enter => {
            if let Some(index) = app.menu_state.selected {
                match &app.menu_state.menu[index].element {
                    MenuElement::List { .. } => app.menu_state.state = AppState::Selecting,
                    MenuElement::Text(_text) => app.menu_state.state = AppState::Editing,
                    MenuElement::Button(_accept) => {
                        let env = app.menu_state.gen_env();
                        let bot_name = env.bot_name.clone();

                        init_with_env(env)?;
                        *directory_name = bot_name.clone();

                        *bot_opt = Some(BotOpt::CsmlBot(load_info(&bot_name).unwrap()));
                        *metadata = load_metadata(directory_name);

                        //load env
                        dotenv::from_path(&format!("{}/.env", directory_name)).ok();

                        app.menu_state.state = AppState::RunEditing;

                        return Ok(Exit::ChangeAPP(AppMode::Run(AppRun::default())));
                    }
                    MenuElement::SelectableBot { .. } => {}
                };
            }
            Ok(Exit::None)
        }
        KeyCode::Down => {
            app.menu_state.next();
            Ok(Exit::None)
        }
        KeyCode::Up => {
            app.menu_state.previous();
            Ok(Exit::None)
        }
        KeyCode::Esc => Ok(Exit::Exit),
        _ => Ok(Exit::None),
    }
}

fn handle_editing_mode(input: KeyCode, app: &mut AppInit) -> Result<Exit, Box<dyn Error>> {
    match input {
        KeyCode::Enter => {
            if let Some(index) = app.menu_state.selected {
                let item = &mut app.menu_state.menu[index];
                let line: String = app.input.drain(..).collect();
                if line.trim().is_empty() {
                    return Ok(Exit::None);
                }

                item.update_value(&line);
                app.menu_state.state = AppState::Normal;
            }

            app.menu_state.state = AppState::Normal;
            Ok(Exit::None)
        }
        KeyCode::Char(c) => {
            app.input.push(c);
            Ok(Exit::None)
        }
        KeyCode::Backspace => {
            app.input.pop();
            Ok(Exit::None)
        }
        KeyCode::Esc => {
            // clear the current input
            app.input.clear();

            app.menu_state.state = AppState::Normal;
            Ok(Exit::None)
        }
        _ => Ok(Exit::None),
    }
}

fn handle_select_mode(input: KeyCode, app: &mut AppInit) -> Result<Exit, Box<dyn Error>> {
    match input {
        KeyCode::Enter => {
            if let Some(index) = app.menu_state.selected {
                let item = &mut app.menu_state.menu[index];
                item.update_value("");

                // check if input item is de same item
                if "mongodb" == item.get_value() || "dynamodb" == item.get_value() {
                    app.menu_state.change_list();
                }

                app.menu_state.state = AppState::Normal;
            }
            Ok(Exit::None)
        }
        KeyCode::Down => {
            if let Some(index) = app.menu_state.selected {
                let item = &mut app.menu_state.menu[index];

                item.next()
            }
            Ok(Exit::None)
        }
        KeyCode::Up => {
            if let Some(index) = app.menu_state.selected {
                let item = &mut app.menu_state.menu[index];

                item.previous()
            }
            Ok(Exit::None)
        }
        KeyCode::Esc => Ok(Exit::Exit),
        _ => Ok(Exit::None),
    }
}

//############################# run menu

fn handle_run_normal_mode(
    input: KeyCode,
    app: &mut AppRun,
    bot_opt: &mut Option<BotOpt>,
    metadata: &mut Option<serde_json::Value>,
    directory_name: &str,
) -> Result<Exit, Box<dyn Error>> {
    match input {
        KeyCode::Char('e') => {
            app.input_mode = AppState::RunEditing;

            return Ok(Exit::None);
        }
        KeyCode::Char('r') => {
            *bot_opt = Some(BotOpt::CsmlBot(load_info(directory_name).unwrap()));
            *metadata = load_metadata(directory_name);

            app.messages.items.push_front(
                RawMessage {
                    sender: "Reload Bot".to_owned(),
                    raw_messages: vec![serde_json::json!(
                        {
                            "content_type": "text",
                            "content": {
                                "text": ""
                            }
                        }
                    )],
                }
            );

            return Ok(Exit::None);
        }
        KeyCode::Char('q') => return Ok(Exit::Exit),
        KeyCode::Left => {
            if app.scroll_x > 0 {
                app.scroll_x -= 1;
            }
            return Ok(Exit::None);
        }
        KeyCode::Right => {
            app.scroll_x += 1;
            return Ok(Exit::None);
        }
        KeyCode::Down => {
            app.scroll_y += 1;
            return Ok(Exit::None);
        }
        KeyCode::Up => {
            if app.scroll_y > 0 {
                app.scroll_y -= 1;
            }
            return Ok(Exit::None);
        }
        _ => return Ok(Exit::None),
    }
}

fn handle_run_editing_mode(
    input: KeyCode,
    app: &mut AppRun,
    bot_opt: &Option<BotOpt>,
    metadata: &mut Option<serde_json::Value>,
    request: Option<CsmlRequest>,
) -> Result<Exit, Box<dyn Error>> {
    match input {
        KeyCode::Enter => {
            let mut user_input: String = app.input.drain(..).collect();
            let raw_msg = if user_input.trim().is_empty() {
                match app.messages.sub_item.selected {
                    true => {
                        user_input = app.messages.sub_item.payload.clone();

                        let msg = RawMessage {
                            sender: "user".to_owned(),
                            raw_messages: vec![serde_json::json!(
                                {
                                    "content_type": "text",
                                    "content": {
                                        "text": &user_input
                                    }
                                }
                            )],
                        };
                        // clear selected sub item container
                        app.messages.sub_item.reset();

                        msg
                    }
                    false => {
                        return Ok(Exit::None);
                    }
                }
            } else {
                RawMessage {
                    sender: "user".to_owned(),
                    raw_messages: vec![serde_json::json!(
                        {
                            "content_type": "text",
                            "content": {
                                "text": &user_input
                            }
                        }
                    )],
                }
            };

            app.messages.items.push_front(raw_msg);

            if let Some(run_opt) = bot_opt {
                run_conversation(&user_input, request, run_opt.clone(), metadata.clone(), app)?;
            }
        }
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        KeyCode::Left => {
            app.messages.left();
        }
        KeyCode::Right => {
            app.messages.right();
        }
        KeyCode::Down => {
            // clear selected sub item container
            app.messages.sub_item.reset();

            app.scroll_x = 0;
            app.scroll_y = 0;
            app.messages.next();
        }
        KeyCode::Up => {
            // clear selected sub item container
            app.messages.sub_item.reset();

            app.scroll_x = 0;
            app.scroll_y = 0;
            app.messages.previous();
        }
        KeyCode::Backspace => {
            app.input.pop();
            app.messages.unselect();
        }
        KeyCode::Esc => {
            app.input_mode = AppState::RunNormal;
        }
        _ => {}
    };

    return Ok(Exit::None);
}

//############################# main

fn handle_main_normal_mode<'a>(input: KeyCode, app: &mut AppMain) -> Result<Exit, Box<dyn Error>> {
    match input {
        KeyCode::Enter => {
            if let Some(index) = app.menu_state.selected {
                match &app.menu_state.menu[index].element {
                    MenuElement::Button(button) => match button.as_str() {
                        "csml init -- setup a new bot project" => {
                            app.menu_state.state = AppState::Normal;

                            return Ok(Exit::ChangeAPP(AppMode::Init(AppInit::default())));
                        }
                        "csml run -- execute your bot script" => {
                            app.menu_state.state = AppState::Selecting;

                            return Ok(Exit::ChangeAPP(
                                AppMode::SelectBot(AppSelectBot::default()),
                            ));
                        }
                        _run => {}
                    },
                    _ => {}
                };
            }
            Ok(Exit::None)
        }
        KeyCode::Down => {
            app.menu_state.next();
            Ok(Exit::None)
        }
        KeyCode::Up => {
            app.menu_state.previous();
            Ok(Exit::None)
        }
        KeyCode::Esc => Ok(Exit::Exit),
        _ => Ok(Exit::None),
    }
}

//############################# select bot to run

fn handle_select_bot_mode<'a>(
    input: KeyCode,
    app: &mut AppSelectBot,
    bot_opt: &mut Option<BotOpt>,
    metadata: &mut Option<serde_json::Value>,
    directory_name: &mut String,
) -> Result<Exit, Box<dyn Error>> {
    match input {
        KeyCode::Enter => {
            if let Some(index) = app.menu_state.selected {
                match &app.menu_state.menu[index].element {
                    MenuElement::SelectableBot { bot, path_info, .. } => {
                        *directory_name = path_info.clone();

                        //load env
                        dotenv::from_path(&format!("{}/.env", directory_name)).ok();

                        *bot_opt = Some(BotOpt::CsmlBot(bot.to_owned()));
                        *metadata = load_metadata(directory_name);

                        app.menu_state.state = AppState::RunEditing;

                        return Ok(Exit::ChangeAPP(AppMode::Run(AppRun::default())));
                    }
                    MenuElement::Button(button) => match button.as_str() {
                        "[create new bot]" => {
                            app.menu_state.state = AppState::Normal;

                            return Ok(Exit::ChangeAPP(AppMode::Init(AppInit::default())));
                        }
                        _run => {}
                    },
                    _ => {}
                };
            }
            Ok(Exit::None)
        }
        KeyCode::Down => {
            app.menu_state.next();
            Ok(Exit::None)
        }
        KeyCode::Up => {
            app.menu_state.previous();
            Ok(Exit::None)
        }
        KeyCode::Esc => Ok(Exit::Exit),
        _ => Ok(Exit::None),
    }
}
