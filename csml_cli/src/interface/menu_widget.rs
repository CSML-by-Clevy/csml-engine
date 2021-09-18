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

use std::iter::Iterator;

use crate::init_package::{DataBase, DynamoRegion, Env, S3Region};
use crate::interface::AppState;

const AWS_REGIONS: &[&str] = &[
    "US East (Ohio) us-east-2",
    "US East (N. Virginia) us-east-1",
    "US West (N. California) us-west-1",
    "US West (Oregon) us-west-2",
    "Africa (Cape Town) af-south-1",
    "Asia Pacific (Hong Kong) ap-east-1",
    "Asia Pacific (Mumbai) ap-south-1",
    "Asia Pacific (Osaka) ap-northeast-3",
    "Asia Pacific (Seoul) ap-northeast-2",
    "Asia Pacific (Singapore) ap-southeast-1",
    "Asia Pacific (Sydney) ap-southeast-2",
    "Asia Pacific (Tokyo) ap-northeast-1",
    "Canada (Central) ca-central-1",
    "Europe (Frankfurt) eu-central-1",
    "Europe (Ireland) eu-west-1",
    "Europe (London) eu-west-2",
    "Europe (Milan) eu-south-1",
    "Europe (Paris) eu-west-3",
    "Europe (Stockholm) eu-north-1",
    "Middle East (Bahrain) me-south-1",
    "South America (São Paulo) sa-east-1",
    "AWS GovCloud (US-East) us-gov-east-1",
    "AWS GovCloud (US-West)  us-gov-west-1",

    "localhost",
];

const AWS_REGIONS_PAYLOAD: &[&str] = &[
    "us-east-2",
    "us-east-1",
    "us-west-1",
    "us-west-2",
    "af-south-1",
    "ap-east-1",
    "ap-south-1",
    "ap-northeast-3",
    "ap-northeast-2",
    "ap-southeast-1",
    "ap-southeast-2",
    "ap-northeast-1",
    "ca-central-1",
    "eu-central-1",
    "eu-west-1",
    "eu-west-2",
    "eu-south-1",
    "eu-west-3",
    "eu-north-1",
    "me-south-1",
    "sa-east-1",
    "us-gov-east-1",
    "us-gov-west-1",

    "localhost",
];

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

        // can be remove ?????
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

        // let highlight_symbol = self.highlight_symbol.unwrap_or("");
        // let blank_symbol = iter::repeat(" ")
        //     .take(highlight_symbol.width())
        //     .collect::<String>();

        let mut current_height = 0;
        let current_width = 40;

        for (i, item) in self
            .items
            .iter_mut()
            .enumerate()
            .skip(state.offset)
            .take(end - start)
        {
            // ########### skip sub menus
            // ## 3 -> 4
            // if i == 3 {
            //     if let MenuComponent::List { selected, components, .. } = &item.component {
            //         let val = components[*selected];
            //         if val != "dynamodb-local" {
            //             state.skip_4 = true;
            //         } else {
            //             state.skip_4 = false;
            //         }
            //     };
            // }
            // if i == 4 && state.skip_4 {
            //     continue;
            // }
            // // ## 6 -> 7
            // if i == 6 {
            //     if let MenuComponent::List { selected, components, .. } = &item.component {
            //         let val = components[*selected];
            //         if val != "localstack" {
            //             state.skip_7 = true;
            //         } else {
            //             state.skip_7 = false;
            //         }
            //     };
            // }
            // if i == 7 && state.skip_7 {
            //     continue;
            // }
            // ###########

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

            let content = item.gen_content(self.input, &state.state);
            for (j, line) in content.lines.iter().enumerate() {
                elem_y = y + j as u16;
                buf.set_spans(x, elem_y, line, max_element_width as u16);
            }
        }
    }
}

pub trait MenuType {
    fn generate_menu(&mut self, menu: &Vec<MenuItem>) -> Vec<MenuItem>;

    fn init() -> Box<dyn MenuType>
    where
        Self: Sized;

