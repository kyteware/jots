use iced::{advanced::{layout::{Limits, Node}, renderer::{Quad, Style}, widget::{tree, Tree}, Layout, Renderer as _Renderer, Widget}, mouse::Cursor, Background, Border, Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};
use petgraph::graph::{DiGraph, NodeIndex};

// pub type OnMoveFunc<Message> = Box<dyn Fn((f64, f64)) -> Message>;

pub struct GoalTree<'a> {
    graph: &'a DiGraph<GraphNode, ()>,
}

#[derive(Debug, Default)]
pub struct GraphNode {
    pub name: String,
    pub complete: bool,
    pub coords: (f64, f64)
}

pub fn goal_tree<'a>(graph: &'a DiGraph<GraphNode, ()>) -> GoalTree<'a> {
    GoalTree { graph }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for GoalTree<'a>
where
    Renderer: _Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &Limits,
    ) -> Node {
        Node::new(Size::new(0., 0.))
    }

    // fn on_event(
    //     &mut self,
    //     tree: &mut Tree,
    //     event: cosmic::iced_core::Event,
    //     layout: Layout<'_>,
    //     cursor: mouse::Cursor,
    //     _renderer: &Renderer,
    //     _clipboard: &mut dyn Clipboard,
    //     shell: &mut Shell<'_, Message>,
    //     _viewport: &Rectangle,
    // ) -> event::Status {
    //     let bounds = layout.bounds();

    //     match event {
    //         core::Event::Mouse(mouse::Event::CursorMoved { .. })
    //         | core::Event::Touch(touch::Event::FingerMoved { .. }) => {
    //             if let Some(position) = cursor.position() {
    //                 let state = tree.state.downcast_mut::<State>();
    //                 if let Some((output_key, region)) = state.dragging.as_mut() {
    //                     update_dragged_region(
    //                         self.tab_model,
    //                         self.list,
    //                         &bounds,
    //                         *output_key,
    //                         region,
    //                         state.max_dimensions,
    //                         (position.x - state.offset.0, position.y - state.offset.1),
    //                     );

    //                     return event::Status::Captured;
    //                 }
    //             }
    //         }

    //         core::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
    //         | core::Event::Touch(touch::Event::FingerPressed { .. }) => {
    //             if let Some(position) = cursor.position() {
    //                 let state = tree.state.downcast_mut::<State>();
    //                 if let Some((output_key, output_region)) = display_region_hovers(
    //                     self.tab_model,
    //                     self.list,
    //                     &bounds,
    //                     state.max_dimensions,
    //                     position,
    //                 ) {
    //                     state.drag_from = position;
    //                     state.offset = (position.x - output_region.x, position.y - output_region.y);
    //                     state.dragging = Some((output_key, output_region));
    //                     return event::Status::Captured;
    //                 }
    //             }
    //         }

    //         core::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
    //         | core::Event::Touch(touch::Event::FingerLifted { .. }) => {
    //             let state = tree.state.downcast_mut::<State>();
    //             if let Some((output_key, region)) = state.dragging.take() {
    //                 if let Some(position) = cursor.position() {
    //                     if position.distance(state.drag_from) < 4.0 {
    //                         if let Some(ref on_select) = self.on_select {
    //                             for id in self.tab_model.iter() {
    //                                 if let Some(&key) = self.tab_model.data::<OutputKey>(id) {
    //                                     if key == output_key {
    //                                         shell.publish(on_select(id));
    //                                     }
    //                                 }
    //                             }
    //                         }

    //                         return event::Status::Captured;
    //                     }
    //                 }

    //                 if let Some(ref on_placement) = self.on_placement {
    //                     shell.publish(on_placement(
    //                         output_key,
    //                         ((region.x - state.max_dimensions.0 - bounds.x) * UNIT_PIXELS) as i32,
    //                         ((region.y - state.max_dimensions.1 - bounds.y) * UNIT_PIXELS) as i32,
    //                     ));
    //                 }

    //                 return event::Status::Captured;
    //             }
    //         }

    //         _ => (),
    //     }

    //     event::Status::Ignored
    // }

    // fn mouse_interaction(
    //     &self,
    //     tree: &Tree,
    //     layout: Layout<'_>,
    //     cursor: mouse::Cursor,
    //     _viewport: &Rectangle,
    //     _renderer: &Renderer,
    // ) -> mouse::Interaction {
    //     let state = tree.state.downcast_ref::<State>();
    //     let bounds = layout.bounds();

    //     for (_output_key, region) in
    //         display_regions(self.tab_model, self.list, &bounds, state.max_dimensions)
    //     {
    //         if cursor.is_over(region) {
    //             return mouse::Interaction::Grab;
    //         }
    //     }

    //     mouse::Interaction::Idle
    // }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

            renderer.fill_quad(
                Quad {
                    bounds: Rectangle { x: 10., y: 10., width: 10., height: 10. },
                    border: Border {
                        color: Color::from_rgb(0.25, 0.25, 0.25),
                        radius: 4.0.into(),
                        width: 3.0,
                    },
                    shadow: Default::default(),
                },
                Background::Color(Color::from_rgb(0.5, 0.5, 0.5)),
            );
            // core::text::Renderer::fill_text(
            //     renderer,
            //     core::Text {
            //         content: itoa::Buffer::new().format(id),
            //         size: core::Pixels(24.0),
            //         line_height: core::text::LineHeight::Relative(1.2),
            //         font: cosmic::font::FONT_BOLD,
            //         bounds: id_bounds.size(),
            //         horizontal_alignment: alignment::Horizontal::Center,
            //         vertical_alignment: alignment::Vertical::Center,
            //         shaping: text::Shaping::Basic,
            //     },
            //     core::Point {
            //         x: id_bounds.center_x(),
            //         y: id_bounds.center_y(),
            //     },
            //     cosmic_theme.palette.neutral_10.into(),
            //     *viewport,
            // );
    }
}

#[derive(Default)]
struct State {
    drag_from: Point,
    dragging: Option<NodeIndex>,
    offset: (f32, f32),
    max_dimensions: (f32, f32),
}

impl<'a, Message, Theme, Renderer> From<GoalTree<'a>> 
    for Element<'a, Message, Theme, Renderer> 
where
    Renderer: _Renderer
{
    fn from(goal_tree: GoalTree<'a>) -> Self {
        Self::new(goal_tree)
    }
}
