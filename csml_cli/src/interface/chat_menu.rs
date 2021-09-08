use std::{error::Error, io::Stdout};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

use std::collections::VecDeque;

use crate::interface::{chat_widget::*, AppRun, AppState};
use crate::run::{init_request, init_request_flow_trigger};
use unicode_width::UnicodeWidthStr;

static HELP: &str = "
[Chat Mode]
Press Escape key to exit Chat Mode.
Arrow keys to navigate.
ALL other keys to interact with the bot

[Normal Mode]
Press 'q' key to EXIT the app.
Press 'r' key to reload bot.
Press 'e' key to go back to Chat Mode
Arrow keys to navigate Payload Debugger.
";

#[derive(Debug)]
pub struct Messages {
    pub sender: String,
    pub messages: Vec<String>,
}

// #################################################################################

use csml_engine::{
    data::{BotOpt, CsmlRequest},
    start_conversation,
};

pub fn format_initial_payload(
    flow: Option<&str>,
    step: Option<&str>,
    text: Option<&str>,
) -> Option<CsmlRequest> {
    match (flow, step, text) {
        (Some(flow), Some(step), _) => Some(init_request_flow_trigger(flow, Some(step))),
        (Some(flow), _, _) => Some(init_request_flow_trigger(flow, None)),
        (_, Some(step), _) => Some(init_request_flow_trigger("default", Some(step))),
        (_, _, Some(text)) => Some(init_request(text, None)),
        _ => None,
    }
}

pub fn run_conversation(
    user_input: &str,
    request: Option<CsmlRequest>,
    run_opt: BotOpt,
    metadata: Option<serde_json::Value>,
    app: &mut AppRun,
) -> Result<bool, Box<dyn Error>> {
    let request = match request {
        Some(mut request) => {
            if let Some(metadata) = metadata {
                request.metadata = metadata;
            }
            request
        }
        None => init_request(user_input, metadata),
    };

    match start_conversation(request, run_opt.clone()) {
        Ok(obj) => {
            let raw_msg = RawMessage {
                sender: "bot".to_owned(),
                raw_messages: obj["messages"].as_array().unwrap_or(&vec![]).to_owned(),
            };

            app.messages.items.push_front(raw_msg);

            app.messages.unselect();

            Ok(false)
        }
        Err(err) => {
            let raw_msg = RawMessage {
                sender: "Error".to_owned(),
                raw_messages: vec![serde_json::json!(
                    {
                        "payload":{
                            "content_type": "error",
                            "content": format!("{:?}", err)
                        }
                    }
                )],
            };

            app.messages.items.push_front(raw_msg);

            app.messages.unselect();
            Ok(false)
        }
    }
}

// fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
//     let popup_layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints(
//             [
//                 Constraint::Percentage((100 - percent_y) / 2),
//                 Constraint::Percentage(percent_y),
//                 Constraint::Percentage((100 - percent_y) / 2),
//             ]
//             .as_ref(),
//         )
//         .split(r);

//     Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints(
//             [
//                 Constraint::Percentage((100 - percent_x) / 2),
//                 Constraint::Percentage(percent_x),
//                 Constraint::Percentage((100 - percent_x) / 2),
//             ]
//             .as_ref(),
//         )
//         .split(popup_layout[1])[1]
// }

// flow: Option<&str>,
// step: Option<&str>,
// text: Option<&str>,
// let run_opt = BotOpt::CsmlBot(load_info());
// let mut start_payload = format_initial_payload(flow, step, text);

pub fn draw_run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut AppRun,
) -> Result<(), Box<dyn Error>> {
    // Draw UI
    terminal.draw(|f| {
        let s = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(Rect {
                x: 0,
                y: 0,
                width: s.width / 2,
                height: s.height,
            });

        // ############################# [INFO] chunk 0
        let (msg, style) = match &app.input_mode {
            AppState::RunEditing => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop chatting, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to send the message"),
                ],
                Style::default(),
            ),
            _run_normal => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start chatting, "),
                    Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to reload bot."),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
        };

        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[0]);
        // ##################################################

        // ############################# [Chat] chunk 1
        let len = chunks[1].width as usize - 2;

        let messages = format_messages(
            &app.messages.items,
            len,
            &mut app.messages.sub_item,
            app.messages.state.clone(),
        );

        let chat = Chat::new(messages, app.input.clone(), len).style(match app.input_mode {
            AppState::RunEditing => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });

        f.render_stateful_widget(chat, chunks[1], &mut app.messages.state);
        // #############################

        // ############################# [input] chunk 2
        let input = Paragraph::new(app.input.as_ref())
            .style(match app.input_mode {
                AppState::RunEditing => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, chunks[2]);
        // #############################

        // ############################# set cursor
        match app.input_mode {
            AppState::RunEditing => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[2].x + app.input.width() as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[2].y + 1,
                )
            }
            _ =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}
        }
        // #############################

        // ############################### optional window
        let info_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Min(1), Constraint::Length(20)].as_ref())
            .split(Rect {
                x: (s.width as f32 / 2.05) as u16,
                y: 1,
                width: (s.width as f32 / 1.9) as u16,
                height: s.height,
            });

        let (title, payload) = match &app.messages.sub_item.raw_payload {
            serde_json::Value::Null => (None, "".to_owned()),
            value => {
                let content_type = value["content_type"].as_str();
                let mut content = "".to_owned();

                if let Some(_) = content_type {
                    content =
                        serde_json::to_string_pretty(&value["content"]).unwrap_or("".to_owned());
                }

                (content_type, content)
            }
        };

        let payload_box = Paragraph::new(payload)
            .style(match app.input_mode {
                AppState::RunNormal => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(if let Some(title) = title {
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Payload Debugger --- content_type: {} ", title))
            } else {
                Block::default()
                    .borders(Borders::ALL)
                    .title("Payload Debugger")
            })
            .scroll((app.scroll_y, app.scroll_x));

        f.render_widget(payload_box, info_layout[0]);

        // ############################### help menu

        let help = Paragraph::new(HELP)
            .style(Style::default().fg(Color::LightBlue))
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .wrap(Wrap { trim: true });

        f.render_widget(help, info_layout[1]);
    })?;

    Ok(())
}

