// use crossterm::style::SetForegroundColor;
use csml_interpreter::data::CsmlBot;
use rand::{distributions::Alphanumeric, Rng};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, StatefulWidget, Widget},
};


use crate::init_package::{DataBase, DynamoRegion, Env, S3Region};
use crate::interface::AppState;
use std::iter::Iterator;

////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Menu<'a> {
    pub block: Option<Block<'a>>,
    pub items: Vec<MenuItem>,

    pub input: &'a str,
    /// Style used as a base style for the widget
    pub style: Style,

    /// Style used to render selected item
    pub highlight_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    pub highlight_symbol: &'a str,
}

impl<'a> Menu<'a> {
    pub fn new<T>(items: T, input: &'a str) -> Menu<'a>
    where
        T: Into<Vec<MenuItem>>,
    {
        Menu {
            block: None,
            items: items.into(),
            input,
            style: Style::default(),
            highlight_style: Style::default(),
            highlight_symbol: "=> ",
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Menu<'a> {
        self.block = Some(block);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Menu<'a> {
        self.highlight_style = style;
        self
    }
}

impl<'a> StatefulWidget for Menu<'a> {
    type State = MenuState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);

        let mode = &state.state;
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }
        if self.items.is_empty() {
            return;
        }

        let list_height = list_area.height as usize;

        let mut start = state.offset;
        let mut end = state.offset;
        let mut height = 0;

        for (i, item) in self.items.iter_mut().enumerate().skip(state.offset) {
            item.selected = state.selected.map(|s| s == i).unwrap_or(false);

            if height + item.height(mode) > list_height {
                break;
            }

            height += item.height(mode);
            end += 1;
        }

        // SELECTOR //////////////////////////////////////////////////////////////////// can be refacto ?????
        let selected = state.selected.unwrap_or(0).min(self.items.len() - 1);
        while selected >= end {
            height = height.saturating_add(self.items[end].height(mode));
            end += 1;
            while height > list_height {
                height = height.saturating_sub(self.items[start].height(mode));
                start += 1;
            }
        }
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.items[start].height(mode));
            while height > list_height {
                end -= 1;
                height = height.saturating_sub(self.items[end].height(mode));
            }
        }
        state.offset = start;

        ////////////////////////////////////////////////////////////////////

        let mut current_height = 0;
        let current_width = 40;

        for item in self
            .items
            .iter_mut()
            .skip(state.offset)
            .take(end - start)
        {
            let (x, y) = (list_area.left(), list_area.top() + current_height);
            current_height += item.height(mode) as u16;

            // add padding for sub menus
            let x = x + item.lvl;

            let area = Rect {
                x,
                y,
                width: list_area.width,
                height: item.height(mode) as u16,
            };

            let item_style = self.style.patch(item.style);
            buf.set_style(area, item_style);

            if item.selected {
                buf.set_style(area, self.highlight_style);
            }

            let tmp = if x > current_width {
                x - current_width
            } else {
                0
            };

            let max_element_width = if tmp > list_area.width {
                (list_area.width - tmp) as usize
            } else {
                list_area.width as usize
            };

            let mut elem_y;

            let component = item.gen_components(self.input, &state.state);
            for (j, line) in component.lines.iter().enumerate() {
                elem_y = y + j as u16;
                buf.set_spans(x, elem_y, line, max_element_width as u16);
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
pub trait MenuType {
    fn generate_menu(&mut self) -> Vec<MenuItem>;

    fn init() -> Box<dyn MenuType>
    where
        Self: Sized;

    fn get_default_state() -> AppState
    where
        Self: Sized;
}
////////////////////////////////////////////////////////////////////

pub fn get_sub_components(component: &MenuComponent, sub_component: &mut Vec<MenuItem>) {
    match component {
        MenuComponent::Text {sub_component: list, ..} => {
            for item in list {
                sub_component.push(item.clone());
                get_sub_components(&item.component, sub_component);
            }
        },
        MenuComponent::List { selected, components, ..} => {
            let selected_component = &components[*selected];

            get_sub_components(selected_component, sub_component)
        },
        _ => {},
    }
}

fn generate_list(old_items: &Vec<MenuItem>, index: Option<(u16, usize)>) -> Vec<MenuItem> {

    let mut lvl_diff = true;

    old_items.iter().enumerate().fold(vec![], |mut acc, (i, item)| {
        match index {
            Some((lvl, index)) if index < i && lvl_diff && lvl >= item.lvl => {
                lvl_diff = false;
            }
            _ => {}
        };
        match index {
            Some((_, index)) if index == i => {
                acc.push(item.clone());
                get_sub_components(&item.component, &mut acc);
            }
            Some((_, index)) if index < i && lvl_diff => {}
            Some(_) => {
                acc.push(item.clone());
            }
            None => {
                acc.push(item.clone());
                get_sub_components(&item.component, &mut acc);
            }
        };

        acc
    })
}

pub struct MenuState {
    pub offset: usize,
    pub selected: Option<usize>,
    pub menu: Vec<MenuItem>,
    pub menu_type: Box<dyn MenuType>,
    pub state: AppState,
}

impl MenuState {
    pub fn init<T>() -> MenuState
    where
        T: MenuType,
    {
        let mut menu_type = T::init();
        let state = T::get_default_state();

        let raw_menu = menu_type.generate_menu();
        let menu = generate_list(&raw_menu, None);

        MenuState {
            offset: 0,
            selected: Some(0),
            menu,
            menu_type,
            state,
        }
    }

    pub fn change_list(&mut self, index: Option<(u16, usize)>) {
        self.menu = generate_list(&self.menu, index);
    }

    pub fn next(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i < self.menu.len() - 1 {
                    i + 1
                } else {
                    0
                }
            }
            None => 0,
        };

        self.selected = Some(i);
    }

    pub fn previous(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    self.menu.len() - 1
                }
            }
            None => 0,
        };

        self.selected = Some(i);
    }

    pub fn gen_env(&self) -> Env {
        let bot_name = self.menu[0].get_value();
        let is_encrypted = self.menu[1].get_value();
        let db = self.menu[2].get_value();

        let database = match db.as_str() {
            "mongodb" => {
                let uri = self.menu[3].get_value();

                DataBase::MongoDB { uri }
            }
            "postgresql" => {
                let uri = self.menu[3].get_value();

                DataBase::Postgres { uri }
            }
            _dynamodb => {

                let mut local_host = 0;

                let dynamodb_region = match self.menu[3].get_value().as_str() {
                    "localhost" => {
                        local_host +=1;
                        DynamoRegion::Endpoint(format!(
                            "http://localhost:{}",
                            self.menu[4].get_value()
                        ))
                    },
                    _ => DynamoRegion::Region(self.menu[3].get_value()),
                };

                let s3_region = match self.menu[5 + local_host].get_value().as_str() {
                    "localhost" => {
                        local_host += 1;
                        S3Region::Endpoint(format!("http://localhost:{}", self.menu[5 + local_host].get_value()))
                    },
                    _ => {
                        S3Region::Region(self.menu[5 + local_host].get_value())
                    }
                };

                DataBase::DynamoDB {
                    dynamodb_region,
                    dynamodb_table: self.menu[4 + local_host].get_value(),
                    s3_region,
                    s3_bucket: self.menu[6 + local_host ].get_value(),
                }
            }
        };

        let encryption = match is_encrypted.as_str() {
            "yes" => {
                let encryption: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(15)
                    .map(char::from)
                    .collect();

                Some(encryption)
            }
            _no => None,
        };

        Env {
            bot_name,
            database,
            encryption,
        }
    }
}


