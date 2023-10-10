use iced::alignment;
use iced::widget::{text, Text};
use iced::Font;

const ICONS: Font = Font::with_name("typicons");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
}

pub fn edit() -> Text<'static> {
    icon('\u{E0C3}')
}

pub fn stopwatch() -> Text<'static> {
    icon('\u{E10C}')
}

pub fn clock() -> Text<'static> {
    icon('\u{E120}')
}

pub fn left_arrow() -> Text<'static> {
    icon('\u{E00D}')
}

pub fn right_arrow() -> Text<'static> {
    icon('\u{E01A}')
}

pub fn settings() -> Text<'static> {
    icon('\u{E050}')
}

pub fn right_chevron() -> Text<'static> {
    icon('\u{E049}')
}

pub fn left_chevron() -> Text<'static> {
    icon('\u{E047}')
}

pub fn resume() -> Text<'static> {
    icon('\u{E0B0}')
}

pub fn stop() -> Text<'static> {
    icon('\u{E0B6}')
}

pub fn export() -> Text<'static> {
    icon('\u{E06D}')
}

pub fn delete() -> Text<'static> {
    icon('\u{E123}')
}

pub fn calendar() -> Text<'static> {
    icon('\u{E039}')
}
