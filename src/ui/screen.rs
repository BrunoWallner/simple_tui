use super::widget::container::Container;
use super::widget::Widget;
use crate::events::{Event, MouseEvent};
use crossterm::{cursor, event, queue, terminal};
use std::io::{stdout, Result, Stdout, Write};

use crate::{Position, Size};
use crate::ui::style::Style;

use std::marker::PhantomData;

pub struct Screen<'a> {
    stdout: Stdout,
    pub width: usize,
    pub height: usize,
    pub canvas: Vec<Vec<char>>,
    pub container: Container<'a>,
    pub messages: Vec<Vec<String>>,
    previos_lines: Vec<String>, // for render optimisation
    _phantom_data: PhantomData<&'a str>,
}
impl<'a> Screen<'a> {
    pub fn new(mut container: Container<'a>) -> Result<Self> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()?;

        let (cols, rows) = terminal::size()?;

        container.set_size(Size {
            x: cols as f32,
            y: rows as f32,
        });
        container.set_position(Position { x: 0.0, y: 0.0 });
        container.apply_style();

        queue!(
            stdout,
            cursor::Hide,
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture
        )?;
        stdout.flush()?;

        Ok(Self {
            stdout,
            width: cols as usize,
            height: rows as usize,
            canvas: vec![vec![' '; cols as usize]; rows as usize],
            container,
            messages: Vec::new(),
            previos_lines: vec![String::new(); rows as usize],
            _phantom_data: PhantomData,
        })
    }

    pub fn handle_event(&mut self, event: Event) -> Container<'a> {
        match event {
            Event::Resize((x, y)) => self.resize(x as usize, y as usize),
            Event::Key(modifier, key) => {
                if self.container.is_selected() {
                    let mut msg = self
                        .container
                        .handle_input_event(Event::Key(modifier, key));
                    self.messages.append(&mut msg);
                }
            }

            // alway handle mouse and transform coords
            Event::Mouse(event) => {
                let (mouse_x, mouse_y) = (event.column as u32, event.row as u32);
                //let widget = *widget;
                let widget_pos = self.container.get_position();
                let widget_size = self.container.get_size();

                let mouse_x = mouse_x as f32;
                let mouse_y = mouse_y as f32;

                if mouse_x <= widget_pos.x + widget_size.x
                    && mouse_x >= widget_pos.x
                    && mouse_y <= widget_pos.y + widget_size.y
                    && mouse_y >= widget_pos.y
                {
                    let mut msg = self.container.handle_input_event(Event::Mouse(MouseEvent {
                        column: (mouse_x - widget_pos.x) as u16,
                        row: (mouse_y - widget_pos.y) as u16,
                        ..event
                    }));

                    self.messages.append(&mut msg);
                }
            }
        }
        self.container.clone()
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.canvas = vec![vec![' '; width]; height];
        self.previos_lines = vec![String::new(); height];
        self.width = width;
        self.height = height;

        self.container.set_size(Size {
            x: width as f32,
            y: height as f32,
        });
    }

    pub fn draw(&mut self) {
        self.container.apply_style();
        // actual drawing to canvas
        let buffer = self.container.to_char_array();
        let pos = self.container.get_position();
        let size = self.container.get_size();
        for y in 0..size.y as usize {
            for x in 0..size.x as usize {
                let canvas_x = x + pos.x as usize;
                let canvas_y = y + pos.y as usize;

                if self.width > canvas_x && self.height > canvas_y {
                    self.canvas[canvas_y][canvas_x] = buffer[y][x]
                }
            }
        }
    }

    pub fn get_message(&mut self) -> Option<Vec<String>> {
        self.messages.pop()
    }

    pub fn render(&mut self) -> Result<()> {
        let mut lines: Vec<String> = Vec::new();
        for row in self.canvas.iter() {
            let mut line: String = String::new();
            for c in row {
                match c {
                    '\t' | '\n' => line.push(' '), // unsupported rn
                    c => line.push(*c),
                }
            }
            lines.push(line);
        }

        for (row, line) in lines.iter().enumerate() {
            if line != &self.previos_lines[row] {
                queue!(self.stdout, cursor::MoveTo(0, row as u16))?;
                write!(self.stdout, "{}", line)?;
            }
        }
        self.stdout.flush()?;
        self.previos_lines = lines;

        Ok(())
    }

    pub fn clear(&mut self) {
        self.canvas = vec![vec![' '; self.width]; self.height];
    }

    pub fn exit(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        queue!(
            self.stdout,
            cursor::Show,
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )?;
        self.stdout.flush()?;

        Ok(())
    }
}
