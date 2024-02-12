use iced_core::layout::{self, Layout};
use iced_core::mouse::Cursor;
use iced_core::widget::{self, Widget};
use iced_core::{event, mouse, overlay, Color, Element, Length, Point, Rectangle, Size, Vector};
use iced_core::{renderer, Clipboard, Shell};

use crate::style;

#[derive(Clone, Copy, Debug, Default)]
struct State {
    drag_origin: Option<Point>,
    is_divider_hovered: bool,
}

pub(crate) struct Divider<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: style::StyleSheet,
{
    content: Element<'a, Message, Theme, Renderer>,
    width: f32,
    on_drag: Box<dyn Fn(f32) -> Message + 'a>,
    on_release: Message,
    style: <Theme as style::StyleSheet>::Style,
}

impl<'a, Message, Theme, Renderer> Divider<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: style::StyleSheet,
{
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        width: f32,
        on_drag: impl Fn(f32) -> Message + 'a,
        on_release: Message,
        style: <Theme as style::StyleSheet>::Style,
    ) -> Self {
        Self {
            content: content.into(),
            width,
            on_drag: Box::new(on_drag),
            on_release,
            style,
        }
    }

    fn divider_bounds(&self, bounds: Rectangle) -> Rectangle {
        Rectangle {
            x: bounds.x + bounds.width - self.width,
            width: self.width,
            ..bounds
        }
    }

    fn divider_hover_bounds(&self, bounds: Rectangle) -> Rectangle {
        let mut bounds = self.divider_bounds(bounds);
        // TODO: Configurable
        bounds.x -= 5.0;
        bounds.width += 10.0;

        bounds
    }

    fn is_content_hovered(&self, mut bounds: Rectangle, cursor: Cursor) -> bool {
        // Ignore left edge to not conflict with other dividers
        bounds.x = (bounds.x + 5.0).min(bounds.x + bounds.width - 5.0);

        cursor.is_over(bounds)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Divider<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
    Theme: style::StyleSheet,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::default())
    }

    fn children(&self) -> Vec<widget::Tree> {
        vec![widget::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[&self.content]);
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let padding = [0.0, self.width, 0.0, 0.0];

        layout::padded(limits, Length::Fill, Length::Shrink, padding, |limits| {
            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits)
        })
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: event::Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<State>();

        let divider_hover_bounds = self.divider_hover_bounds(layout.bounds());

        state.is_divider_hovered = cursor.is_over(divider_hover_bounds);

        if let event::Event::Mouse(event) = event {
            match event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    if let Some(origin) = cursor.position_over(divider_hover_bounds) {
                        state.drag_origin = Some(origin);
                        return event::Status::Captured;
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    if state.drag_origin.take().is_some() {
                        shell.publish(self.on_release.clone());
                        return event::Status::Captured;
                    }
                }
                mouse::Event::CursorMoved { .. } => {
                    if let Some(position) = cursor.position() {
                        if let Some(origin) = state.drag_origin {
                            shell.publish((self.on_drag)((position - origin).x));
                            return event::Status::Captured;
                        }
                    }
                }
                _ => {}
            }
        }

        self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();

        if state.drag_origin.is_some() || state.is_divider_hovered {
            mouse::Interaction::ResizingHorizontally
        } else {
            self.content.as_widget().mouse_interaction(
                &tree.children[0],
                layout.children().next().unwrap(),
                cursor,
                viewport,
                renderer,
            )
        }
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );

        if self.is_content_hovered(layout.bounds(), cursor)
            || state.is_divider_hovered
            || state.drag_origin.is_some()
        {
            let appearance = theme.divider(
                &self.style,
                state.is_divider_hovered || state.drag_origin.is_some(),
            );

            let snap = |bounds: Rectangle| {
                let position = bounds.position();

                Rectangle {
                    x: position.x.floor(),
                    y: position.y.floor(),
                    width: self.width,
                    ..bounds
                }
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: snap(self.divider_bounds(layout.bounds())),
                    border: appearance.border,
                    shadow: Default::default(),
                },
                appearance
                    .background
                    .unwrap_or_else(|| Color::TRANSPARENT.into()),
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'_, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }

    fn operate(
        &self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.content.as_widget().operate(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Divider<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: renderer::Renderer + 'a,
    Theme: style::StyleSheet + 'a,
{
    fn from(divider: Divider<'a, Message, Theme, Renderer>) -> Self {
        Element::new(divider)
    }
}
