use iced_widget::container;

/// A set of rules that dictate the styling of a [`Table`](crate::Table).
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default + Clone;

    /// The header [`Appearance`](iced_widget::container::Appearance) of the [`StyleSheet`].
    fn header(&self, style: &Self::Style) -> container::Appearance;
    /// The footer [`Appearance`](iced_widget::container::Appearance) of the [`StyleSheet`].
    fn footer(&self, style: &Self::Style) -> container::Appearance;
    /// The row [`Appearance`](iced_widget::container::Appearance) of the [`StyleSheet`].
    fn row(&self, style: &Self::Style, index: usize) -> container::Appearance;
    /// The divider [`Appearance`](iced_widget::container::Appearance) of the [`StyleSheet`].
    fn divider(&self, style: &Self::Style, hovered: bool) -> container::Appearance;
}

impl StyleSheet for iced_style::Theme {
    type Style = ();

    fn header(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(self.extended_palette().background.strong.text),
            background: Some(self.extended_palette().background.strong.color.into()),
            ..Default::default()
        }
    }

    fn footer(&self, style: &Self::Style) -> container::Appearance {
        self.header(style)
    }

    fn row(&self, _style: &Self::Style, index: usize) -> container::Appearance {
        let pair = if index % 2 == 0 {
            self.extended_palette().background.base
        } else {
            self.extended_palette().background.weak
        };

        container::Appearance {
            text_color: Some(pair.text),
            background: Some(pair.color.into()),
            ..Default::default()
        }
    }

    fn divider(&self, _style: &Self::Style, hovered: bool) -> container::Appearance {
        let pair = if hovered {
            self.extended_palette().primary.base
        } else {
            self.extended_palette().background.weak
        };

        container::Appearance {
            background: Some(pair.color.into()),
            ..Default::default()
        }
    }
}

pub(crate) mod wrapper {
    use iced_core::{mouse::Cursor, Color, Element, Length, Size, Vector, Widget};
    use iced_widget::container;

    pub fn header<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::StyleSheet>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::StyleSheet + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::Header,
            style,
        }
        .into()
    }

    pub fn footer<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::StyleSheet>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::StyleSheet + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::Footer,
            style,
        }
        .into()
    }

    pub fn row<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::StyleSheet>::Style,
        index: usize,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::StyleSheet + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target: Target::Row { index },
            style,
        }
        .into()
    }

    enum Target {
        Header,
        Footer,
        Row { index: usize },
    }

    impl Target {
        fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as super::StyleSheet>::Style,
        ) -> container::Appearance
        where
            Theme: super::StyleSheet,
        {
            match self {
                Target::Header => theme.header(style),
                Target::Footer => theme.footer(style),
                Target::Row { index } => theme.row(style, *index),
            }
        }
    }

    struct Wrapper<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer,
        Theme: super::StyleSheet,
    {
        content: Element<'a, Message, Theme, Renderer>,
        target: Target,
        style: <Theme as super::StyleSheet>::Style,
    }

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Wrapper<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer,
        Theme: super::StyleSheet,
    {
        fn size(&self) -> Size<Length> {
            self.content.as_widget().size()
        }

        fn layout(
            &self,
            state: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.content.as_widget().layout(state, renderer, limits)
        }

        fn draw(
            &self,
            state: &iced_core::widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &iced_core::renderer::Style,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            viewport: &iced_core::Rectangle,
        ) {
            let appearance = self.target.appearance::<Theme>(theme, &self.style);

            renderer.fill_quad(
                iced_core::renderer::Quad {
                    bounds: layout.bounds(),
                    border: appearance.border,
                    shadow: Default::default(),
                },
                appearance
                    .background
                    .unwrap_or_else(|| Color::TRANSPARENT.into()),
            );

            let style = appearance
                .text_color
                .map(|text_color| iced_core::renderer::Style { text_color })
                .unwrap_or(*style);

            self.content
                .as_widget()
                .draw(state, renderer, theme, &style, layout, cursor, viewport)
        }

        fn tag(&self) -> iced_core::widget::tree::Tag {
            self.content.as_widget().tag()
        }

        fn state(&self) -> iced_core::widget::tree::State {
            self.content.as_widget().state()
        }

        fn children(&self) -> Vec<iced_core::widget::Tree> {
            self.content.as_widget().children()
        }

        fn diff(&self, tree: &mut iced_core::widget::Tree) {
            self.content.as_widget().diff(tree)
        }

        fn operate(
            &self,
            state: &mut iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn iced_core::widget::Operation<Message>,
        ) {
            self.content
                .as_widget()
                .operate(state, layout, renderer, operation)
        }

        fn on_event(
            &mut self,
            state: &mut iced_core::widget::Tree,
            event: iced_core::Event,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn iced_core::Clipboard,
            shell: &mut iced_core::Shell<'_, Message>,
            viewport: &iced_core::Rectangle,
        ) -> iced_core::event::Status {
            self.content.as_widget_mut().on_event(
                state, event, layout, cursor, renderer, clipboard, shell, viewport,
            )
        }

        fn mouse_interaction(
            &self,
            state: &iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            viewport: &iced_core::Rectangle,
            renderer: &Renderer,
        ) -> iced_core::mouse::Interaction {
            self.content
                .as_widget()
                .mouse_interaction(state, layout, cursor, viewport, renderer)
        }

        fn overlay<'b>(
            &'b mut self,
            state: &'b mut iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            renderer: &Renderer,
            translation: Vector,
        ) -> Option<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
            self.content
                .as_widget_mut()
                .overlay(state, layout, renderer, translation)
        }
    }

    impl<'a, Message, Theme, Renderer> From<Wrapper<'a, Message, Theme, Renderer>>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::StyleSheet + 'a,
        Message: 'a,
    {
        fn from(wrapper: Wrapper<'a, Message, Theme, Renderer>) -> Self {
            Element::new(wrapper)
        }
    }
}
