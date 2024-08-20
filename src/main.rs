use std::time::Duration;

use bevy::app::FixedUpdate;
use bevy::DefaultPlugins;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{App, AppExtStates, BuildChildren, Camera2dBundle, Commands, default, DespawnRecursiveExt, Display, Entity, in_state, IntoSystemConfigs, KeyCode, NextState, not, OnEnter, PluginGroup, Query, Res, ResMut, Startup, States, TextBundle, Time, Transform, Update, Window, With, Without};
use bevy::time::Fixed;
use bevy::ui::Style;
use bevy::window::{WindowPlugin, WindowResolution};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::Rng;
use crate::apple::{Apple, move_apple_system, spawn_apple};
use crate::images::{Images, load_images};
use crate::level_map::{LevelMap, load_level, MapTile};
use crate::levels::MENU;
use crate::menu::level_select_system;
use crate::snake::{reset_snake, SnakeHead, SnakeTail};
use crate::ui::{GameOverText, ResetText, spawn_game_over_text};

mod images;
mod level_map;
mod snake;
mod levels;
mod apple;
mod ui;
mod menu;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MENU,
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
        .insert_resource(level_map::load_level(levels::MENU))
        .init_state::<GameState>()
        .add_systems(Startup, (load_images, setup, spawn_game_over_text).chain())
        .add_systems(OnEnter(GameState::RUNNING), spawn_apple)
        .add_systems(FixedUpdate,
                     snake::snake_movement_system
                         .run_if(not(in_state(GameState::DEFEAT)))
        )
        .add_systems(Update, level_select_system.run_if(in_state(GameState::MENU)))
        .add_systems(Update, (snake::change_direction_system, snake::collision_system, snake::eating_system.run_if(in_state(GameState::RUNNING))))
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
    apple_query: Query<Entity, With<Apple>>,
    mut game_over_text_query: Query<&mut Style, With<GameOverText>>,
    mut reset_text_query: Query<&mut Style, (With<ResetText>, Without<GameOverText>)>,
    windows: Query<&Window>,
    images: Res<Images>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut level_map: ResMut<LevelMap>,
    mut tile_query: Query<Entity, With<MapTile>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);
        for entity in apple_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in tile_query.iter() {
            commands.entity(entity).despawn();
        }
        game_over_text_query.single_mut().display = Display::None;
        reset_text_query.single_mut().display = Display::None;
        level_map.grid = load_level(MENU).grid;
        let commands2 = level_map.draw(commands, &images, window_center);
        reset_snake(commands2, head_query, tail_query, windows, images);
        next_state.set(GameState::MENU);
    }
}