////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum MenuComponent {
    Text{
        text: String,
        payload: Option<String>,
        sub_component: Vec<MenuItem>
    },
    Button{
        text: String,
        payload: Option<String>
    },
    List {
        components: Vec<MenuComponent>,
        selected: usize,
        scroll_index: usize,
    },

    Bot {
        path_info: String,
        bot: CsmlBot,
        text: String,
    },
}

impl MenuComponent {

    pub fn height(&self) -> usize {
        match &self {
            Self::Text{..} => 2,
            Self::Button{..} => 1,
            Self::List { components, .. } => components.len() + 2,
            Self::Bot { .. } => 1,
        }
    }

    pub fn get_value(&self) -> String {
        match &self {
            MenuComponent::List {selected, components, ..} => components[*selected].get_value(),
            MenuComponent::Text{payload: Some(value), ..} => value.to_owned(),
            MenuComponent::Text{text: value, ..} => value.to_owned(),
            MenuComponent::Button{payload: Some(value), ..} => value.to_owned(),
            MenuComponent::Button{text: value, ..} => value.to_owned(),
            MenuComponent::Bot{path_info, ..} => path_info.to_owned(),
        }
    }
}

////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub text: String,
    pub component: MenuComponent,

    pub lvl: u16,
    pub value: String,

    pub selected: bool,
    pub style: Style,
}

impl MenuItem {
    pub fn new(text: &str, component: MenuComponent, lvl: u16) -> MenuItem {
        MenuItem {
            text: text.to_owned(),
            component,
            lvl,
            value: String::new(),
            selected: false,
            style: Style::default(),
        }
    }

