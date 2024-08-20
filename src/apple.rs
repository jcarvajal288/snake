use bevy::asset::ron::de::Position;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, default, Query, Res, SpriteBundle, Transform, Window, Without};
use rand::Rng;

use crate::images::Images;
use crate::level_map::{LevelMap, transform_from_position};
use crate::snake::{SnakeHead, SnakeTail};

#[derive(Component)]
pub struct Apple {
    pub position: Position
}
pub fn spawn_apple(
    mut commands: Commands,
    level_map: Res<LevelMap>,
    images: Res<Images>,
    windows: Query<&Window>,
    mut head_query: Query<&mut SnakeHead>,
    tail_query: Query<&mut SnakeTail>,
) {
    let apple_position = find_open_position(level_map, &mut head_query, &tail_query);

    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);
    commands.spawn((
        SpriteBundle {
            texture: images.apple.clone(),
            transform: transform_from_position(&apple_position, window_center, 0.5),
            ..default()
        },
        Apple { position: Position {
            col: apple_position.col,
            line: apple_position.line
        }}
    ));
}

pub fn find_open_position(level_map: Res<LevelMap>, head_query: &mut Query<&mut SnakeHead>, tail_query: &Query<&mut SnakeTail>) -> Position {
    let head_position = head_query.single_mut().position;
    let mut all_snake_positions: Vec<Position> = tail_query.iter()
        .map(|segment| { return segment.position })
        .collect::<Vec<Position>>();
    all_snake_positions.extend([head_position].iter());
    let mut open_floor_positions = Vec::new();
    for line in 0..level_map.grid.len() {
        for col in 0..level_map.grid.get(line).unwrap().len() {
            let position = Position { col, line };
            if level_map.is_position_walkable(&position) && !all_snake_positions.contains(&position) {
                open_floor_positions.extend([position].iter());
            }
        }
    }
    let apple_position_index = rand::thread_rng().gen_range(0..open_floor_positions.len());
    let apple_position: &Position = open_floor_positions.get(apple_position_index).unwrap();
    return Position {
        col: apple_position.col,
        line: apple_position.line
    }
}
