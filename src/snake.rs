use bevy::prelude::{Commands, Component, default, Query, Res, SpriteBundle, Transform, Window, Without};
use bevy::scene::ron::de::Position;
use bevy::math::Vec2;
use bevy::hierarchy::BuildChildren;
use crate::images::Images;
use crate::{level1, level_map};
use crate::level_map::transform_from_position;

#[derive(Component)]
pub struct SnakeHead {
    pub position: Position,
}

pub fn initial_snake_head(position: Position) -> SnakeHead {
    return SnakeHead {
        position
    }
}

#[derive(Component)]
pub struct SnakeTail {
    pub position: Position,
    pub order: usize
}

pub fn initial_snake_tail(position: Position, order: usize) -> SnakeTail {
    return SnakeTail {
        position,
        order
    }
}

pub fn spawn_snake(mut commands: &mut Commands, images: &Res<Images>, window_center: Vec2) {
    let start = level1::STARTING_POSITION;
    let snake_tail = [
        initial_snake_tail(Position { col: start.col, line: start.line + 1 }, 0),
        initial_snake_tail(Position { col: start.col, line: start.line + 2 }, 1),
    ];
    commands.spawn((
        SpriteBundle {
            texture: images.snake_head.clone(),
            transform: transform_from_position(&start, window_center, 1.0),
            ..default()
        },
        initial_snake_head(level1::STARTING_POSITION)
    ));
    let snake_tail_entities = snake_tail.map(|segment| {
        return commands.spawn((
            SpriteBundle {
                texture: images.snake_tail.clone(),
                transform: transform_from_position(&segment.position, window_center, 1.0),
                ..default()
            },
            segment
        )).id();
    });
}

pub fn snake_movement_system(
    mut head_query: Query<(&mut SnakeHead, &mut Transform)>,
    mut tail_query: Query<(&mut SnakeTail, &mut Transform), Without<SnakeHead>>,
    windows: Query<&Window>
) {
    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);

    let (mut head, mut head_transform) = head_query.single_mut();
    let old_head_position = head.position;
    if old_head_position.col > 0 && old_head_position.line > 0 {
        head.position = Position { col: old_head_position.col, line: old_head_position.line - 1 };
        head_transform.translation = transform_from_position(&head.position, window_center, 1.0).translation;

        let mut next_position = old_head_position;
        for mut tail_segment in &mut tail_query {
            let mut tail = tail_segment.0;
            let mut tail_transform = tail_segment.1;
            (tail.position, next_position) = (next_position, tail.position);
            tail_transform.translation = transform_from_position(&tail.position, window_center, 1.0).translation;
        }
    }
}