mod images;
mod level_map;
mod snake;
#[path = "levels/level1.rs"] mod level1;

use bevy::DefaultPlugins;
use bevy::math::Vec2;
use bevy::prelude::{App, Camera2dBundle, ClearColor, Commands, default, IntoSystemConfigs, PluginGroup, Query, Res, ResMut, SpriteBundle, Startup, Window};
use bevy::window::{WindowPlugin, WindowResolution};
use crate::images::{Images, load_images};
use crate::level_map::LevelMap;
use crate::snake::Snake;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(736., 736.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Images::default())
        .insert_resource(level_map::load_level_1())
        .add_systems(Startup, (load_images, setup).chain())
        //.add_systems(Update, player_movement_system)
        .run();
}
fn setup(mut commands: Commands, level_map: Res<LevelMap>, images: Res<Images>, windows: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    let window_center = Vec2::new(windows.single().resolution.width() / 2., windows.single().resolution.height() / 2.);

    commands.spawn((
        SpriteBundle {
            texture: images.snake_head.clone(),
            transform: level_map::transform_from_position(&level1::STARTING_POSITION, window_center, 1.0),
            ..default()
        },
        Snake {
            positions: vec![level1::STARTING_POSITION],
        }
    ));

    level_map.draw(commands, &images, window_center);
}