pub fn format_messages<'a, 'b>(
    messages: &'a VecDeque<RawMessage>,
    window_length: usize,
    sub_item: &'b mut SubItem,
    state: ChatState,
) -> Vec<MessageType<'a>> {
    let mut vec = vec![];

    let mut global_index = 0;

    for msg in messages.iter() {
        let last = msg.raw_messages.len() - 1;
        // revert messages
        for (index, message) in msg.raw_messages.iter().rev().enumerate() {
            let mut spans = vec![];

            if index == last {
                get_header(msg.sender.as_str(), window_length, &mut spans);
            }

            match msg.sender.as_str() {
                "user" => {
                    add_line_to_span(
                        message["content"]["text"].as_str().unwrap_or(""),
                        &mut spans,
                        None,
                        window_length,
                        false,
                    );

                    match state.selected {
                        Some(index) if index == global_index => {
                            sub_item.raw_payload = message.clone();
                        }
                        _ => {}
                    };

                    // add spacing
                    spans.push(Spans::from(""));
                }
                _bot => {
                    let selected = match state.selected {
                        Some(index) if index == global_index => {
                            sub_item.raw_payload = message["payload"].clone();
                            Some(state.sub_selected)
                        }
                        _ => None,
                    };

                    bot_message_to_raw_msg(
                        &message["payload"],
                        &mut spans,
                        window_length,
                        sub_item,
                        selected,
                    );
                }
            };

            ////////////// save message to display
            let tmp = MessageType::Text {
                content: spans.into(),
                style: Style::default(),
                selected: false,
            };

            vec.push(tmp);
            //////////////

            global_index += 1;
        }
    }
    vec
}

pub fn bot_message_to_raw_msg<'a>(
    message: &serde_json::Value,
    spans: &mut Vec<Spans>,
    window_length: usize,
    sub_item: &'a mut SubItem,
    selected: Option<usize>,
) {
    let content_type = message["content_type"].as_str().unwrap_or("text");

    match content_type {
        "text" => {
            let msg = message["content"][content_type]
                .as_str()
                .unwrap_or("")
                .to_owned();

            add_line_to_span(&msg, spans, None, window_length, true);
        }
        "button" => {
            let title = message["content"]["title"]
                .as_str()
                .unwrap_or("Null")
                .to_owned();

            let payload = message["content"]["payload"]
                .as_str()
                .unwrap_or("Null")
                .to_owned();

            let button = format!("[{}]", title);

            match selected {
                Some(_) => {
                    sub_item.selected = true;
                    sub_item.array_length = Some(1);
                    sub_item.payload = payload;

                    add_line_to_span(
                        &button,
                        spans,
                        Some(Style::default().fg(Color::LightMagenta)),
                        window_length,
                        true,
                    );
                }
                None => {
                    add_line_to_span(&button, spans, None, window_length, true);
                }
            }
        }
        "question" => {
            let title = message["content"]["title"].as_str().unwrap_or("");
            let buttons = message["content"]["buttons"].as_array().map(|v| {
                v.iter()
                    .map(|b| {
                        let title = b["content"]["title"].as_str().unwrap_or("");
                        let payload = b["content"]["payload"].as_str().unwrap_or("");

                        (title.to_owned(), payload.to_owned())
                    })
                    .collect::<Vec<(String, String)>>()
            });

            add_line_to_span(title, spans, None, window_length, true);

            if let Some(buttons) = buttons {
                if let Some(_) = selected {
                    sub_item.array_length = Some(buttons.len());
                }

                let span =
                    buttons
                        .iter()
                        .enumerate()
                        .fold(vec![], |mut acc, (i, (title, payload))| {
                            let button = format!("[{}] ", title);

                            let button = match (selected, i) {
                                (Some(index), i) if i == index => {
                                    sub_item.selected = true;
                                    sub_item.payload = payload.to_owned();

                                    Span::styled(button, Style::default().fg(Color::LightMagenta))
                                }
                                _ => Span::raw(button),
                            };
                            acc.push(button);
                            acc
                        });

                let lines = split_bot_spans(span, window_length);
                for line in lines {
                    spans.push(Spans::from(line));
                }
            }
        }
        _ => {
            let mut chars: Vec<char> = message["content_type"]
                .as_str()
                .unwrap_or(" ")
                .chars()
                .collect();
            if !chars.is_empty() {
                chars[0] = chars[0].to_uppercase().nth(0).unwrap_or(chars[0]);
            }

            let content_type: String = chars.into_iter().collect();

            let line = format!("💬 {}(...)", content_type);

            add_line_to_span(&line, spans, None, window_length, true);
        }
    };

    spans.push(Spans::from(""));
}

