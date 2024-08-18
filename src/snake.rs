use bevy::prelude::Component;
use bevy::scene::ron::de::Position;

#[derive(Component)]
pub struct Snake {
    pub positions: Vec<Position>
}

impl Snake {
    pub fn initial_snake(&self, starting_position: Position) -> Snake {
        return Snake {
            positions: vec![
                starting_position,
                Position { col: starting_position.col, line: starting_position.line - 1},
                Position { col: starting_position.col, line: starting_position.line - 2},
            ]
        }
    }
}
