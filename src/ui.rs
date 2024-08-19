use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, JustifyText, Query, TextBundle, Window};
use bevy::text::TextStyle;
use bevy::ui::{PositionType, Style, Val};
use bevy::utils::default;

#[derive(Component)]
pub struct GameOverText;

pub fn show_game_over_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
        "GAME OVER",
        TextStyle {
                font_size: 100.0,
                ..default()
            }
        ).with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(40.0),
                left: Val::Percent(17.0),
                ..default()
            }),
        GameOverText,
    ));
}