fn split_bot_spans<'a>(spans: Vec<Span<'a>>, window_length: usize) -> Vec<Vec<Span<'a>>> {
    let max_usable_len = (window_length as f32 * 0.8) as usize;

    let mut total_spans = vec![];

    let mut total_len = 0;
    let mut sub_vec = vec![];
    for span in spans {
        if total_len <= max_usable_len {
            total_len += span.width();
            sub_vec.push(span);
        } else {
            total_spans.push(sub_vec);
            total_len = 0;
            sub_vec = vec![];
        }
    }

    if !sub_vec.is_empty() {
        total_spans.push(sub_vec);
    }

    total_spans
}

pub fn add_line_to_span(
    message: &str,
    spans: &mut Vec<Spans>,
    style: Option<Style>,
    window_length: usize,
    left_align: bool,
) {
    let max_usable_len = (window_length as f32 * 0.8) as usize;

    if message.len() >= max_usable_len {
        match message.find(' ') {
            Some(_) => {
                let split_line = message.split(' ').collect::<Vec<&str>>();
                split_white_space(
                    &split_line,
                    spans,
                    style,
                    max_usable_len,
                    window_length,
                    left_align,
                );
            }
            None => {
                let mut rest = message;
                let mut split_line = vec![];
                while rest.len() >= max_usable_len {
                    let (line, end) = message.split_at(max_usable_len);
                    split_line.push(line);
                    rest = end;
                }
                split_line.push(rest);

                for line in split_line {
                    add_line(line, spans, style, window_length, left_align);
                }
            }
        }
    } else {
        add_line(&message, spans, style, window_length, left_align);
    }
}

pub fn add_line(
    line: &str,
    spans: &mut Vec<Spans>,
    style: Option<Style>,
    window_length: usize,
    left_align: bool,
) {
    let skip_len = match left_align {
        true => 0,
        false if window_length > line.len() => window_length - line.len(),
        _ => 0,
    };

    let ln = format!("{}{}", " ".repeat(skip_len), line);

    let span = match style {
        Some(style) => Spans::from(Span::styled(ln, style)),
        None => Spans::from(ln),
    };

    spans.push(span);
}

pub fn split_white_space(
    split_line: &[&str],
    spans: &mut Vec<Spans>,
    style: Option<Style>,
    max_usable_len: usize,
    window_length: usize,
    left_align: bool,
) {
    let mut current_line_len = 0;
    let mut start: usize = 0;
    let last_element: usize = split_line.len() - 1;

    let l_tmp = split_line
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (index, &string)| {
            current_line_len += string.len();

            // Check if 'current_line_len' is bigger than the current max_usable_len
            // if so remove the last 'chuck' and create a new 'current_line_len'
            // else add +1 length 'current_line_len' to account for separator ' '
            if index > 0 && current_line_len > max_usable_len {
                acc.push((start, index - 1));
                start = index;
                current_line_len = string.len();
            } else {
                current_line_len += 1;
            }

            // if at the end there is a 'current_line_len' add it to the list
            if current_line_len != 0 && index == last_element {
                acc.push((start, index));
            }

            acc
        });

    for (start, end) in l_tmp {
        let line = split_line[start..=end].join(" ");
        add_line(&line, spans, style, window_length, left_align);
    }
}

fn get_header(sender: &str, length: usize, spans: &mut Vec<Spans>) {
    // header
    let header = match sender {
        "user" => {
            let skip_len = length - sender.len();

            Spans::from(vec![
                Span::from("-".repeat(skip_len)),
                Span::styled(format!("{}", sender), Style::default().fg(Color::LightBlue)),
            ])
        }
        _bot => {
            let skip_len = length - sender.len();

            Spans::from(vec![
                Span::styled(format!("{}", sender), Style::default().fg(Color::LightRed)),
                Span::from("-".repeat(skip_len)),
            ])
        }
    };

    spans.push(header);
    spans.push(Spans::from(""));
}
