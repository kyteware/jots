use std::path::PathBuf;

use iced::{widget, Command, Element};

use crate::{app::Message, fs::load_notes};

pub enum SidebarMode {
    Notes { notes: Vec<NoteHeader> },
}

impl SidebarMode {
    pub fn load() -> (Self, Command<Message>) {
        let notes = vec![];
        (
            Self::Notes { notes },
            Command::perform(load_notes(), Message::NotesLoaded),
        )
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        match self {
            Self::Notes { notes } => {
                let buttons = notes
                    .iter()
                    .map(|note| {
                        widget::button(widget::text(note.title.clone()))
                            .on_press(Message::OpenNote(note.clone()))
                            .into()
                    })
                    .collect::<Vec<_>>();
                widget::Column::with_children(buttons).spacing(10).into()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoteHeader {
    pub title: String,
    pub path: PathBuf,
}
