use std::time::Duration;

use bevy::app::FixedUpdate;
use bevy::DefaultPlugins;
use bevy::math::Vec2;
use bevy::prelude::{App, AppExtStates, BuildChildren, Camera2dBundle, Commands, default, in_state, IntoSystemConfigs, PluginGroup, Query, Res, Startup, States, Time, Update, Window};
use bevy::time::Fixed;
use bevy::window::{WindowPlugin, WindowResolution};
use rand::Rng;

use crate::images::{Images, load_images};
use crate::level_map::LevelMap;

mod images;
mod level_map;
mod snake;
#[path = "levels/level1.rs"] mod level1;
mod apple;

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
        .add_systems(Startup, (load_images, setup, apple::spawn_apple).chain())
        .add_systems(FixedUpdate, snake::snake_movement_system.run_if(in_state(GameState::RUNNING)))
        .add_systems(Update, (snake::change_direction_system, snake::collision_system, snake::eating_system))
        .run();
}

fn setup(mut commands: Commands, level_map: Res<LevelMap>, images: Res<Images>, windows: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());
    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);
    snake::spawn_snake(&mut commands, &images, window_center);
    level_map.draw(commands, &images, window_center);
}

