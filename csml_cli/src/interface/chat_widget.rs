use tui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use std::collections::VecDeque;

use std::iter::Iterator;

#[derive(Debug, Clone)]
pub struct RawMessage {
    pub sender: String,
    pub raw_messages: Vec<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum MessageType<'a> {
    Text {
        content: Text<'a>,
        style: Style,
        selected: bool,
    },
}

impl<'a> MessageType<'a> {
    pub fn set_selection(&mut self, is_selected: bool) {
        match self {
            Self::Text { selected, .. } => *selected = is_selected,
        }
    }

    pub fn is_selected(&self) -> bool {
        match self {
            Self::Text { selected, .. } => *selected,
        }
    }

    pub fn display(&self, buf: &mut Buffer, x: u16, y: u16, max_element_width: u16) {
        match self {
            Self::Text { content, .. } => {
                let mut elem_y;

                for (j, line) in content.lines.iter().enumerate() {
                    elem_y = y + j as u16;
                    buf.set_spans(x, elem_y, line, max_element_width as u16);
                }
            }
        }
    }

    pub fn get_style(&self) -> Style {
        match self {
            Self::Text { style, .. } => style.to_owned(),
        }
    }

    pub fn height(&self) -> usize {
        match self {
            // update normal and self.selected element height
            Self::Text { content, .. } => content.height(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chat<'a> {
    pub block: Option<Block<'a>>,
    pub messages: Vec<MessageType<'a>>,

    pub input: String,
    pub length: usize,

    /// Style used as a base style for the widget
    pub style: Style,
    pub start_corner: Corner,

    /// Style used to render selected item
    pub highlight_style: Style,
}

impl<'a> Chat<'a> {
    pub fn new<T>(messages: T, input: String, length: usize) -> Chat<'a>
    where
        T: Into<Vec<MessageType<'a>>>,
    {
        Chat {
            block: Some(Block::default().borders(Borders::ALL).title("Chat Window")),
            messages: messages.into(),
            input,
            length,
            style: Style::default(),
            start_corner: Corner::BottomLeft,
            highlight_style: Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        }
    }

    pub fn style(mut self, style: Style) -> Chat<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for Chat<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ChatState::default();

        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a> StatefulWidget for Chat<'a> {
    type State = ChatState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // set style

        // match st
        buf.set_style(area, self.style);

        // .style(match app.input_mode {
        //     AppState::RunEditing => Style::default().fg(Color::Yellow),
        //     _ => Style::default(),
        // })

        // get Widget type
        // let mode = &state.mode;

        // ################## get area and set protections protections
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
        if self.messages.is_empty() {
            return;
        }
        // ################## get area and protections

        // ################## find chunk of list to display
        let list_height = list_area.height as usize;

        let mut start = state.offset;
        let mut end = state.offset;
        let mut height = 0;

        for (i, message) in self.messages.iter_mut().enumerate().skip(state.offset) {
            message.set_selection(state.selected.map(|s| s == i).unwrap_or(false));

            if height + message.height() > list_height {
                break;
            }

            height += message.height();
            end += 1;
        }

        let selected = state.selected.unwrap_or(0).min(self.messages.len() - 1);
        while selected >= end {
            height = height.saturating_add(self.messages[end].height());
            end += 1;
            while height > list_height {
                height = height.saturating_sub(self.messages[start].height());
                start += 1;
            }
        }
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.messages[start].height());
            while height > list_height {
                end -= 1;
                height = height.saturating_sub(self.messages[end].height());
            }
        }

        state.offset = start;

        // ##################

        // ################## display list chunk

        let mut current_height = 0;

        // let has_selection = state.selected.is_some();
        for message in self
            .messages
            .iter_mut()
            .skip(state.offset)
            .take(end - start)
        {
            current_height += message.height() as u16;
            let (x, y) = (list_area.left(), list_area.bottom() - current_height);

            let area = Rect {
                x,
                y,
                width: list_area.width,
                height: message.height() as u16,
            };

            let item_style = self.style.patch(message.get_style());
            buf.set_style(area, item_style);

            if message.is_selected() {
                buf.set_style(area, self.highlight_style);
            }

            // let max_element_width = (list_area.width - (current_width - x)) as u16;
            message.display(buf, x, y, list_area.width);
        }
        // ###########################################################################
    }
}

#[derive(Debug, Clone)]
pub struct ChatState {
    pub offset: usize,
    pub selected: Option<usize>,
    pub sub_selected: usize,

    pub selected_item: Option<serde_json::Value>,
}

impl Default for ChatState {
    fn default() -> ChatState {
        ChatState {
            offset: 0,
            selected: None,
            sub_selected: 0,
            selected_item: None,
        }
    }
}

impl ChatState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }
}

// ############################################################################## StatefulList
pub struct SubItem {
    pub selected: bool,
    pub array_length: Option<usize>,
    pub payload: String,
    pub raw_payload: serde_json::Value,
}

impl SubItem {
    pub fn default() -> Self {
        Self {
            selected: false,
            array_length: None,
            payload: "".to_owned(),
            raw_payload: serde_json::Value::Null,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub struct StatefulList {
    pub state: ChatState,
    pub items: VecDeque<RawMessage>,

    pub sub_item: SubItem,
}

impl StatefulList {
    pub fn new() -> StatefulList {
        StatefulList {
            state: ChatState::default(),
            items: VecDeque::new(),
            sub_item: SubItem::default(),
        }
    }

    pub fn len(&self) -> usize {
        let len = self.items.iter().fold(0, |mut acc, item| {
            acc += item.raw_messages.len();
            acc
        });

        len
    }

    ///////
    pub fn left(&mut self) {
        if self.state.sub_selected > 0 {
            self.state.sub_selected -= 1
        }
    }

    pub fn right(&mut self) {
        let len = match self.sub_item.array_length {
            Some(len) => len,
            None => return,
        };

        if len > 0 && self.state.sub_selected < len - 1 {
            self.state.sub_selected += 1
        }
    }
    ///////

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.len() > 0 && i < self.len() - 1 {
                    i + 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
