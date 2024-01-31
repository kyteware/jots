use std::sync::Arc;

use iced::{
    theme::Checkbox, widget::{self, column, text_editor::{self, Content}, Column, TextEditor}, Command, Element
};

use jotdown::{JdElement, JdTextMod};

use crate::app::Message;

pub enum EditorMode {
    Note(Option<NoteContents>),
}

impl EditorMode {
    pub fn new() -> Self {
        Self::Note(None)
    }

    pub fn view<'a>(&'a self) -> iced::Element<'a, Message> {
        match self {
            Self::Note(Some(contents)) => {
                let mut col = Column::new();
                for content in &contents.items {
                    col = col.push(content.view());
                }
                col.into()
            }
            _ => widget::text("no note :(").into()
        }
    }

    pub fn update(&mut self, msg: EditorMessage) -> Command<Message> {
        match msg {
            EditorMessage::TextEdited(action) => {
                #[allow(irrefutable_let_patterns)]
                // if let Self::Note { content } = self {
                //     content.as_mut().unwrap().perform(action);
                // }
                Command::none()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum EditorMessage {
    TextEdited(text_editor::Action),
}

pub struct NoteContents {
    pub items: Vec<NoteItem>
}

impl <'a> From<&Vec<JdElement<'a>>> for NoteContents {
    fn from(elements: &Vec<JdElement<'a>>) -> Self {
        let mut contents = vec![];
        for element in elements {
            contents.push(NoteItem::from(element));
        }
        NoteContents { items: contents }
    }
}

pub enum NoteItem {
    Text(Content),
    // Checklist(Vec<Checkbox>)

}

impl<'a> From<&JdElement<'a>> for NoteItem {
    fn from(element: &JdElement) -> Self {
        match element {
            // JdElement::TitleOrHeading(_) => todo!(),
            // JdElement::CodeBlock(_) => todo!(),
            // JdElement::Checklist(_) => todo!(),
            // JdElement::UnorderedList(_) => todo!(),
            // JdElement::OrderedList(_) => todo!(),
            // JdElement::Embed(_) => todo!(),
            JdElement::Paragraph(mods) => {
                let mut content = Content::new();
                for text_mod in mods {
                    match text_mod {
                        JdTextMod::Normal(text) => content.perform(text_editor::Action::Edit(text_editor::Edit::Paste(Arc::new(text.to_string())))),
                        _ => todo!()
                    }
                }
                NoteItem::Text(content)
            },
            _ => todo!()
        }
    }
}

impl NoteItem {
    fn view(&self) -> Element<Message> {
        match self {
            NoteItem::Text(content) => TextEditor::new(content).shrink_to_content(true).into(),
        }
    }
}
