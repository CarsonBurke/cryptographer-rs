use iced::Color;

use crate::App;

pub enum CustomThemeChoice {
    Light,
    Dark,
}

impl CustomThemeChoice {
    pub fn from_system() -> Self {
        CustomThemeChoice::Light
    }
}

pub fn custom_theme<'a>(_: &'a App) -> iced::Theme {
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
}
