use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, NextState, Query, Res, ResMut, Transform, Window, With};
use crate::GameState;
use crate::images::Images;
use crate::level_map::{LevelMap, load_level, MapTile};
use crate::levels::{LEVEL_1_SELECT, LEVEL_1, LEVEL_2_SELECT, LEVEL_2, LEVEL_3, LEVEL_3_SELECT};
use crate::snake::{reset_snake, SnakeHead, SnakeTail};

pub fn level_select_system(
    mut commands: Commands,
    head_query: Query<(&mut SnakeHead, &mut Transform)>,
    mut tile_query: Query<Entity, With<MapTile>>,
    images: Res<Images>,
    mut level_map: ResMut<LevelMap>,
    windows: Query<&Window>,
    tail_query: Query<Entity, With<SnakeTail>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);
    let (head, _head_transform) = head_query.single();
    let selected_level: &str = if head.position == LEVEL_1_SELECT {
        LEVEL_1
    } else if head.position == LEVEL_2_SELECT {
        LEVEL_2
    } else if head.position == LEVEL_3_SELECT {
        LEVEL_3
    } else {
        "NONE"
    };

    if selected_level == "NONE" {
        return;
    }

    for entity in tile_query.iter() {
        commands.entity(entity).despawn();
    }
    level_map.grid = load_level(selected_level).grid;
    let commands2 = level_map.draw(commands, &images, window_center);
    reset_snake(commands2, head_query, tail_query, windows, images);
    next_state.set(GameState::RUNNING);
}
