use std::fmt::Display;
use std::hash::Hash;
use iced::font::Font;
use iced_widget::core;

mod style;
mod node;

pub use style::{StyleSheet, Appearance, TreeListStyles};
use node::Node;
pub use node::Node as TreeListNode;

pub struct TreeList<'a, T: 'a, Message, Renderer>
where
    T: Clone + Display + Eq + Hash,
    [T]: ToOwned<Owned = Vec<T>>,
    Renderer: core::Renderer + core::text::Renderer<Font = Font>,
    Renderer::Theme: StyleSheet
{
    pub tree: &'a [Node<T>],
    pub font: Renderer::Font,
    pub style: <Renderer::Theme as StyleSheet>::Style,
    pub on_selected: Box<dyn Fn(usize, T) -> Message>,
    pub padding: f32,
    pub text_size: f32,
    pub selected: Option<usize>,
}

