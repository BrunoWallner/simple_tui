use super::{Widget, TAB_WIDTH};
use crate::events::{Event, MouseEvent, MouseEventKind};
use crate::ui::style::{Style, StyleSheet};
use crate::{Position, Size};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text<'a> {
    text: String, // should only be changed with self.set_text because of dimensions

    pub position: Position,
    pub size: Size,

    pub style: StyleSheet,

    pub onclick_message: Option<&'a str>,
    pub selected: bool,
}

impl<'a> Text<'a> {
    pub fn new(text: &str, style: StyleSheet) -> Self {
        let height = text.matches('\n').count() + 1;
        let width = text
            .split('\n')
            .into_iter()
            .map(|x| x.len())
            .max()
            .unwrap_or(0);

        Self {
            text: String::from(text),

            position: Position { x: 0.0, y: 0.0 },
            size: Size {
                x: width as f32,
                y: height as f32,
            },

            style,

            onclick_message: None,
            selected: false,
        }
    }

    pub fn on_click(mut self, message: &'a str) -> Self {
        self.onclick_message = Some(message);
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

    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl<'a> Widget<'a> for Text<'a> {
    fn to_char_array(&self) -> Vec<Vec<char>> {
        let mut buffer: Vec<Vec<char>> =
            vec![vec![' '; self.size.x as usize]; self.size.y as usize];

        let chars: Vec<char> = self.text.chars().collect();

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
                    if (x as f32) < self.size.x && (y as f32) < self.size.y {
                        buffer[y][x] = char;
                    }
                    x += 1;
                }
            }
        }

        buffer
    }

    fn handle_input_event(&mut self, event: Event) -> Vec<Vec<String>> {
        match event {
            Event::Mouse(event) => match event {
                MouseEvent { kind, .. } => {
                    match kind {
                        MouseEventKind::Down(_) => {
                            if let Some(message) = self.onclick_message {
                                return vec![vec![String::from(message)]];
                            }
                            // DEBUG !!!
                            self.set_text("clicked");
                        }
                        _ => (),
                    }
                }
            },
            _ => (),
        }
        Vec::new()
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

impl<'a> Style for Text<'a> {
    fn get_style(&self) -> StyleSheet {
        self.style
    }

    fn set_style(&mut self, style: StyleSheet) {
        self.style = style;
    }

    fn apply_style(&mut self) {
        todo!();
    }
}
