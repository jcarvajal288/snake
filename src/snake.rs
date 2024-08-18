use bevy::prelude::{Commands, Component, default, Res, SpriteBundle};
use bevy::scene::ron::de::Position;
use bevy::math::Vec2;
use bevy::hierarchy::BuildChildren;
use crate::images::Images;
use crate::{level1, level_map};

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
    pub position: Position
}

pub fn initial_snake_tail(position: Position) -> SnakeTail {
    return SnakeTail {
        position
    }
}

pub fn spawn_snake(mut commands: &mut Commands, images: &Res<Images>, window_center: Vec2) {
    let start = level1::STARTING_POSITION;
    let snake_tail = [
        initial_snake_tail(Position { col: start.col, line: start.line + 1 }),
        initial_snake_tail(Position { col: start.col, line: start.line + 2 }),
    ];
    let snake_head = commands.spawn((
        SpriteBundle {
            texture: images.snake_head.clone(),
            transform: level_map::transform_from_position(&start, window_center, 1.0),
            ..default()
        },
        initial_snake_head(level1::STARTING_POSITION)
    )).id();
    let snake_tail_entities = snake_tail.map(|segment| {
        return commands.spawn((
            SpriteBundle {
                texture: images.snake_tail.clone(),
                transform: level_map::transform_from_position(&segment.position, window_center, 1.0),
                ..default()
            },
            segment
        )).id();
    });

    commands.entity(snake_head).push_children(&snake_tail_entities);
}