//! A table widget for iced
#![deny(missing_debug_implementations, missing_docs)]
pub use style::StyleSheet;
pub use table::{table, Table};

mod divider;
mod style;

pub mod table {
    //! Display rows of data into columns
    use iced_core::{Element, Length, Padding};
    use iced_widget::{column, container, row, scrollable, Space};

    use super::divider::Divider;
    use super::style;

    /// Creates a new [`Table`] with the provided [`Column`] definitions
    /// and [`Row`](Column::Row) data.
    ///
    /// `on_sync` is needed to keep the header & footer scrollables in sync with
    /// the body scrollable. It is up to the consumer to emit a [`scroll_to`](iced_widget::scrollable::scroll_to) operation
    /// from `update` when this message is received.
    pub fn table<'a, Column, Row, Message, Renderer>(
        header: scrollable::Id,
        body: scrollable::Id,
        columns: &'a [Column],
        rows: &'a [Row],
        on_sync: fn(scrollable::AbsoluteOffset) -> Message,
    ) -> Table<'a, Column, Row, Message, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Renderer::Theme: style::StyleSheet + container::StyleSheet,
    {
        Table {
            header,
            body,
            footer: None,
            columns,
            rows,
            on_sync,
            on_column_drag: None,
            on_column_release: None,
            min_width: 0.0,
            min_column_width: 4.0,
            divider_width: 2.0,
            cell_padding: 4.into(),
            style: Default::default(),
            scrollable_properties: Box::new(|| Default::default()),
        }
    }

    /// Defines what a column looks like for each [`Row`](Self::Row) of data.
    pub trait Column<'a, 'b, Message, Renderer> {
        /// A row of data.
        type Row;

        /// Define the header [`Element`] for this column.
        fn header(&'b self, col_index: usize) -> Element<'a, Message, Renderer>;

        /// Define the cell [`Element`] for this column.
        fn cell(
            &'b self,
            col_index: usize,
            row_index: usize,
            row: &'b Self::Row,
        ) -> Element<'a, Message, Renderer>;

        /// Define the footer [`Element`] for this column.
        fn footer(
            &'b self,
            _col_index: usize,
            _rows: &'b [Self::Row],
        ) -> Option<Element<'a, Message, Renderer>> {
            None
        }

        /// Return the fixed width for this column.
        fn width(&self) -> f32;

        /// Return the offset of an on-going resize of this column.
        fn resize_offset(&self) -> Option<f32>;
    }

    /// An element to display rows of data into columns.
    #[allow(missing_debug_implementations)]
    pub struct Table<'a, Column, Row, Message, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Renderer::Theme: style::StyleSheet + container::StyleSheet,
    {
        header: scrollable::Id,
        body: scrollable::Id,
        footer: Option<scrollable::Id>,
        columns: &'a [Column],
        rows: &'a [Row],
        on_sync: fn(scrollable::AbsoluteOffset) -> Message,
        on_column_drag: Option<fn(usize, f32) -> Message>,
        on_column_release: Option<Message>,
        min_width: f32,
        min_column_width: f32,
        divider_width: f32,
        cell_padding: Padding,
        style: <Renderer::Theme as style::StyleSheet>::Style,
        // TODO: Upstream make this Copy
        scrollable_properties: Box<dyn Fn() -> scrollable::Properties + 'a>,
    }

    impl<'a, Column, Row, Message, Renderer> Table<'a, Column, Row, Message, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Renderer::Theme: style::StyleSheet + container::StyleSheet,
    {
        /// Sets the message that will be produced when a [`Column`] is resizing. Setting this
        /// will enable the resizing interaction.
        ///
        /// `on_drag` will emit a message during an on-going resize. It is up to the consumer to return
        /// this value for the associated column in [`Column::resize_offset`].
        ///
        /// `on_release` is emited when the resize is finished. It is up to the consumer to apply the last
        /// `on_drag` offset to the column's stored width.
        pub fn on_column_resize(
            self,
            on_drag: fn(usize, f32) -> Message,
            on_release: Message,
        ) -> Self {
            Self {
                on_column_drag: Some(on_drag),
                on_column_release: Some(on_release),
                ..self
            }
        }

        /// Show the footer returned by [`Column::footer`].
        pub fn footer(self, footer: scrollable::Id) -> Self {
            Self {
                footer: Some(footer),
                ..self
            }
        }

        /// Sets the minimum width of table.
        ///
        /// This is useful to use in conjuction with [`responsive`](iced_widget::responsive) to ensure
        /// the table always fills the width of it's parent container.
        pub fn min_width(self, min_width: f32) -> Self {
            Self { min_width, ..self }
        }

        /// Sets the minimum width a column can be resized to.
        pub fn min_column_width(self, min_column_width: f32) -> Self {
            Self {
                min_column_width,
                ..self
            }
        }

        /// Sets the width of the column dividers.
        pub fn divider_width(self, divider_width: f32) -> Self {
            Self {
                divider_width,
                ..self
            }
        }

        /// Sets the [`Padding`] used inside each cell of the [`Table`].
        pub fn cell_padding(self, cell_padding: impl Into<Padding>) -> Self {
            Self {
                cell_padding: cell_padding.into(),
                ..self
            }
        }

        /// Sets the style variant of this [`Table`].
        pub fn style(
            self,
            style: impl Into<<Renderer::Theme as style::StyleSheet>::Style>,
        ) -> Self {
            Self {
                style: style.into(),
                ..self
            }
        }

        ///  Sets the [`Properties`](iced_widget::scrollable::Properties) used for the table's body scrollable.
        pub fn scrollable_properties(self, f: impl Fn() -> scrollable::Properties + 'a) -> Self {
            Self {
                scrollable_properties: Box::new(f),
                ..self
            }
        }
    }

    impl<'a, 'b, Column, Row, Message, Renderer> From<Table<'b, Column, Row, Message, Renderer>>
        for Element<'a, Message, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Renderer::Theme: style::StyleSheet + container::StyleSheet + scrollable::StyleSheet,
        Column: self::Column<'a, 'b, Message, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        fn from(table: Table<'b, Column, Row, Message, Renderer>) -> Self {
            let Table {
                header,
                body,
                footer,
                columns,
                rows,
                on_sync,
                on_column_drag,
                on_column_release,
                min_width,
                min_column_width,
                divider_width,
                cell_padding,
                style,
                scrollable_properties,
            } = table;

            let header = scrollable(style::wrapper::header(
                row(columns
                    .iter()
                    .enumerate()
                    .map(|(index, column)| {
                        header_container(
                            index,
                            column,
                            on_column_drag,
                            on_column_release.clone(),
                            min_column_width,
                            divider_width,
                            cell_padding,
                            style.clone(),
                        )
                    })
                    .chain(dummy_container(columns, min_width, min_column_width))
                    .collect()),
                style.clone(),
            ))
            .id(header)
            .horizontal_scroll(
                scrollable::Properties::new()
                    .width(0)
                    .margin(0)
                    .scroller_width(0),
            )
            .vertical_scroll(
                scrollable::Properties::new()
                    .width(0)
                    .margin(0)
                    .scroller_width(0),
            );

            let body = scrollable(column(
                rows.iter()
                    .enumerate()
                    .map(|(row_index, _row)| {
                        style::wrapper::row(
                            row(columns
                                .iter()
                                .enumerate()
                                .map(|(col_index, column)| {
                                    body_container(
                                        col_index,
                                        row_index,
                                        column,
                                        _row,
                                        min_column_width,
                                        divider_width,
                                        cell_padding,
                                    )
                                })
                                .chain(dummy_container(columns, min_width, min_column_width))
                                .collect()),
                            style.clone(),
                            row_index,
                        )
                        .into()
                    })
                    .collect(),
            ))
            .id(body)
            .on_scroll(move |viewport| {
                let offset = viewport.absolute_offset();

                (on_sync)(scrollable::AbsoluteOffset { y: 0.0, ..offset })
            })
            .horizontal_scroll((scrollable_properties)())
            .vertical_scroll((scrollable_properties)())
            .height(Length::Fill);

            let footer = footer.map(|footer| {
                scrollable(style::wrapper::footer(
                    row(columns
                        .iter()
                        .enumerate()
                        .map(|(index, column)| {
                            footer_container(
                                index,
                                column,
                                rows,
                                on_column_drag,
                                on_column_release.clone(),
                                min_column_width,
                                divider_width,
                                cell_padding,
                                style.clone(),
                            )
                        })
                        .chain(dummy_container(columns, min_width, min_column_width))
                        .collect()),
                    style,
                ))
                .id(footer)
                .horizontal_scroll(
                    scrollable::Properties::new()
                        .width(0)
                        .margin(0)
                        .scroller_width(0),
                )
                .vertical_scroll(
                    scrollable::Properties::new()
                        .width(0)
                        .margin(0)
                        .scroller_width(0),
                )
            });

            let mut column = column![header, body];

            if let Some(footer) = footer {
                column = column.push(footer);
            }

            column.height(Length::Fill).into()
        }
    }

    fn header_container<'a, 'b, Column, Row, Message, Renderer>(
        index: usize,
        column: &'b Column,
        on_drag: Option<fn(usize, f32) -> Message>,
        on_release: Option<Message>,
        min_column_width: f32,
        divider_width: f32,
        cell_padding: Padding,
        style: <Renderer::Theme as style::StyleSheet>::Style,
    ) -> Element<'a, Message, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Renderer::Theme: style::StyleSheet + container::StyleSheet,
        Column: self::Column<'a, 'b, Message, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let content = container(column.header(index))
            .width(Length::Fill)
            .padding(cell_padding)
            .into();

        with_divider(
            index,
            column,
            content,
            on_drag,
            on_release,
            min_column_width,
            divider_width,
            style,
        )
    }

    fn body_container<'a, 'b, Column, Row, Message, Renderer>(
        col_index: usize,
        row_index: usize,
        column: &'b Column,
        row: &'b Row,
        min_column_width: f32,
        divider_width: f32,
        cell_padding: Padding,
    ) -> Element<'a, Message, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Renderer::Theme: style::StyleSheet + container::StyleSheet,
        Column: self::Column<'a, 'b, Message, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let width = column.width() + column.resize_offset().unwrap_or_default();

        let content = container(column.cell(col_index, row_index, row))
            .width(Length::Fill)
            .padding(cell_padding);

        let spacing = Space::new(divider_width, Length::Shrink);

        row![content, spacing]
            .width(width.max(min_column_width))
            .into()
    }

    fn footer_container<'a, 'b, Column, Row, Message, Renderer>(
        index: usize,
        column: &'b Column,
        rows: &'b [Row],
        on_drag: Option<fn(usize, f32) -> Message>,
        on_release: Option<Message>,
        min_column_width: f32,
        divider_width: f32,
        cell_padding: Padding,
        style: <Renderer::Theme as style::StyleSheet>::Style,
    ) -> Element<'a, Message, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Renderer::Theme: style::StyleSheet + container::StyleSheet,
        Column: self::Column<'a, 'b, Message, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let content = if let Some(footer) = column.footer(index, rows) {
            container(footer)
                .width(Length::Fill)
                .padding(cell_padding)
                .center_y()
                .into()
        } else {
            Element::from(Space::with_width(Length::Fill))
        };

        with_divider(
            index,
            column,
            content,
            on_drag,
            on_release,
            min_column_width,
            divider_width,
            style,
        )
    }

    fn with_divider<'a, 'b, Column, Row, Message, Renderer>(
        index: usize,
        column: &'b Column,
        content: Element<'a, Message, Renderer>,
        on_drag: Option<fn(usize, f32) -> Message>,
        on_release: Option<Message>,
        min_column_width: f32,
        divider_width: f32,
        style: <Renderer::Theme as style::StyleSheet>::Style,
    ) -> Element<'a, Message, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Renderer::Theme: style::StyleSheet + container::StyleSheet,
        Column: self::Column<'a, 'b, Message, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let width =
            (column.width() + column.resize_offset().unwrap_or_default()).max(min_column_width);

        if let Some((on_drag, on_release)) = on_drag.zip(on_release) {
            let old_width = column.width();

            container(Divider::new(
                content,
                divider_width,
                move |offset| {
                    let new_width = (old_width + offset).max(min_column_width);

                    (on_drag)(index, new_width - old_width)
                },
                on_release,
                style,
            ))
            .width(width)
            .into()
        } else {
            row![content, Space::new(divider_width, Length::Shrink)]
                .width(width)
                .into()
        }
    }

    // Used to enforce "min_width"
    fn dummy_container<'a, 'b, Column, Row, Message, Renderer>(
        columns: &'b [Column],
        min_width: f32,
        min_column_width: f32,
    ) -> Option<Element<'a, Message, Renderer>>
    where
        Renderer: iced_core::Renderer + 'a,
        Renderer::Theme: style::StyleSheet + container::StyleSheet,
        Column: self::Column<'a, 'b, Message, Renderer, Row = Row>,
        Message: 'a + Clone,
    {
        let total_width: f32 = columns
            .iter()
            .map(|column| {
                (column.width() + column.resize_offset().unwrap_or_default()).max(min_column_width)
            })
            .sum();

        let remaining = min_width - total_width;

        (remaining > 0.0).then(|| container(Space::with_width(remaining)).into())
    }
}
