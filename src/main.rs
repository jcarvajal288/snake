mod images;
mod level_map;
mod snake;
#[path = "levels/level1.rs"] mod level1;

use std::time::Duration;
use bevy::app::FixedUpdate;
use bevy::DefaultPlugins;
use bevy::math::Vec2;
use bevy::prelude::{App, BuildChildren, Camera2dBundle, Commands, default, IntoSystemConfigs, PluginGroup, Query, Res, Startup, Time, Window};
use bevy::time::{Fixed};
use bevy::window::{WindowPlugin, WindowResolution};
use crate::images::{Images, load_images};
use crate::level_map::LevelMap;

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
        .add_systems(Startup, (load_images, setup).chain())
        .add_systems(FixedUpdate, snake::snake_movement_system)
        .run();
}
fn setup(mut commands: Commands, level_map: Res<LevelMap>, images: Res<Images>, windows: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);

    snake::spawn_snake(&mut commands, &images, window_center);

    level_map.draw(commands, &images, window_center);
}

