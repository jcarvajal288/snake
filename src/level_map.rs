use bevy::asset::Handle;
use bevy::asset::ron::de::Position;
use bevy::prelude::{Commands, Component, Image, Res, Resource, Transform, Vec2};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;

use crate::images::Images;

#[path = "levels/level1.rs"] mod level1;

const MAP_WIDTH: usize = 23;
const MAP_HEIGHT: usize = 23;
pub const TILE_SIZE: f32 = 32.0;

#[derive(Clone)]
pub enum Tile {
    WALL,
    FLOOR,
}

#[derive(Component)]
pub struct Apple {
    pub position: Position
}

#[derive(Resource)]
pub struct LevelMap {
    pub grid: Vec<Vec<Tile>>
}

impl Default for LevelMap {
    fn default() -> Self {
        Self {
            grid: vec![vec![Tile::WALL; MAP_HEIGHT]; MAP_WIDTH],
        }
    }
}

impl LevelMap {
    pub fn draw(&self, mut commands: Commands, images: &Res<Images>, window_center: Vec2) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                commands.spawn(SpriteBundle {
                    texture: get_image_for_tile(self.grid.get(y).unwrap().get(x).unwrap(), images),
                    transform: transform_from_position(&Position { line: y, col: x }, window_center, 0.0),
                    ..default()
                });
            }
        }
    }

    pub fn is_position_walkable(&self, position: &Position) -> bool {
        return match self.grid.get(position.line).unwrap_or(&vec!(Tile::WALL))
            .get(position.col).unwrap_or(&Tile::WALL) {
                Tile::FLOOR => true,
                _ => false
        }
    }
}

pub fn load_level_1() -> LevelMap {
    return LevelMap {
        grid: read_level_tiles(level1::MAP_DATA),
    }
}

pub fn transform_from_position(position: &Position, window_center: Vec2, zindex: f32) -> Transform {
    let half_tile_size = TILE_SIZE / 2.0;
    return Transform::from_xyz(
        (position.col as f32 * TILE_SIZE) - window_center.x + half_tile_size,
        -(position.line as f32 * TILE_SIZE) + window_center.y - half_tile_size,
        zindex
    )
}

fn get_image_for_tile(tile: &Tile, images: &Res<Images>) -> Handle<Image> {
    return match tile {
        Tile::WALL => images.wall.clone(),
        Tile::FLOOR => images.floor.clone(),
    }
}

fn read_level_tiles(map_data: &str) -> Vec<Vec<Tile>> {
    return map_data.lines().map(|line| {
        return line.chars().map(|character| {
            return match character {
                '#' => Tile::WALL,
                '.' => Tile::FLOOR,
                _   => Tile::FLOOR,
            }
        }).collect()
    }).collect();
}
