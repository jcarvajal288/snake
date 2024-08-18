use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{Image, Res, ResMut, Resource};

#[derive(Resource)]
pub struct Images {
    pub wall: Handle<Image>,
    pub floor: Handle<Image>,
    pub snake_head: Handle<Image>,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            wall: Handle::default(),
            floor: Handle::default(),
            snake_head: Handle::default(),
        }
    }
}

pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.wall = asset_server.load("images/brick_brown_0.png");
    images.floor = asset_server.load("images/grey_dirt_0_new.png");
    images.snake_head = asset_server.load("images/marble_wall_11.png");
}