use std::{time::Duration};

use simple_tui::{
    message::Message,
    events::{
        Event,
    },
    ui::{
        style::{StyleSheet, Style, Length},
        screen::Screen,
        widget::{Widgets, container::{Container, ContainerKind}, input::Input, button::Button}
    },
    application::Application,
};


fn main() {
    let mut application = State::new();
    application.run();
}

struct State<'a> {
    screen: Screen<'a>,

    container: Container<'a>,

    exit: bool,
}
impl<'a> Application for State<'a> {
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
            _ => (),
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

fn build_ui<'a>() -> Container<'a> {
    let main_container = Container::new(
        ContainerKind::Column,
        vec![
            // exit button
            Widgets::Button(Button::new(
                "exit",
                StyleSheet {
                    width: Length::Absolute(6.0),
                    height: Length::Absolute(3.0),
                    padding: (0.0, 0.0),
                    ..Default::default()
                }
            ).on_click("exit")),

            Widgets::Container(Container::new(
                ContainerKind::Row,
                vec![
                    Widgets::Input(Input::new(
                        "input: ",
                        "input_change",
                        StyleSheet {
                            width: Length::Relative(0.33),
                            height: Length::Relative(0.33),
                            ..Default::default()
                        }
                    ).with_tab_blocker(true)),
                    Widgets::Input(Input::new(
                        "input: ",
                        "input_change",
                        StyleSheet {
                            width: Length::Relative(0.5),
                            height: Length::Relative(0.5),
                            ..Default::default()
                        }
                    ).with_tab_blocker(true)),
                    Widgets::Input(Input::new(
                        "input: ",
                        "input_change",
                        StyleSheet {
                            width: Length::Relative(1.0),
                            height: Length::Relative(1.0),
                            ..Default::default()
                        }
                    ).with_tab_blocker(true))
                ],
                StyleSheet {
                    width: Length::Relative(1.0),
                    height: Length::Relative(1.0),
                    padding: (0.0, 0.0),
                    ..Default::default()
                }
            )
            .with_default_borders()
            .with_tab_selector())
        ],
        StyleSheet::default(),
    )
    .with_default_borders()
    .with_tab_selector()
    .always_selected();

    main_container
}