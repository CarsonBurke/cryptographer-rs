use std::fmt::Display;

use constants::spacings;
use iced::{
    widget::{button, column, container, row, text, text_editor, text_input},
    Alignment, Color, Element, Length,
};
use iced_aw::selection_list;
use theme::CustomThemeChoice;

pub mod constants;
pub mod theme;

pub fn main() -> iced::Result {
    iced::application("Cryptographer", App::update, App::view)
        .theme(|_| {
            let theme_color = CustomThemeChoice::from_system();

            let theme = match theme_color {
                CustomThemeChoice::Dark => iced::Theme::custom(
                    String::from("Custom"),
                    iced::theme::Palette {
                        success: Color::from_rgb(46. / 255., 194. / 255., 126. / 255.),
                        danger: Color::from_rgb(244. / 255., 27. / 255., 36. / 255.),
                        text: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.),
                        // primary: Color::from_rgb(30. / 255., 30. / 255., 30. / 255.),
                        primary: Color::from_rgb(0.21, 0.52, 0.89),
                        background: Color::from_rgb(42. / 255., 42. / 255., 42. / 255.),
                    },
                ),
                CustomThemeChoice::Light => iced::Theme::custom(
                    String::from("Custom"),
                    iced::theme::Palette {
                        success: Color::from_rgb(46. / 255., 194. / 255., 126. / 255.),
                        danger: Color::from_rgb(244. / 255., 27. / 255., 36. / 255.),
                        text: Color::from_rgb(255. / 255., 255. / 255., 255. / 255.),
                        // primary: Color::from_rgb(30. / 255., 30. / 255., 30. / 255.),
                        primary: Color::from_rgb(0.21, 0.52, 0.89),
                        background: Color::from_rgb(42. / 255., 42. / 255., 42. / 255.),
                    },
                ),
            };

            theme
        })
        .run()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Algorithm {
    Dec2,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Default)]
pub struct App {
    encode_content: text_editor::Content,
    decode_content: text_editor::Content,
    algorithms: Vec<Algorithm>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Key(String),
    ToEncode(String),
    ToDecode(String),
    SelectAlgorithm(usize, Algorithm),
}

impl App {
    fn new() -> Self {
        Self {
            encode_content: text_editor::Content::with_text("encode text"),
            decode_content: text_editor::Content::with_text("encode text"),
            algorithms: vec![Algorithm::Dec2],
        }
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            _ => {}
        }
    }

    fn view(&self) -> Element<AppMessage> {
        // two columns in a row. similar design to translation tools like google translate
        // switch button in the middle to change output with input

        let main = container(
            column![
                column![
                column![
                    text("key"),
                    row![
                        text_input("key", "x").width(300),
                        button("Generate key icon"),
                    ]
                    .spacing(spacings::MEDIUM),
                ]
                .spacing(spacings::SMALL),
                selection_list(&self.algorithms, AppMessage::SelectAlgorithm)
                ].spacing(spacings::MEDIUM),
                row![
                    column![
                        text("encode"),
                        text_editor(&self.encode_content).placeholder("text to encode"),
                    ]
                    .spacing(spacings::SMALL),
                    column![text("hashing algorithm"), button("flip icon"),]
                        .spacing(spacings::SMALL),
                    column![
                        text("decode"),
                        text_editor(&self.decode_content).placeholder("text to decode"),
                    ]
                    .spacing(spacings::SMALL),
                ]
                .spacing(spacings::MEDIUM)
                .align_y(Alignment::Center),
            ]
            .padding(spacings::LARGE)
            .spacing(spacings::MEDIUM)
            .align_x(Alignment::Center),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill);

        let container = container(main);
        container.into()
    }
}
