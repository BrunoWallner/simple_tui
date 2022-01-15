use super::{Widget, Widgets};
use crate::events::{Event, KeyCode, MouseEvent};
use crate::ui::style::{Style, StyleSheet, Length};
use crate::{Position, Size};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContainerKind {
    Row,
    Column,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Container {
    pub position: Position,
    pub size: Size,

    pub style: StyleSheet,
    pub kind: ContainerKind,

    pub content: Vec<Widgets>,
    pub borders: [char; 12],

    pub selected: bool,
    pub selection_index: usize,
    pub tab_selector: bool,
    pub always_selected: bool,
}
impl Container {
    pub fn new(kind: ContainerKind, content: Vec<Widgets>, style: StyleSheet) -> Self {
        Self {
            position: Position { x: 0.0, y: 0.0 },
            size: Size { x: 0.0, y: 0.0 },

            style,
            kind,

            content,
            borders: [' '; 12],

            selected: false,
            selection_index: 0,
            tab_selector: false,
            always_selected: false,
        }
    }

    pub fn push(&mut self, widget: Widgets) {
        self.content.push(widget)
    }

    pub fn with_tab_selector(mut self) -> Self {
        self.tab_selector = true;
        self
    }

    pub fn always_selected(mut self) -> Self {
        self.always_selected = true;
        self
    }

    pub fn with_default_borders(mut self) -> Self {
        let borders = ['┌', '┐', '┘', '└', '─', '│', '.', '.', '.', '.', '.', ':'];
        self.borders = borders;
        self
    }

    pub fn with_borders(mut self, borders: [char; 12]) -> Self {
        self.borders = borders;
        self
    }

    pub fn modify_element<F>(&mut self, index: usize, mut task: F)
    where
        F: FnMut(Widgets) -> Widgets,
    {
        let content = self.content[index].clone();
        self.content[index] = task(content);
    }
}

impl Widget for Container {
    fn to_char_array(&self) -> Vec<Vec<char>> {
        if self.size.x >= 1.0 && self.size.y >= 1.0 {
            let mut array: Vec<Vec<char>> =
                vec![vec![' '; self.size.x as usize]; self.size.y as usize];
            for c in &self.content {
                let arr = c.to_char_array();
                let pos = c.get_position();
                let size = c.get_size();
                for y in 0..size.y as usize {
                    for x in 0..size.x as usize {
                        let array_x = x + pos.x as usize;
                        let array_y = y + pos.y as usize;

                        if self.size.x.floor() > array_x as f32
                            && self.size.y.floor() > array_y as f32
                        {
                            array[array_y][array_x] = arr[y][x]
                        }
                    }
                }
            }

            // draws border
            if self.borders != [' '; 12] {
                let x_edge = array[0].len() - 1;
                let y_edge = array.len() - 1;
    
                if self.selected || self.always_selected {
                    // upper and bottom line
                    for x in 0..x_edge {
                        array[0][x] = self.borders[4];
                        array[y_edge][x] = self.borders[4]
                    }
    
                    // right and left line
                    for y in 0..y_edge {
                        array[y][x_edge] = self.borders[5];
                        array[y][0] = self.borders[5];
                    }
    
                    array[0][0] = self.borders[0]; // upper left border
                    array[0][x_edge] = self.borders[1]; // upper right border
                    array[y_edge][x_edge] = self.borders[2]; // bottom right
                    array[y_edge][0] = self.borders[3]; // bottom left
                } else {
                    let offset: usize = 6;
                    // upper and bottom line
                    for x in 0..x_edge {
                        array[0][x] = self.borders[4 + offset];
                        array[y_edge][x] = self.borders[4 + offset]
                    }
    
                    // right and left line
                    for y in 0..y_edge {
                        array[y][x_edge] = self.borders[5 + offset];
                        array[y][0] = self.borders[5 + offset];
                    }
    
                    array[0][0] = self.borders[0 + offset]; // upper left border
                    array[0][x_edge] = self.borders[1 + offset]; // upper right border
                    array[y_edge][x_edge] = self.borders[2 + offset]; // bottom right
                    array[y_edge][0] = self.borders[3 + offset]; // bottom left
                }
            }

            array
        } else {
            Vec::new()
        }
    }
    fn handle_input_event(&mut self, event: Event) -> Vec<Vec<String>> {
        let mut messages: Vec<Vec<String>> = Vec::new();

        match event {
            Event::Resize(_) => (),
            Event::Key(modifier, key) => {
                // sends key event to every selected widget
                let mut other_container_tab_selecting: bool = false;
                for widget in self.content.iter_mut() {
                    if widget.is_selected() {
                        let mut msg =
                            widget.handle_input_event(Event::Key(modifier.clone(), key.clone()));
                        messages.append(&mut msg);

                        match widget {
                            Widgets::Container(c) => {
                                if c.tab_selector && c.is_selected() {
                                    other_container_tab_selecting = true
                                }
                            }
                            _ => (),
                        }
                    }
                }
                // tab selector
                if key == KeyCode::Tab && self.tab_selector && !other_container_tab_selecting {
                    self.selection_index += 1;
                    if self.selection_index >= self.content.len() {
                        self.selection_index = 0;
                        self.select(false);
                    }
                    // deselection
                    for c in self.content.iter_mut() {
                        c.select(false);
                    }
                    // selection
                    self.content[self.selection_index].select(true);
                }
            }

            // transforms coordinates for each content widget
            Event::Mouse(event) => {
                let (mouse_x, mouse_y) = (event.column as u32, event.row as u32);
                for widget in self.content.iter_mut() {
                    //let widget = *widget;
                    let widget_pos = widget.get_position();
                    let widget_size = widget.get_size();

                    let mouse_x = mouse_x as f32;
                    let mouse_y = mouse_y as f32;

                    if mouse_x <= widget_pos.x + widget_size.x
                        && mouse_x >= widget_pos.x
                        && mouse_y <= widget_pos.y + widget_size.y
                        && mouse_y >= widget_pos.y
                    {
                        let mut msg = widget.handle_input_event(Event::Mouse(MouseEvent {
                            column: (mouse_x - widget_pos.x) as u16,
                            row: (mouse_y - widget_pos.y) as u16,
                            ..event.clone()
                        }));

                        messages.append(&mut msg);
                    }
                }
            }
        }

        messages
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
        self.selected || self.always_selected
    }

    fn select(&mut self, selected: bool) {
        self.selected = selected;

        if !self.content.is_empty() {
            self.content[0].select(selected);
        }
    }
}

impl Style for Container {
    fn get_style(&self) -> StyleSheet {
        self.style
    }

    fn set_style(&mut self, style: StyleSheet) {
        self.style = style;
    }

    fn apply_style(&mut self) {
        // applies position and size for every sub content
        let mut x_pos: f32 = 0.0; // starting at 1.0 because of borders
        let mut y_pos: f32 = 0.0;
        for c in self.content.iter_mut() {
            //let pos_x = c.get_style().position.0;
            //let pos_y = c.get_style().position.1;
            /* start padding */
            x_pos += c.get_style().padding.x_start;
            y_pos += c.get_style().padding.y_start;

            let width = c.get_style().width;
            let height = c.get_style().height;

            let width: f32 = match width {
                Length::Absolute(x) => x,
                Length::Relative(x) => {
                    (self.size.x - 0.0 - x_pos) * x - c.get_style().padding.x_end
                }
            };
            let height: f32 = match height {
                Length::Absolute(y) => y,
                Length::Relative(y) => {
                    (self.size.y - 0.0 - y_pos) * y - c.get_style().padding.y_end
                }
            };

            
            let wanted_pos: Position = Position{x: x_pos, y: y_pos};
            let wanted_size: Size = Size{x: width, y: height};

            c.set_position(wanted_pos);
            c.set_size(wanted_size);

            // request style application of every widget
            c.apply_style();

            /* creates room for next widget */
            /* end padding */
            //x_pos += c.get_style().padding.x_end;
            //y_pos += c.get_style().padding.y_end;
            
            match self.kind {
                ContainerKind::Row => {
                    y_pos = 0.0;
                    x_pos += width;
                },  
                ContainerKind::Column => {
                    x_pos = 0.0;
                    y_pos += height;
                }
            }
        }
    }
}
