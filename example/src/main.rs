use std::fmt;

use iced::widget::{checkbox, column, container, horizontal_space, responsive, scrollable, text};
use iced::{Application, Command, Element, Length, Renderer, Theme};
use iced_table::table;

fn main() {
    App::run(Default::default()).unwrap()
}

#[derive(Debug, Clone)]
pub enum Message {
    SyncHeader(scrollable::AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
    ResizeColumnsEnabled(bool),
    FooterEnabled(bool),
    MinWidthEnabled(bool),
    DarkThemeEnabled(bool),
}

pub struct App {
    columns: Vec<Column>,
    rows: Vec<usize>,
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
                Column::new(Letter::A),
                Column::new(Letter::B),
                Column::new(Letter::C),
                Column::new(Letter::D),
                Column::new(Letter::E),
            ],
            rows: (1..=50).collect(),
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
    letter: Letter,
    width: f32,
    resize_offset: Option<f32>,
}

impl Column {
    fn new(letter: Letter) -> Self {
        Self {
            letter,
            width: 100.0,
            resize_offset: None,
        }
    }
}

enum Letter {
    A,
    B,
    C,
    D,
    E,
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Letter::A => "A",
            Letter::B => "B",
            Letter::C => "C",
            Letter::D => "D",
            Letter::E => "E",
        }
        .fmt(f)
    }
}

impl<'a, 'b> table::Column<'a, 'b, Message, Renderer> for Column {
    type Row = usize;

    fn header(&'b self, _col_index: usize) -> Element<'a, Message> {
        container(text(format!("Column {}", self.letter)))
            .height(24)
            .center_y()
            .into()
    }

    fn cell(
        &'b self,
        _col_index: usize,
        row_index: usize,
        _row: &'b Self::Row,
    ) -> Element<'a, Message> {
        container(text(format!("Cell {}{row_index}", self.letter)))
            .height(24)
            .center_y()
            .into()
    }

    fn footer(&'b self, _col_index: usize, rows: &'b [Self::Row]) -> Option<Element<'a, Message>> {
        let content = if matches!(self.letter, Letter::C) {
            Element::from(text(format!("Count: {}", rows.len())))
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
