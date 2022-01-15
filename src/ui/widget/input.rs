use super::{Widget, TAB_WIDTH};
use crate::events::{Event, KeyCode};
use crate::ui::style::{Style, StyleSheet};
use crate::{Position, Size};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub position: Position,
    pub size: Size,

    pub style: StyleSheet,

    pub placeholder: String,
    pub text: String,

    pub selected: bool,
    pub is_password: bool,
    pub block_tab: bool,
    pub cursor: bool,
    pub submit_msg: Option<String>,
    pub on_change: String,
}
impl Input {
    pub fn new<S: Into<String>>(placeholder: S, message: S, style: StyleSheet) -> Self {
        Self {
            position: Position { x: 0.0, y: 0.0 },
            size: Size { x: 0.0, y: 0.0 },

            style,

            placeholder: placeholder.into(),
            text: String::new(),

            selected: false,
            is_password: false,
            block_tab: false,
            cursor: true,
            submit_msg: None,
            on_change: message.into(),
        }
    }

    pub fn on_submit<S: Into<String>>(mut self, msg: S) -> Self {
        self.submit_msg = Some(msg.into());
        self
    }

    pub fn with_tab_blocker(mut self, block: bool) -> Self {
        self.block_tab = block;
        self
    }

    pub fn with_cursor(mut self, cursor: bool) -> Self {
        self.block_tab = cursor;
        self
    }

    pub fn with_password(mut self) -> Self {
        self.is_password = true;
        self
    }

    pub fn set_text(&mut self, text: &str) {
        let height = text.matches('\n').count() + 1;
        let width = text
            .split('\n')
            .into_iter()
            .map(|x| x.len())
            .max()
            .unwrap_or(0);

        self.size = Size {
            x: width as f32,
            y: height as f32,
        };
        self.text = String::from(text);
    }
}

impl Widget for Input {
    fn to_char_array(&self) -> Vec<Vec<char>> {
        let mut buffer: Vec<Vec<char>> =
            vec![vec![' '; self.size.x as usize]; self.size.y as usize];

        let chars: Vec<char> = if !self.text.is_empty() {
            self.text.chars().collect()
        } else {
            self.placeholder.chars().collect()
        };

        let mut x: usize = 0;
        let mut y: usize = 0;
        for char in chars {
            match char {
                '\n' => {
                    y += 1;
                    x = 0;
                }
                '\t' => {
                    x += TAB_WIDTH;
                }
                char => {
                    if (x as f32) < self.size.x.floor() && (y as f32) < self.size.y.floor() {
                        if !self.is_password {
                            buffer[y][x] = char;
                        } else {
                            buffer[y][x] = '*';
                        }
                    }
                    x += 1;
                }
            }
        }

        // cursor
        if self.is_selected() && self.cursor && buffer.len() > y && buffer[y].len() > x {
            buffer[y][x] = '▉';
        }

        buffer
    }

    fn handle_input_event(&mut self, event: Event) -> Vec<Vec<String>> {
        match event {
            Event::Key(_modifier, code) => match code {
                KeyCode::Enter => {
                    if let Some(msg) = self.submit_msg.clone() {
                        return vec![vec![String::from(msg)]];
                    } else {
                        self.text.push('\n');

                        return vec![vec![String::from(self.on_change.clone()), String::from(&self.text)]];
                    }
                }
                KeyCode::Tab => {
                    if !self.block_tab {
                        self.text.push('\t');

                        return vec![vec![String::from(self.on_change.clone()), String::from(&self.text)]];
                    }
                }
                KeyCode::Char(c) => {
                    self.text.push(c);

                    return vec![vec![String::from(self.on_change.clone()), String::from(&self.text)]];
                }
                KeyCode::Backspace => {
                    self.text.pop();

                    return vec![vec![String::from(self.on_change.clone()), String::from(&self.text)]];
                }
                _ => (),
            },
            _ => (),
        }

        vec![]
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
    fn select(&mut self, selected: bool) {
        self.selected = selected
    }
}

impl Style for Input {
    fn get_style(&self) -> StyleSheet {
        self.style
    }

    fn set_style(&mut self, style: StyleSheet) {
        self.style = style;
    }

    fn apply_style(&mut self) {
    }
}
