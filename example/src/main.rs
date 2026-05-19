use std::fmt;

use iced::widget::{button, checkbox, column, container, operation, pick_list, responsive, scrollable, space, text, text_input};
use iced::{widget, Element, Length, Renderer, Task, Theme};
use iced_table::table;

fn main() {
    iced::application(App::boot, App::update, App::view)
        .title("Iced Table")
        .theme(App::theme)
        .run()
        .unwrap()
}

#[derive(Debug, Clone)]
enum Message {
    SyncHeader(scrollable::AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
    ResizeColumnsEnabled(bool),
    FooterEnabled(bool),
    MinWidthEnabled(bool),
    DarkThemeEnabled(bool),
    Notes(usize, String),
    Category(usize, Category),
    Enabled(usize, bool),
    Delete(usize),
}

struct App {
    columns: Vec<Column>,
    rows: Vec<Row>,
    header: widget::Id,
    body: widget::Id,
    footer: widget::Id,
    resize_columns_enabled: bool,
    footer_enabled: bool,
    min_width_enabled: bool,
    theme: Theme,
}

impl App {
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn boot() -> App {
        App {
            columns: vec![
                Column::new(ColumnKind::Index),
                Column::new(ColumnKind::Category),
                Column::new(ColumnKind::Enabled),
                Column::new(ColumnKind::Notes),
                Column::new(ColumnKind::Delete),
            ],
            rows: (0..50).map(Row::generate).collect(),
            header: widget::Id::unique(),
            body: widget::Id::unique(),
            footer: widget::Id::unique(),
            resize_columns_enabled: true,
            footer_enabled: true,
            min_width_enabled: true,
            theme: Theme::Light,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SyncHeader(offset) => {
                return Task::batch(vec![
                    operation::scroll_to(self.header.clone(), offset),
                    operation::scroll_to(self.footer.clone(), offset),
                ])
            }
            Message::Resizing(index, offset) => {
                if let Some(column) = self.columns.get_mut(index) {
                    column.resize_offset = Some(offset);
                }
            }
            Message::Resized => self.columns.iter_mut().for_each(|column| {
                if let Some(offset) = column.resize_offset.take() {
                    column.width += offset;
                }
            }),
            Message::ResizeColumnsEnabled(enabled) => self.resize_columns_enabled = enabled,
            Message::FooterEnabled(enabled) => self.footer_enabled = enabled,
            Message::MinWidthEnabled(enabled) => self.min_width_enabled = enabled,
            Message::DarkThemeEnabled(enabled) => {
                if enabled {
                    self.theme = Theme::Dark;
                } else {
                    self.theme = Theme::Light;
                }
            }
            Message::Category(index, category) => {
                if let Some(row) = self.rows.get_mut(index) {
                    row.category = category;
                }
            }
            Message::Enabled(index, is_enabled) => {
                if let Some(row) = self.rows.get_mut(index) {
                    row.is_enabled = is_enabled;
                }
            }
            Message::Notes(index, notes) => {
                if let Some(row) = self.rows.get_mut(index) {
                    row.notes = notes;
                }
            }
            Message::Delete(index) => {
                self.rows.remove(index);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let table = responsive(|size| {
            let mut table = table(
                self.header.clone(),
                self.body.clone(),
                &self.columns,
                &self.rows,
                Message::SyncHeader,
            );

            if self.resize_columns_enabled {
                table = table.on_column_resize(Message::Resizing, Message::Resized);
            }
            if self.footer_enabled {
                table = table.footer(self.footer.clone());
            }
            if self.min_width_enabled {
                table = table.min_width(size.width);
            }

            table.into()
        });

        let content = column![
            checkbox(self.resize_columns_enabled,)
                .label("Resize Columns")
                .on_toggle(Message::ResizeColumnsEnabled),
            checkbox(self.footer_enabled,)
                .label("Footer")
                .on_toggle(Message::FooterEnabled),
            checkbox(self.min_width_enabled,)
                .label("Min Width")
                .on_toggle(Message::MinWidthEnabled),
            checkbox(matches!(self.theme, Theme::Dark),)
                .label("Dark Theme")
                .on_toggle(Message::DarkThemeEnabled),
            table,
        ]
        .spacing(6);

        container(container(content).width(Length::Fill).height(Length::Fill))
            .padding(20)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

struct Column {
    kind: ColumnKind,
    width: f32,
    resize_offset: Option<f32>,
}

impl Column {
    fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::Index => 60.0,
            ColumnKind::Category => 100.0,
            ColumnKind::Enabled => 155.0,
            ColumnKind::Notes => 400.0,
            ColumnKind::Delete => 100.0,
        };

        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

enum ColumnKind {
    Index,
    Category,
    Enabled,
    Notes,
    Delete,
}

struct Row {
    notes: String,
    category: Category,
    is_enabled: bool,
}

impl Row {
    fn generate(index: usize) -> Self {
        let category = match index % 5 {
            0 => Category::A,
            1 => Category::B,
            2 => Category::C,
            3 => Category::D,
            4 => Category::E,
            _ => unreachable!(),
        };
        let is_enabled = index % 5 < 4;

        Self {
            notes: String::new(),
            category,
            is_enabled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    A,
    B,
    C,
    D,
    E,
}

impl Category {
    const ALL: &'static [Self] = &[Self::A, Self::B, Self::C, Self::D, Self::E];
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::A => "A",
            Category::B => "B",
            Category::C => "C",
            Category::D => "D",
            Category::E => "E",
        }
        .fmt(f)
    }
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for Column {
    type Row = Row;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Index => "Index",
            ColumnKind::Category => "Category",
            ColumnKind::Enabled => "Enabled",
            ColumnKind::Notes => "Notes",
            ColumnKind::Delete => "",
        };

        container(text(content)).center_y(24).into()
    }

    fn cell(&'a self, _col_index: usize, row_index: usize, row: &'a Row) -> Element<'a, Message> {
        let content: Element<_> = match self.kind {
            ColumnKind::Index => text(row_index).into(),
            ColumnKind::Category => pick_list(Category::ALL, Some(row.category), move |category| {
                Message::Category(row_index, category)
            })
            .into(),
            ColumnKind::Enabled => checkbox(row.is_enabled)
                .on_toggle(move |enabled| Message::Enabled(row_index, enabled))
                .into(),
            ColumnKind::Notes => text_input("", &row.notes)
                .on_input(move |notes| Message::Notes(row_index, notes))
                .width(Length::Fill)
                .into(),
            ColumnKind::Delete => button(text("Delete"))
                .on_press(Message::Delete(row_index))
                .into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn footer(&'a self, _col_index: usize, rows: &'a [Row]) -> Option<Element<'a, Message>> {
        let content = if matches!(self.kind, ColumnKind::Enabled) {
            let total_enabled = rows.iter().filter(|row| row.is_enabled).count();

            Element::from(text(format!("Total Enabled: {total_enabled}")))
        } else {
            space::horizontal().into()
        };

        Some(container(content).center_y(24).into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
