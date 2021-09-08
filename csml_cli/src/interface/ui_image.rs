// use crate::util::event::{Config, Event, Events};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::{
    error::Error,
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Points, Rectangle};
use tui::widgets::{Block, Borders};
use tui::Terminal;

use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgb, RgbImage};
use std::collections::HashMap;
use std::path::Path;
use viuer::{print_from_file, Config};

pub struct ImageData {
    pub data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub color_data: HashMap<(u8, u8, u8), Vec<(f64, f64)>>,
}

impl ImageData {
    pub fn get_data(path: &str) -> ImageData {
        let data = open(path);
        let color_data = group_by_color(data.clone());

        ImageData { data, color_data }
    }
}

pub fn open<P>(path: P) -> RgbImage
where
    P: AsRef<Path>,
{
    let img = image::open(path).unwrap();

    img
        // .resize(80, 80, FilterType::Nearest)
        .to_rgb8()
}

pub fn group_by_color(img: RgbImage) -> HashMap<(u8, u8, u8), Vec<(f64, f64)>> {
    let mut result = HashMap::<(u8, u8, u8), Vec<(f64, f64)>>::new();

    let (_, height) = img.dimensions();
    let height = height as i32;

    for (x, y, color) in img.enumerate_pixels() {
        let x = f64::from(x);
        let y = f64::from(height - 1 - (y as i32));

        let key = (color[0], color[1], color[2]);
        if let Some(origin_value) = result.get(&key) {
            let mut value = origin_value.clone();
            value.push((x, y));
            result.insert(key, value);
        } else {
            let mut value = Vec::<(f64, f64)>::new();
            value.push((x, y));
            result.insert(key, value);
        }
    }
    result
}

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct App {}
impl Default for App {
    fn default() -> App {
        App {}
    }
}

pub fn image() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    // Terminal initialization
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // let mut directory_name = ".".to_owned();

    let mut app = App::default();

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
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key.code)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let img = open("assets/csml-logo.png");

    let width = img.width();
    let height = img.height();
    println!("start getting colors");
    let img_data = group_by_color(img);
    println!("end getting colors");
    // let title = format!("{}x{}", width, height);

    // let conf = Config {
    //     // set offset
    //     x: 0,
    //     y: 0,
    //     // set dimensions
    //     width: Some(20),
    //     height: Some(10),
    //     ..Default::default()
    // };
    // let img2 = image::open("assets/amerelo.jpg").unwrap().resize(40, 40, FilterType::Nearest);
    // viuer::print(&img2, &conf).expect("Image printing failed.");

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(Rect {
                    x: 40,
                    y: 0,
                    width: 40,
                    height: 30,
                });

            let canvas = Canvas::default()
                .block(
                    Block::default().borders(Borders::NONE),
                    // .title(title.as_str())
                )
                .x_bounds([0.0, (width - 1) as f64])
                .y_bounds([0.0, (height - 1) as f64])
                .paint(|ctx| {
                    for color in img_data.keys() {
                        if let Some(points) = img_data.get(&color) {
                            ctx.draw(&Points {
                                coords: points,
                                color: Color::Rgb(color.0, color.1, color.2),
                            })
                        }
                    }
                });
            // .render(chunks[0], &mut f)
            f.render_widget(canvas, chunks[0]);
        })?;

        if let Event::Input(input) = rx.recv()? {
            match input {
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    terminal.hide_cursor()?;
                    terminal.flush()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
