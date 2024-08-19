use bevy::prelude::{Commands, Component, JustifyText, TextBundle};
use bevy::text::TextStyle;
use bevy::ui::{Display, PositionType, Style, Val};
use bevy::utils::default;

#[derive(Component)]
pub struct GameOverText;

pub fn spawn_game_over_text(mut commands: Commands) {
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
                display: Display::None,
                ..default()
            }),
        GameOverText,
    ));
}