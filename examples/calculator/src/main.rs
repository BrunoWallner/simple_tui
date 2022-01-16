use std::{time::Duration};

use simple_tui::{
    message::Message,
    events::{
        Event,
    },
    ui::{
        style::{StyleSheet, Style, Length, Padding},
        screen::Screen,
        widget::{Widgets, text::Text, container::{Container, ContainerKind}, button::Button}
    },
    application::Application,
};


fn main() {
    let mut application = State::new();
    application.run();
}

struct State {
    screen: Screen,

    container: Container,

    exit: bool,
}
impl Application for State {
    fn new() -> Self {
        let ui_container = build_ui();
        let screen = Screen::new(ui_container).unwrap();
        let container = screen.container.clone();
        Self {
            screen,

            container: container,

            exit: false,
        }
    }

    fn update(&mut self, msg: Vec<String>) {
        let msg = Message::from_strings(msg);
        match msg.parameter[0].as_str() {
            "exit" => self.exit = true,
            number => {
                self.container.modify_element(
                    0,
                    |mut wid_cont| {
                        match wid_cont {
                            Widgets::Container(ref mut cont) => {
                                cont.modify_element(
                                    1,
                                    |text| {
                                        match text {
                                            Widgets::Text(mut text) => {
                                                text.set_text( &(text.get_text() + number) );
                                                return Widgets::Text(text);
                                            }
                                            other => {
                                                return other;
                                            }
                                        }
                                    }
                                )
                            }
                            other => {
                                return other;
                            }
                        }
                        wid_cont
                    }
                )
            }
        }
    }

    fn view(&mut self) {
        self.screen.container = self.container.clone();
        self.screen.container.apply_style();
    }

    fn run(&mut self) {
        while !self.exit {
            self.screen.clear();
            self.view();
            self.screen.draw();
            self.screen.render().unwrap();

            if let Some(e) = Event::read_non_blocking(Duration::from_secs(1)) {
                let cont = self.screen.handle_event(e.clone());
                self.container = cont.clone();
                let msg = self.screen.get_message();
                if let Some(msg) = msg {
                    self.update(msg);
                }
            }
        }
        self.screen.exit().unwrap();
    }
}

fn build_ui() -> Container {
    let main_container = Container::new(
        ContainerKind::Column,
        vec![
            // exit button and text
            Widgets::Container(Container::new(
                ContainerKind::Row,
                vec![
                    Widgets::Button(Button::new(
                        "exit",
                        StyleSheet {
                            width: Length::Absolute(6.0),
                            height: Length::Absolute(3.0),
                            ..Default::default()
                        }
                    ).on_click("exit")),
                    // text that displays numbers
                    Widgets::Text(Text::new(
                        "",
                        StyleSheet {
                            width: Length::Relative(1.0),
                            height: Length::Relative(1.0),
                            padding: Padding {
                                x_start: 1.0,
                                x_end: 1.0,
                                y_start: 2.0,
                                y_end: 2.0,
                            }
                        }
                    ))
                ],
                StyleSheet {
                    width: Length::Relative(1.0),
                    height: Length::Absolute(5.0),
                    padding: Padding {
                        x_start: 1.0,
                        x_end: 1.0,
                        y_start: 1.0,
                        y_end: 0.0,
                    }
                }
            ).with_default_borders()),
            build_number_row(1),
            build_number_row(2),
            build_number_row(3)
        ],
        StyleSheet::default(),
    )
    .with_tab_selector()
    .always_selected();

    main_container
}

fn build_number_row(row: u8) -> Widgets {
    let offset: u8 = (row - 1) * 3;
    let name_1 = format!("{}", (offset + 1));
    let name_2 = format!("{}", (offset + 2));
    let name_3 = format!("{}", (offset + 3));

    let row = 
        Widgets::Container(Container::new(
            ContainerKind::Row,
            vec![
                Widgets::Button(Button::new(
                    name_1.clone(),
                    get_button_style(1),
                ).on_click(name_1)),
                Widgets::Button(Button::new(
                    name_2.clone(),
                    get_button_style(2),
                ).on_click(name_2)),
                Widgets::Button(Button::new(
                    name_3.clone(),
                    get_button_style(3),
                ).on_click(name_3)),
            ],
            get_button_cont_style(row)
        ).with_tab_selector());
        
    row.clone()
}
fn get_button_style(num: u8) -> StyleSheet {
    StyleSheet {
        width: Length::Relative(1.0 / (4.0 - num as f32)),
        height: Length::Relative(1.0),
        padding: Padding {
            x_start: 0.0,
            x_end: 0.0,
            y_start: 0.0,
            y_end: 0.0
        }
    }
}

fn get_button_cont_style(num: u8) -> StyleSheet {
    StyleSheet {
        width: Length::Relative(1.0),
        height: Length::Relative(1.0 / (4.0 - num as f32)),
        padding: Padding {
            x_start: 0.0,
            x_end: 0.0,
            y_start: 0.0,
            y_end: 0.0
        }
    }
}