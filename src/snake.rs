use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, default, KeyCode, Mut, NextState, Query, Res, ResMut, SpriteBundle, Transform, Window, Without};
use bevy::scene::ron::de::Position;

use crate::images::Images;
use crate::{GameState, level1};
use crate::apple::{Apple, find_open_position};
use crate::level_map::{LevelMap, transform_from_position};
use crate::snake::Direction::{DOWN, LEFT, RIGHT, UP};

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Component)]
pub struct SnakeHead {
    pub position: Position,
    pub direction: Direction,
}

pub fn initial_snake_head(position: Position) -> SnakeHead {
    return SnakeHead {
        position,
        direction: Direction::UP,
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
    windows: Query<&Window>,
) {
    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);

    let (mut head, mut head_transform) = head_query.single_mut();
    let old_head_position = head.position;
    if old_head_position.col > 0 && old_head_position.line > 0 {
        head = move_head_position(head);
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

fn move_head_position(mut head: Mut<SnakeHead>) -> Mut<SnakeHead> {
    head.position = match head.direction {
        LEFT => Position { col: head.position.col - 1, line: head.position.line },
        RIGHT => Position { col: head.position.col + 1, line: head.position.line },
        UP => Position { col: head.position.col, line: head.position.line - 1 },
        DOWN => Position { col: head.position.col, line: head.position.line + 1},
    };
    return head;
}

pub fn change_direction_system(
    mut head_query: Query<&mut SnakeHead>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut head = head_query.single_mut();
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        head.direction = LEFT;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        head.direction = RIGHT;
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        head.direction = UP;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        head.direction = DOWN;
    }
}

pub fn collision_system(
    mut head_query: Query<&mut SnakeHead>,
    level_map: Res<LevelMap>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let mut head = head_query.single_mut();
    if !level_map.is_position_walkable(&head.position) {
        next_state.set(GameState::DEFEAT);
    }
}

pub fn eating_system(
    mut head_query: Query<&mut SnakeHead>,
    tail_query: Query<&mut SnakeTail>,
    mut apple_query: Query<(&mut Apple, &mut Transform), Without<SnakeHead>>,
    level_map: Res<LevelMap>,
    windows: Query<&Window>,
) {
    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);
    let mut head = head_query.single_mut();
    let (mut apple, mut apple_transform) = apple_query.single_mut();
    if(head.position == apple.position) {
        let new_apple_position = find_open_position(level_map, &mut head_query, tail_query);
        apple_transform.translation = transform_from_position(&new_apple_position, window_center, 0.5).translation;
        apple.position = new_apple_position;
    }
}