    fn get_default_state() -> AppState
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct InitMenu {
    pub start: Vec<MenuItem>,
}

impl MenuType for InitMenu {
    fn init() -> Box<dyn MenuType> {
        let mongodb_sub_menu = vec![
            MenuItem::new(
                "MongoDB Connection String URI",
                MenuComponent::Text{text: "mongodb://localhost:27017".to_owned(), payload: None, sub_component: vec![]},
                1,
            )
        ];

        let dynamodb_sub_menu = vec![
            MenuItem::new(
                "DynamoDB region",
                MenuComponent::List {
                    components: vec![],//AWS_REGIONS,
                    selected: 0,
                    scroll_index: 0,
                    sub_component: vec![
                        // only if localhost |
                        MenuItem::new("Dynamodb port", MenuComponent::Text{text: "8000".to_owned(), payload: None, sub_component: vec![]}, 1)
                    ]
                },
                1,
            ),
            MenuItem::new(
                "Dynamodb table name",
                MenuComponent::Text{text: "csml-engine-db-local".to_owned(), payload: None, sub_component: vec![]},
                1,
            ),
            MenuItem::new(
                "S3 bucket Location",
                MenuComponent::List {
                    components: vec![],//AWS_REGIONS,
                    selected: 0,
                    scroll_index: 0,
                    sub_component: vec![
                        // only if localhost |
                        MenuItem::new("Localstack port", MenuComponent::Text{text: "8000".to_owned(), payload: None, sub_component: vec![]}, 2),
                    ]
                },
                1,
            ),
            MenuItem::new(
                "S3 bucket name",
                MenuComponent::Text{text: "csml-engine-bucket".to_owned(), payload: None, sub_component: vec![]},
                1,
            ),
        ];

        let init = InitMenu {
            start: vec![
                MenuItem::new("Chatbot name", MenuComponent::Text{text: "bot".to_owned(), payload: None, sub_component: vec![]}, 0),
                MenuItem::new(
                    "Enable variable encryption",
                    MenuComponent::List {
                        components: vec![
                                MenuComponent::Text{text: "yes".to_owned(), payload: None, sub_component: vec![]},
                                MenuComponent::Text{text: "no".to_owned(), payload: None, sub_component: vec![]}
                            ],
                        selected: 0,
                        scroll_index: 0,
                        sub_component: vec![]
                    },
                    0,
                ),
                MenuItem::new(
                    "Database type",
                    MenuComponent::List {
                        components: vec![
                                MenuComponent::Text{text: "mongodb".to_owned(), payload: None, sub_component: mongodb_sub_menu },
                                MenuComponent::Text{text: "dynamodb".to_owned(), payload: None, sub_component: dynamodb_sub_menu}
                            ],
                        selected: 0,
                        scroll_index: 0,
                        sub_component: vec![],
                    },
                    0,
                ),
                MenuItem::new(
                    "",
                    MenuComponent::Button{text: "               [Validate]".to_owned(), payload: None},
                    0,
                ),
            ],
        };

        Box::new(init)
    }

    fn get_default_state() -> AppState {
        AppState::Normal
    }

    fn generate_menu(&mut self, menu: &Vec<MenuItem>) -> Vec<MenuItem> {
        let mut new_menu = vec![];

        if menu.is_empty() {
            new_menu.append(&mut self.start.clone());
            // new_menu.append(&mut self.mongodb.clone());
            // new_menu.append(&mut self.end.clone());
        } else {
            let name = &menu[0];
            let encryption = &menu[1];
            let db = &menu[2];

            match db.get_value().as_str() {
                "mongodb" => {
                    new_menu.push(name.clone());
                    new_menu.push(encryption.clone());
                    new_menu.push(db.clone());
                    // new_menu.append(&mut self.mongodb.clone());
                    // new_menu.append(&mut self.end.clone());
                }
                _dynamodb => {
                    new_menu.push(name.clone());
                    new_menu.push(encryption.clone());
                    new_menu.push(db.clone());
                    // new_menu.append(&mut self.dynamodb.clone());
                    // new_menu.append(&mut self.end.clone());
                }
            }
        };

        new_menu
    }
}

pub struct MenuState {
    pub offset: usize,
    pub selected: Option<usize>,
    pub menu: Vec<MenuItem>,
    pub menu_type: Box<dyn MenuType>,
    pub state: AppState,

    // tmp
    pub skip_4: bool,
    pub skip_7: bool,
}

impl MenuState {
    pub fn init<T>() -> MenuState
    where
        T: MenuType,
    {
        let mut menu_type = T::init();
        let state = T::get_default_state();

        let menu = menu_type.generate_menu(&vec![]);

        MenuState {
            offset: 0,
            selected: Some(0),
            menu,
            menu_type,
            state,

            skip_4: false,
            skip_7: false,
        }
    }

    pub fn change_list(&mut self) {
        self.menu = self.menu_type.generate_menu(&self.menu);

        // TODO: save previous data
        self.skip_4 = false;
        self.skip_7 = false;
    }

    pub fn next(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i < self.menu.len() - 1 {
                    if i == 3 && self.skip_4 {
                        i + 2
                    } else if i == 6 && self.skip_7 {
                        i + 2
                    } else {
                        i + 1
                    }
                } else {
                    i
                }
            }
            None => 0,
        };

