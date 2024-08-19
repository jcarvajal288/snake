use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Image, Res, ResMut, Resource};

#[derive(Resource)]
pub struct Images {
    pub wall: Vec<Handle<Image>>,
    pub floor: Vec<Handle<Image>>,
    pub snake_head: Handle<Image>,
    pub snake_tail: Handle<Image>,
    pub apple: Handle<Image>,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            wall: vec![Handle::default()],
            floor: vec![Handle::default()],
            snake_head: Handle::default(),
            snake_tail: Handle::default(),
            apple: Handle::default(),
        }
    }
}

pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.wall = vec![
        asset_server.load("embedded://images/wall/brick_brown_0.png"),
        asset_server.load("embedded://images/wall/brick_brown_1.png"),
        asset_server.load("embedded://images/wall/brick_brown_2.png"),
        asset_server.load("embedded://images/wall/brick_brown_3.png"),
        asset_server.load("embedded://images/wall/brick_brown_4.png"),
        asset_server.load("embedded://images/wall/brick_brown_5.png"),
        asset_server.load("embedded://images/wall/brick_brown_6.png"),
        asset_server.load("embedded://images/wall/brick_brown_7.png"),
    ];
    images.floor = vec![
        asset_server.load("embedded://images/floor/grey_dirt_0_new.png"),
        asset_server.load("embedded://images/floor/grey_dirt_1_new.png"),
        asset_server.load("embedded://images/floor/grey_dirt_2_new.png"),
        asset_server.load("embedded://images/floor/grey_dirt_3_new.png"),
        asset_server.load("embedded://images/floor/grey_dirt_4_new.png"),
        asset_server.load("embedded://images/floor/grey_dirt_5_new.png"),
        asset_server.load("embedded://images/floor/grey_dirt_6_new.png"),
        asset_server.load("embedded://images/floor/grey_dirt_7_new.png"),
    ];
    images.snake_head = asset_server.load("embedded://images/marble_wall_11.png");
    images.snake_tail = asset_server.load("embedded://images/crystal_wall_lightcyan.png");
    images.apple = asset_server.load("embedded://images/apple.png");
}