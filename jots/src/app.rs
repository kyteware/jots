use std::{io, sync::Arc, fmt::Display, fmt::{Formatter, self}};

use crate::error::Error;

use iced::{
    executor,
    widget::{self, horizontal_space, text_editor},
    Application, Command, Length, Theme,
};

use crate::fs::{load_file, pick_file};

pub struct App {
    content: text_editor::Content,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    Open,
    FileOpened(Result<Arc<String>, Error>),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                content: text_editor::Content::new(),
                error: None,
            },
            Command::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Message::FileOpened,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
                Command::none()
            }
            Message::FileOpened(Ok(content)) => {
                self.content = text_editor::Content::with(&content);
                Command::none()
            }
            Message::FileOpened(Err(e)) => {
                self.error = Some(e);
                Command::none()
            }
            Message::Open => Command::perform(pick_file(), Message::FileOpened),
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let controls = widget::row![widget::button("Open").on_press(Message::Open)];
        let input = text_editor(&self.content).on_edit(Message::Edit);

        let position = {
            let (line, column) = self.content.cursor_position();

            widget::Text::new(format!("{}:{}", line + 1, column + 1))
        };

        let status_bar = widget::row!(horizontal_space(Length::Fill), position);

        widget::container(widget::column![controls, input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
