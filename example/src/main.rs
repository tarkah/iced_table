use std::fmt;

use iced::widget::{
    button, checkbox, column, container, horizontal_space, pick_list, responsive, scrollable, text,
    text_input,
};
use iced::{Application, Command, Element, Length, Renderer, Theme};
use iced_table::table;

fn main() {
    App::run(Default::default()).unwrap()
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
    header: scrollable::Id,
    body: scrollable::Id,
    footer: scrollable::Id,
    resize_columns_enabled: bool,
    footer_enabled: bool,
    min_width_enabled: bool,
    theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            columns: vec![
                Column::new(ColumnKind::Index),
                Column::new(ColumnKind::Category),
                Column::new(ColumnKind::Enabled),
                Column::new(ColumnKind::Notes),
                Column::new(ColumnKind::Delete),
            ],
            rows: (0..50).map(Row::generate).collect(),
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
            footer: scrollable::Id::unique(),
            resize_columns_enabled: true,
            footer_enabled: true,
            min_width_enabled: true,
            theme: Theme::Light,
        }
    }
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Iced Table".into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SyncHeader(offset) => {
                return Command::batch(vec![
                    scrollable::scroll_to(self.header.clone(), offset),
                    scrollable::scroll_to(self.footer.clone(), offset),
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

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
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
            checkbox(
                "Resize Columns",
                self.resize_columns_enabled,
                Message::ResizeColumnsEnabled
            ),
            checkbox("Footer", self.footer_enabled, Message::FooterEnabled),
            checkbox(
                "Min Width",
                self.min_width_enabled,
                Message::MinWidthEnabled
            ),
            checkbox(
                "Dark Theme",
                matches!(self.theme, Theme::Dark),
                Message::DarkThemeEnabled
            ),
            table,
        ]
        .spacing(6);

        container(container(content).width(Length::Fill).height(Length::Fill))
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
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

impl<'a, 'b> table::Column<'a, 'b, Message, Renderer> for Column {
    type Row = Row;

    fn header(&'b self, _col_index: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Index => "Index",
            ColumnKind::Category => "Category",
            ColumnKind::Enabled => "Enabled",
            ColumnKind::Notes => "Notes",
            ColumnKind::Delete => "",
        };

        container(text(content)).height(24).center_y().into()
    }

    fn cell(
        &'b self,
        _col_index: usize,
        row_index: usize,
        row: &'b Self::Row,
    ) -> Element<'a, Message> {
        let content: Element<_> = match self.kind {
            ColumnKind::Index => text(row_index).into(),
            ColumnKind::Category => pick_list(Category::ALL, Some(row.category), move |category| {
                Message::Category(row_index, category)
            })
            .into(),
            ColumnKind::Enabled => checkbox("", row.is_enabled, move |enabled| {
                Message::Enabled(row_index, enabled)
            })
            .into(),
            ColumnKind::Notes => text_input("", &row.notes)
                .padding(2)
                .on_input(move |notes| Message::Notes(row_index, notes))
                .width(Length::Fill)
                .into(),
            ColumnKind::Delete => button(text("Delete"))
                .padding(2)
                .on_press(Message::Delete(row_index))
                .into(),
        };

        container(content)
            .width(Length::Fill)
            .height(24)
            .center_y()
            .into()
    }

    fn footer(&'b self, _col_index: usize, rows: &'b [Self::Row]) -> Option<Element<'a, Message>> {
        let content = if matches!(self.kind, ColumnKind::Enabled) {
            let total_enabled = rows.iter().filter(|row| row.is_enabled).count();

            Element::from(text(format!("Total Enabled: {total_enabled}")))
        } else {
            horizontal_space(Length::Fill).into()
        };

        Some(container(content).height(24).center_y().into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
