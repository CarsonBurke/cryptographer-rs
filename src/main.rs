use std::fmt::Display;

use app::App;
use constants::spacings;
use iced::{
    widget::{button, column, container, row, scrollable, text, text_editor, text_input}, Alignment, Color, Element, Font, Length
};
use iced_aw::selection_list;
use theme::{custom_theme, CustomThemeChoice};

pub mod constants;
pub mod theme;
pub mod app;
pub mod styles;
pub mod general_widgets;
pub mod utils;

pub fn main() -> iced::Result {
    iced::application("Cryptographer", App::update, App::view)
        .theme(custom_theme)
        .default_font(Font::DEFAULT)
        .run()
}
