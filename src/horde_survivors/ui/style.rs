use bevy::prelude::*;

pub const MAIN_WINDOW_BG_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.justify_content = JustifyContent::SpaceEvenly;
    style.align_content = AlignContent::Center;
    style
};
pub const MAIN_WINDOW_BG_COLOR: BackgroundColor = BackgroundColor(Color::RgbaLinear { red: 0., green: 0., blue: 0., alpha: 0.9 });


pub fn menu_text(text: &str, size: f32) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size:  size,
            ..default()
        }
    ).with_style(Style {
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        ..default()
    })
}