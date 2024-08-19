use std::time::Duration;

use bevy::app::FixedUpdate;
use bevy::DefaultPlugins;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{App, AppExtStates, BuildChildren, Camera2dBundle, Commands, default, DespawnRecursiveExt, Display, Entity, in_state, IntoSystemConfigs, KeyCode, NextState, PluginGroup, Query, Res, ResMut, Startup, States, TextBundle, Time, Transform, Update, Window, With};
use bevy::time::Fixed;
use bevy::ui::Style;
use bevy::window::{WindowPlugin, WindowResolution};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::Rng;

use crate::images::{Images, load_images};
use crate::level_map::LevelMap;
use crate::snake::{reset_snake, SnakeHead, SnakeTail};
use crate::ui::{GameOverText, spawn_game_over_text};

mod images;
mod level_map;
mod snake;
#[path = "levels/level1.rs"] mod level1;
mod apple;
mod ui;

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
                resolution: WindowResolution::new(672., 672.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EmbeddedAssetPlugin::default())
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(300)))
        .insert_resource(Images::default())
        .insert_resource(level_map::load_level_1())
        .init_state::<GameState>()
        .add_systems(Startup, (load_images, setup, apple::spawn_apple, spawn_game_over_text).chain())
        .add_systems(FixedUpdate, snake::snake_movement_system.run_if(in_state(GameState::RUNNING)))
        .add_systems(Update, (snake::change_direction_system, snake::collision_system, snake::eating_system))
        .add_systems(Update, reset.run_if(in_state(GameState::DEFEAT)))
        .run();
}

fn setup(mut commands: Commands, level_map: Res<LevelMap>, images: Res<Images>, windows: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());
    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);
    snake::spawn_snake(&mut commands, &images, window_center);
    level_map.draw(commands, &images, window_center);
}

fn reset(
    mut commands: Commands,
    head_query: Query<(&mut SnakeHead, &mut Transform)>,
    tail_query: Query<Entity, With<SnakeTail>>,
    mut game_over_text_query: Query<&mut Style, With<GameOverText>>,
    windows: Query<&Window>,
    images: Res<Images>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        game_over_text_query.single_mut().display = Display::None;
        reset_snake(commands, head_query, tail_query, windows, images);
        next_state.set(GameState::RUNNING);
    }
}

