use super::{Widget, TAB_WIDTH};
use crate::events::{Event, KeyCode, MouseEvent, MouseEventKind};
use crate::ui::style::{Style, StyleSheet};
use crate::{Position, Size};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Button {
    pub position: Position,
    pub size: Size,

    style: StyleSheet,

    pub text: String,
    pub onclick_message: Option<String>,
    pub selected: bool,
}
impl Button {
    pub fn new<S: Into<String>>(text: S, style: StyleSheet) -> Self {
        Self {
            position: Position { x: 0.0, y: 0.0 },
            size: Size { x: 0.0, y: 0.0 },

            style,

            text: text.into(),
            onclick_message: None,
            selected: false,
        }
    }

    pub fn on_click<S: Into<String>>(mut self, message: S) -> Self {
        self.onclick_message = Some(message.into());
        self
    }
}

impl Widget for Button {
    fn to_char_array(&self) -> Vec<Vec<char>> {
        let mut buffer: Vec<Vec<char>> =
            vec![vec![' '; self.size.x as usize]; self.size.y as usize];

        // draws border
        for y in 0..self.size.y as usize {
            for x in 0..self.size.x as usize {
                if y == 0
                    || y == self.size.y as usize - 1
                    || x == 0
                    || x == self.size.x as usize - 1
                {
                    buffer[y as usize][x as usize] = if self.selected { '#' } else { '+' }
                }
            }
        }

        let chars: Vec<char> = self.text.chars().collect();

        let mut x: usize = 1;
        let mut y: usize = 1;
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
                    if (x as f32) < self.size.x - 1.0 && (y as f32) < self.size.y - 1.0 {
                        buffer[y][x] = char;
                    }
                    x += 1;
                }
            }
        }

        buffer
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

    fn handle_input_event(&mut self, event: Event) -> Vec<Vec<String>> {
        match event {
            Event::Mouse(event) => match event {
                MouseEvent { kind, .. } => match kind {
                    MouseEventKind::Down(_) => {
                        if let Some(message) = self.onclick_message.clone() {
                            return vec![vec![String::from(message)]];
                        }
                    }
                    _ => (),
                },
            },
            Event::Key(_modifier, code) => match code {
                KeyCode::Enter => {
                    if let Some(msg) = self.onclick_message.clone() {
                        return vec![vec![String::from(msg)]];
                    }
                }
                _ => (),
            },
            _ => (),
        }
        Vec::new()
    }

    fn is_selected(&self) -> bool {
        self.selected
    }

    fn select(&mut self, selected: bool) {
        self.selected = selected
    }
}

impl Style for Button {
    fn get_style(&self) -> StyleSheet {
        self.style
    }

    fn set_style(&mut self, style: StyleSheet) {
        self.style = style;
    }

    fn apply_style(&mut self) {
        //todo!();
    }
}
