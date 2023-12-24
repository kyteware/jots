use std::sync::Arc;

use crate::{
    error::Error,
    fs::load_file,
    sidebar::{NoteHeader, SidebarMode},
};

use iced::{executor, Application, Command, Theme};

pub struct App {
    sidebar_mode: SidebarMode,
}

#[derive(Debug, Clone)]
pub enum Message {
    NotesLoaded(Result<Vec<NoteHeader>, Error>),
    OpenNote(NoteHeader),
    NoteOpened(Result<(NoteHeader, Arc<String>), Error>),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (sidebar_mode, note_load) = SidebarMode::load();
        (Self { sidebar_mode }, note_load)
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::NotesLoaded(Ok(notes)) => {
                self.sidebar_mode = SidebarMode::Notes { notes };
                Command::none()
            }
            Message::OpenNote(note) => Command::perform(load_file(note.path.clone()), |res| {
                Message::NoteOpened(if let Ok(contents) = res {
                    Ok((note, contents))
                } else {
                    Err(res.unwrap_err())
                })
            }),
            Message::NoteOpened(Ok((note, contents))) => {
                dbg!(note, contents);
                Command::none()
            }

            // TODO: Handle errors
            Message::NotesLoaded(Err(e)) => {
                eprintln!("Error loading notes: {}", e);
                Command::none()
            }
            Message::NoteOpened(Err(e)) => {
                eprintln!("Error opening note: {}", e);
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let sidebar = self.sidebar_mode.view();
        sidebar.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
