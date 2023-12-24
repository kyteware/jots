use iced::{
    widget::{self, text_editor},
    Command,
};

use crate::app::Message;

pub enum EditorMode {
    Text {
        content: Option<text_editor::Content>,
    },
}

impl EditorMode {
    pub fn new() -> Self {
        Self::Text { content: None }
    }

    pub fn view<'a>(&'a self) -> iced::Element<'a, Message> {
        match self {
            Self::Text { content } => {
                if let Some(content) = content {
                    text_editor::TextEditor::new(content)
                        .on_edit(|a| Message::Editor(EditorMessage::TextEdited(a)))
                        .into()
                } else {
                    iced::Element::new(widget::text("No note loaded"))
                }
            }
        }
    }

    pub fn update(&mut self, msg: EditorMessage) -> Command<Message> {
        match msg {
            EditorMessage::TextEdited(action) => {
                #[allow(irrefutable_let_patterns)]
                if let Self::Text { content } = self {
                    content.as_mut().unwrap().edit(action);
                }
                Command::none()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum EditorMessage {
    TextEdited(text_editor::Action),
}
