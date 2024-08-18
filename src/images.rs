use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Image, Res, ResMut, Resource};

#[derive(Resource)]
pub struct Images {
    pub wall: Handle<Image>,
    pub floor: Handle<Image>,
    pub snake_head: Handle<Image>,
    pub snake_tail: Handle<Image>,
    pub apple: Handle<Image>,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            wall: Handle::default(),
            floor: Handle::default(),
            snake_head: Handle::default(),
            snake_tail: Handle::default(),
            apple: Handle::default(),
        }
    }
}

pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.wall = asset_server.load("embedded://images/brick_brown_0.png");
    images.floor = asset_server.load("embedded://images/grey_dirt_0_new.png");
    images.snake_head = asset_server.load("embedded://images/marble_wall_11.png");
    images.snake_tail = asset_server.load("embedded://images/crystal_wall_lightcyan.png");
    images.apple = asset_server.load("embedded://images/apple.png");
}