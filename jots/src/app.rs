use std::sync::Arc;

use crate::{
    editor::{EditorMessage, EditorMode, NoteContents},
    error::Error,
    fs::load_file,
    sidebar::{NoteHeading, SidebarMode},
};

use iced::{
    executor,
    widget::{self},
    Application, Command, Theme,
};
use jotdown::parse_jotdown;

pub struct App {
    sidebar_mode: SidebarMode,
    editor_mode: EditorMode,
}

#[derive(Debug, Clone)]
pub enum Message {
    NotesLoaded(Result<Vec<NoteHeading>, Error>),
    OpenNote(NoteHeading),
    NoteOpened(Result<(NoteHeading, Arc<String>), Error>),
    Editor(EditorMessage),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (sidebar_mode, note_load) = SidebarMode::load();
        let editor_mode = EditorMode::new();
        (
            Self {
                sidebar_mode,
                editor_mode,
            },
            note_load,
        )
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        dbg!(&message);
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
            Message::NoteOpened(Ok((_note, contents))) => {
                let parsed = parse_jotdown(&contents).unwrap();
                self.editor_mode = EditorMode::Note(Some(NoteContents::from(&parsed.1)));
                Command::none()
            }
            Message::Editor(msg) => self.editor_mode.update(msg),

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
        let editor = self.editor_mode.view();

        let sep = widget::vertical_rule(10);
        let full_content = widget::row![sidebar, sep, editor].spacing(10);
        widget::container(full_content).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
