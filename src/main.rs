mod images;
mod level_map;
mod snake;
#[path = "levels/level1.rs"] mod level1;

use std::time::Duration;
use bevy::app::FixedUpdate;
use bevy::asset::ron::de::Position;
use bevy::DefaultPlugins;
use bevy::math::Vec2;
use bevy::prelude::{App, AppExtStates, BuildChildren, Camera2dBundle, Commands, default, in_state, IntoSystemConfigs, PluginGroup, Query, Res, SpriteBundle, Startup, States, Time, Transform, Update, Window, Without};
use bevy::time::{Fixed};
use bevy::window::{WindowPlugin, WindowResolution};
use rand::Rng;
use crate::images::{Images, load_images};
use crate::level_map::{Apple, LevelMap, transform_from_position};
use crate::snake::{initial_snake_head, SnakeHead, SnakeTail};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    RUNNING,
    DEFEAT,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(736., 736.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(500)))
        .insert_resource(Images::default())
        .insert_resource(level_map::load_level_1())
        .init_state::<GameState>()
        .add_systems(Startup, (load_images, setup, spawn_apple).chain())
        .add_systems(FixedUpdate, snake::snake_movement_system.run_if(in_state(GameState::RUNNING)))
        .add_systems(Update, (snake::change_direction_system, snake::collision_system))
        .run();
}

fn setup(mut commands: Commands, level_map: Res<LevelMap>, images: Res<Images>, windows: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());
    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);
    snake::spawn_snake(&mut commands, &images, window_center);
    level_map.draw(commands, &images, window_center);
}

fn spawn_apple(
    mut commands: Commands,
    level_map: Res<LevelMap>,
    images: Res<Images>,
    windows: Query<&Window>,
    mut head_query: Query<&mut SnakeHead>,
    mut tail_query: Query<&mut SnakeTail>,
) {
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
    let apple_position = open_floor_positions.get(apple_position_index).unwrap();

    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);
    commands.spawn((
        SpriteBundle {
            texture: images.apple.clone(),
            transform: transform_from_position(apple_position, window_center, 1.0),
            ..default()
        },
        Apple { position: *apple_position }
    ));
}
