use std::rc::Rc;
use iced_widget::{style::Theme, core::{Color, Background}};

pub trait StyleSheet {
    type Style: Default + Clone;
    fn style(&self, style: &Self::Style) -> Appearance;
}

#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    pub text_color: Color,
    pub background: Background,
    pub hovered_text_color: Color,
    pub hovered_background: Background,
    pub selected_text_color: Color,
    pub selected_background: Background,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            text_color: Color::BLACK,
            background: Background::Color([0.87, 0.87, 0.87].into()),
            hovered_text_color: Color::WHITE,
            hovered_background: Background::Color([0.0, 0.5, 1.0].into()),
            selected_text_color: Color::WHITE,
            selected_background: Background::Color([0.2, 0.5, 0.8].into()),
        }
    }
}

#[derive(Clone, Default)]
pub enum TreeListStyles {
    #[default]
    Default,
    Custom(Rc<dyn StyleSheet<Style = Theme>>),
}

impl TreeListStyles {
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Rc::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = TreeListStyles;
    fn style(&self, style: &Self::Style) -> Appearance {
        if let TreeListStyles::Custom(custom) = style {
            return custom.style(self);
        }

        let palette = self.extended_palette();
        let foreground = self.palette();

        Appearance {
            text_color: foreground.text,
            background: palette.background.base.color.into(),
            hovered_text_color: palette.primary.weak.text,
            hovered_background: palette.primary.weak.color.into(),
            selected_text_color: palette.primary.strong.text,
            selected_background: palette.primary.strong.color.into(),
        }
    }
}