    pub fn get_value(&self) -> String {
        match &self.component {
            MenuComponent::List { selected, components, ..} => components[*selected].get_value(),
            MenuComponent::Text{payload: Some(value), ..} => value.to_owned(),
            MenuComponent::Text{text: value, ..} => value.to_owned(),
            MenuComponent::Button{payload: Some(value), ..} => value.to_owned(),
            MenuComponent::Button{text: value, ..} => value.to_owned(),
            MenuComponent::Bot { path_info, ..} => path_info.to_owned(),
        }
    }

    pub fn update_value(&mut self, value: &str) {
        match &mut self.component {
            MenuComponent::List {
                selected,
                scroll_index,
                ..
            } => {
                *selected = *scroll_index;
            }
            MenuComponent::Text{ text: old_value , payload, ..} => {
                if let Some(payload) = payload {
                    *payload = value.to_string();
                };

                *old_value = value.to_string();
            }
            MenuComponent::Button{ .. } => {}
            MenuComponent::Bot { .. } => {}
        }
    }

    pub fn next(&mut self) {
        if let MenuComponent::List {
            scroll_index, components, ..
        } = &mut self.component
        {
            if *scroll_index < components.len() - 1 {
                *scroll_index += 1;
            } else {
                *scroll_index = 0
            }
        }
    }

    pub fn previous(&mut self) {
        if let MenuComponent::List { scroll_index, components, .. } = &mut self.component {
            if *scroll_index > 0 {
                *scroll_index -= 1;
            } else {
                *scroll_index = components.len() - 1;
            }
        }
    }

    fn format_text(&self) -> String {
        if !self.text.is_empty() {
            if self.lvl == 0 {
                format!("{}: ", self.text.to_owned())
            } else {
                format!("|-- {}: ", self.text)
            }
        } else {
            self.text.to_owned()
        }
    }

    fn format_span<'a>(
        &self,
        mode: &AppState,
        elem: String,
        additional_info: Option<Vec<Spans::<'a>>>
    ) -> Text::<'a> {
        // (160, 231, 229) blue
        // (180, 248, 200) green
        let span = Spans::from(vec![
            if self.selected && *mode == AppState::Normal {
                Span::styled("=> ", Style::default().fg(Color::Rgb(160, 231, 229)))
            } else {
                Span::from("   ")
            },
            Span::styled(
                self.format_text(),
                Style::default()
                    .fg(Color::Rgb(180, 248, 200))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                elem,
                if self.selected {
                    Style::default().fg(Color::LightYellow)
                } else {
                    Style::default().fg(Color::Rgb(180, 248, 200))
                },
            ),
        ]);

        let mut spans = vec![span];

        if let Some(additional_spans) = additional_info {
            for span in additional_spans {
                spans.push(span);
            }
        }

        Text { lines: spans }
    }

    pub fn gen_components(&self, input: &str, mode: &AppState) -> Text {
        match &self.component {
            MenuComponent::Text{..} if self.selected && *mode == AppState::Editing => {
                self.format_span(
                    mode, input.to_owned(), None
                )
            },
            MenuComponent::Text{payload: Some(text), ..} => {
                self.format_span(
                    mode, text.to_owned(), None
                )
            },
            MenuComponent::Text{text, ..} => {
                self.format_span(
                    mode, text.to_owned(), None
                )
            },

            MenuComponent::List {
                components: list_components, scroll_index, ..
            } if self.selected && *mode == AppState::Selecting => {
                let additional_info = Some(
                    list_components.iter()
                        .enumerate()
                        .map(|(i, value)| {
                            let symbol = if *mode == AppState::Selecting && i == *scroll_index {
                                "=> "
                            } else {
                                "   "
                            };

                            Spans::from(vec![Span::styled(
                                format!("{}  |-- {}", symbol, value.get_value()),
                                Style::default().fg(Color::LightYellow),
                            )])
                        })
                        .collect::<Vec<Spans>>(),
                );

                self.format_span(
                    mode, "".to_owned(), additional_info
                )
            }
            MenuComponent::List { selected, components: list_components, .. } => {
                self.format_span(
                    mode, list_components[*selected].get_value(), None
                )
            },

            MenuComponent::Button{payload: Some(text), ..} => {
                self.format_span(
                    mode, text.to_owned(), None
                )
            },
            MenuComponent::Button{text, ..} => {
                self.format_span(
                    mode, text.to_owned(), None
                )
            },
            MenuComponent::Bot { text, .. } => {
                self.format_span(
                    mode, text.to_owned(), None
                )
            },
        }
    }

    pub fn height(&self, mode: &AppState) -> usize {
        match self.selected && *mode == AppState::Selecting {
            true => self.component.height(),
            false => 1,
        }
    }
}
