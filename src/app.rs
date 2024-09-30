use core::fmt;
use std::fmt::Display;

use iced::{
    advanced::graphics::{futures::backend::default, text::cosmic_text::Action},
    widget::{button, column, container, row, scrollable, text, text_editor, text_input},
    Alignment, Color, Element, Font, Length,
};
use iced_aw::selection_list;

use crate::{
    constants::{self, spacings}, theme::custom_theme, utils::{self, decode, encode}
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum Algorithm {
    #[default]
    Dec2,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::Dec2 => write!(f, "Dec2"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum CryptoAction {
    Encode,
    #[default]
    Decode,
}

impl CryptoAction {
    fn opposite(&self) -> Self {
        match self {
            CryptoAction::Encode => CryptoAction::Decode,
            CryptoAction::Decode => CryptoAction::Encode,
        }
    }

    fn as_str(&self) -> String {
        match self {
            CryptoAction::Encode => "encode".to_string(),
            CryptoAction::Decode => "decode".to_string(),
        }
    }

    fn input(&self) -> String {
        self.opposite().as_str()
    }

    fn output(&self) -> String {
        self.as_str()
    }
}

#[derive(Default)]
pub struct App {
    key: String,
    encode_content: text_editor::Content,
    decode_content: text_editor::Content,
    crypto_order: CryptoAction,
    algorithms: Vec<Algorithm>,
    using_algorithm: Algorithm,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    UpdateKey(String),
    ToEncode(iced::widget::text_editor::Action),
    ToDecode(iced::widget::text_editor::Action),
    SelectAlgorithm(usize, Algorithm),
    Translate,
    Flip,
    NewKey,
}

impl App {
    fn new() -> Self {
        Self {
            key: String::new(),
            crypto_order: CryptoAction::Decode,
            encode_content: text_editor::Content::with_text("encode text"),
            decode_content: text_editor::Content::with_text("encode text"),
            algorithms: vec![Algorithm::Dec2],
            using_algorithm: Algorithm::Dec2,
        }
    }

    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::UpdateKey(new_key) => {
                self.key = new_key;
            }
            AppMessage::ToEncode(action) => {
                self.encode_content.perform(action);
            }
            AppMessage::ToDecode(action) => {
                self.decode_content.perform(action);
            }
            AppMessage::NewKey => {
                self.key = utils::new_key();
            }
            AppMessage::Translate => {
                match self.crypto_order {
                    CryptoAction::Encode => {
                        let input = &self.decode_content;

                        self.encode_content =
                            text_editor::Content::with_text(&encode(input.text()));
                    }
                    CryptoAction::Decode => {
                        let input = &self.encode_content;

                        self.decode_content =
                            text_editor::Content::with_text(&decode(input.text()));
                    }
                };
            }
            AppMessage::Flip => {
                self.crypto_order = self.crypto_order.opposite();

                let decoded_text = self.decode_content.text().clone();

                self.decode_content = text_editor::Content::with_text(&self.encode_content.text());
                self.encode_content = text_editor::Content::with_text(&decoded_text);
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<AppMessage> {
        // two columns in a row. similar design to translation tools like google translate
        // switch button in the middle to change output with input

        let main = container(
            column![
                container(
                    column![
                        column![
                            row![
                                text("Key"),
                                text("symmetrical").color(constants::custom_theme::GREY_TEXT)
                            ]
                            .spacing(spacings::SMALL),
                            row![
                                text_input("key", self.key.as_str())
                                    .width(300)
                                    .on_input(AppMessage::UpdateKey),
                                button("Generate key icon").on_press(AppMessage::NewKey),
                            ]
                            .spacing(spacings::MEDIUM),
                        ]
                        .spacing(spacings::SMALL),
                        // selection_list(&self.algorithms, AppMessage::SelectAlgorithm).width(300)
                    ]
                    .spacing(spacings::MEDIUM),
                ),
                container(
                    row![
                        column![
                            row![
                                text("Input"),
                                text(format!("{}d", self.crypto_order.output()))
                                    .color(constants::custom_theme::GREY_TEXT)
                            ]
                            .spacing(spacings::SMALL),
                            text_editor(&self.encode_content)
                                .placeholder(format!("text to {}", self.crypto_order.input()))
                                .width(300)
                                .on_action(AppMessage::ToEncode),
                        ]
                        .spacing(spacings::SMALL),
                        column![
                            row![
                                text("Using"),
                                text(format!("{}", self.using_algorithm)).color(constants::custom_theme::GREY_TEXT)
                            ]
                            .spacing(spacings::SMALL),
                            button("translate icon")
                                .on_press(AppMessage::Translate),
                            button("flip icon")
                                .on_press(AppMessage::Flip),
                        ]
                        .spacing(spacings::SMALL)
                        .align_x(Alignment::Center)
                        .width(Length::Shrink),
                        column![
                            row![
                                text("Output"),
                                text(format!("{}d", self.crypto_order.input()))
                                    .color(constants::custom_theme::GREY_TEXT)
                            ]
                            .spacing(spacings::SMALL),
                            text_editor(&self.decode_content)
                                .placeholder(format!("{}d text", self.crypto_order.input()))
                                .width(300)
                                .on_action(AppMessage::ToDecode),
                        ]
                        .spacing(spacings::SMALL),
                    ]
                    .spacing(spacings::MEDIUM)
                    .align_y(Alignment::Center),
                ),
            ]
            .padding(spacings::LARGE)
            .spacing(spacings::LARGE)
            .align_x(Alignment::Center),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill);/* .style(|_| container::dark(&custom_theme(&self))); */

        let container = container(main /* scrollable(main) */);
        container.into()
    }
}