        self.selected = Some(i);
    }

    pub fn previous(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i == 5 && self.skip_4 {
                    i - 2
                } else if i == 8 && self.skip_7 {
                    i - 2
                } else if i > 0 {
                    i - 1
                } else {
                    i
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
            _dynamodb => {
                // pub skip_4: bool,
                // pub skip_7: bool,
                let dynamodb_region = match self.skip_4 {
                    true => DynamoRegion::Region(self.menu[3].get_value()),
                    false => DynamoRegion::Endpoint(format!(
                        "http://localhost:{}",
                        self.menu[4].get_value()
                    )),
                };
                let s3_region = match self.skip_7 {
                    true => S3Region::Region(self.menu[6].get_value()),
                    false => {
                        S3Region::Endpoint(format!("http://localhost:{}", self.menu[7].get_value()))
                    }
                };

                DataBase::DynamoDB {
                    dynamodb_region,
                    dynamodb_table: self.menu[5].get_value(),
                    s3_region,
                    s3_bucket: self.menu[8].get_value(),
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

// #[derive(Debug, Clone)]
// pub enum MenuElement<'a> {
//     Text(String),
//     Button(String),
//     List {
//         vec: &'a [&'a str],
//         selected: usize,
//         scroll_index: usize,
//     },
//     SelectableBot {
//         path_info: String,
//         bot: CsmlBot,
//         text: String,
//     },
// }

// impl<'a> MenuElement<'a> {
//     pub fn new_list(vec: &'a [&'a str]) -> Self {
//         Self::List {
//             vec,
//             selected: 0,
//             scroll_index: 0,
//         }
//     }

//     pub fn height(&self) -> usize {
//         match &self {
//             Self::Text(_) => 2,
//             Self::Button(_) => 1,
//             Self::List { vec, .. } => vec.len() + 2,
//             Self::SelectableBot { .. } => 1,
//         }
//     }
// }

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
        sub_component: Vec<MenuItem>,
    },

    Bot {
        path_info: String,
        bot: CsmlBot,
        text: String,
    },
}

impl MenuComponent {
    // pub fn new_list(vec: &'a [&'a str]) -> Self {
    //     Self::List {
    //         vec,
    //         selected: 0,
    //         scroll_index: 0,
    //     }
    // }

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
            MenuComponent::List { selected, components, ..} => components[*selected].get_value(),
            MenuComponent::Text{payload: Some(value), ..} => value.to_owned(),
            MenuComponent::Text{text: value, ..} => value.to_owned(),
            MenuComponent::Button{payload: Some(value), ..} => value.to_owned(),
            MenuComponent::Button{text: value, ..} => value.to_owned(),
            MenuComponent::Bot { path_info, ..} => path_info.to_owned(),
        }
    }

}

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
            }
        }
    }

    pub fn previous(&mut self) {
        if let MenuComponent::List { scroll_index, .. } = &mut self.component {
            if *scroll_index > 0 {
                *scroll_index -= 1;
            }
        }
    }

    pub fn gen_content(&self, input: &str, mode: &AppState) -> Text {
        let text = if !self.text.is_empty() {
            if self.lvl == 0 {
                format!("{}: ", self.text.to_owned())
            } else {
                format!("|-- {}: ", self.text)
            }
        } else {
            self.text.to_owned()
        };

        let mut additional_info = None;

        let elem: String = match &self.component {
            MenuComponent::Text{..} if self.selected && *mode == AppState::Editing => input.to_owned(),
            MenuComponent::Text{payload: Some(text), ..} => text.to_owned(),
            MenuComponent::Text{text, ..} => text.to_owned(),
            MenuComponent::List {
                components, scroll_index, ..
            } if self.selected && *mode == AppState::Selecting => {
                additional_info = Some(
                    components.iter()
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

                "".to_owned()
            }
            MenuComponent::List { selected, components, .. } => components[*selected].get_value(),
            MenuComponent::Button{payload: Some(text), ..} => text.to_owned(),
            MenuComponent::Button{text, ..} => text.to_owned(),
            MenuComponent::Bot { text, .. } => {
                // additional_info = Some(
                //     vec![
                //         Spans::from(Span::from(format!("Path: {}", path_info))),
                //         Spans::from(Span::from(format!("Bot name: {}", bot.name)))
                //     ]
                // );

                text.to_owned()
            }
        };

        // (160, 231, 229) blue
        // (180, 248, 200) green
        let span = Spans::from(vec![
            if self.selected && *mode == AppState::Normal {
                Span::styled("=> ", Style::default().fg(Color::Rgb(160, 231, 229)))
            } else {
                Span::from("   ")
            },
            Span::styled(
                text,
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

    pub fn height(&self, mode: &AppState) -> usize {
        match self.selected && *mode == AppState::Selecting {
            true => self.component.height(),
            false => 1,
        }
    }
}
