pub mod button;
pub mod container;
pub mod input;
pub mod text;

pub const TAB_WIDTH: usize = 4;

use crate::events::Event;
use crate::{Position, Size};
use crate::ui::style::{Style, StyleSheet};

pub trait Msg<M> {
    fn to_msg(field: String) -> M;
}

pub trait Widget {
    fn to_char_array(&self) -> Vec<Vec<char>>;
    fn handle_input_event(&mut self, event: Event) -> Vec<Vec<String>>;

    fn get_position(&self) -> Position;
    fn get_size(&self) -> Size;

    fn set_position(&mut self, position: Position);
    fn set_size(&mut self, size: Size);

    fn is_selected(&self) -> bool;
    fn select(&mut self, selected: bool);
}

// very dirty but best for end user i think

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Widgets {
    Text(text::Text),
    Button(button::Button),
    Container(container::Container),
    Input(input::Input),
}
impl Widget for Widgets{
    fn to_char_array(&self) -> Vec<Vec<char>> {
        match self {
            Widgets::Text(text) => text.to_char_array(),
            Widgets::Button(button) => button.to_char_array(),
            Widgets::Container(container) => container.to_char_array(),
            Widgets::Input(input) => input.to_char_array(),
        }
    }
    fn handle_input_event(&mut self, event: Event) -> Vec<Vec<String>> {
        match self {
            Widgets::Text(text) => text.handle_input_event(event),
            Widgets::Button(button) => button.handle_input_event(event),
            Widgets::Container(container) => container.handle_input_event(event),
            Widgets::Input(input) => input.handle_input_event(event),
        }
    }

    fn get_position(&self) -> Position {
        match self {
            Widgets::Text(text) => text.get_position(),
            Widgets::Button(button) => button.get_position(),
            Widgets::Container(container) => container.get_position(),
            Widgets::Input(input) => input.get_position(),
        }
    }
    fn get_size(&self) -> Size {
        match self {
            Widgets::Text(text) => text.get_size(),
            Widgets::Button(button) => button.get_size(),
            Widgets::Container(container) => container.get_size(),
            Widgets::Input(input) => input.get_size(),
        }
    }

    fn set_position(&mut self, position: Position) {
        match self {
            Widgets::Text(text) => text.set_position(position),
            Widgets::Button(button) => button.set_position(position),
            Widgets::Container(container) => container.set_position(position),
            Widgets::Input(input) => input.set_position(position),
        }
    }
    fn set_size(&mut self, size: Size) {
        match self {
            Widgets::Text(text) => text.set_size(size),
            Widgets::Button(button) => button.set_size(size),
            Widgets::Container(container) => container.set_size(size),
            Widgets::Input(input) => input.set_size(size),
        }
    }

    fn is_selected(&self) -> bool {
        match self {
            Widgets::Text(text) => text.is_selected(),
            Widgets::Button(button) => button.is_selected(),
            Widgets::Container(container) => container.is_selected(),
            Widgets::Input(input) => input.is_selected(),
        }
    }

    fn select(&mut self, selected: bool) {
        match self {
            Widgets::Text(text) => text.select(selected),
            Widgets::Button(button) => button.select(selected),
            Widgets::Container(container) => container.select(selected),
            Widgets::Input(input) => input.select(selected),
        }
    }
}

impl Style for Widgets {
    fn get_style(&self) -> StyleSheet {
        match self {
            Widgets::Text(text) => text.get_style(),
            Widgets::Button(button) => button.get_style(),
            Widgets::Container(container) => container.get_style(),
            Widgets::Input(input) => input.get_style(),
        }    
    }

    fn set_style(&mut self, style: StyleSheet) {
        match self {
            Widgets::Text(text) => text.set_style(style),
            Widgets::Button(button) => button.set_style(style),
            Widgets::Container(container) => container.set_style(style),
            Widgets::Input(input) => input.set_style(style),
        }
    }

    fn apply_style(&mut self) {
        match self {
            Widgets::Text(text) => text.apply_style(),
            Widgets::Button(button) => button.apply_style(),
            Widgets::Container(container) => container.apply_style(),
            Widgets::Input(input) => input.apply_style(),
        }    
    }

}
