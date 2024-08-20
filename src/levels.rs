#![allow(dead_code)]
use bevy::asset::ron::de::Position;

pub const LEVEL_1_SELECT: Position = Position { line: 3, col: 3 };
pub const LEVEL_2_SELECT: Position = Position { line: 3, col: 10 };
pub const LEVEL_3_SELECT: Position = Position { line: 3, col: 17 };

pub const MENU: &str = "#####################\n\
                        #...................#\n\
                        #...................#\n\
                        #..>......>......>..#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #...................#\n\
                        #####################";

pub const LEVEL_1: &str = "#####################\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #....#.........#....#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #....#.........#....#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #####################";

pub const LEVEL_2: &str = "#####################\n\
                           #.........#.........#\n\
                           #.........#.........#\n\
                           #...................#\n\
                           #...................#\n\
                           #....##.......##....#\n\
                           #....#.........#....#\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           ###...............###\n\
                           #...................#\n\
                           #...................#\n\
                           #...................#\n\
                           #....#.........#....#\n\
                           #....##.......##....#\n\
                           #...................#\n\
                           #...................#\n\
                           #.........#.........#\n\
                           #.........#.........#\n\
                           #####################";

pub const LEVEL_3: &str = "#####################\n\
                           #..#...#...#..#..#..#\n\
                           #.#..#...#.#.....#..#\n\
                           #.#....#....#.##....#\n\
                           #..#.#...##......#.##\n\
                           ##.#...#...#..#..#..#\n\
                           #....#..##...##.#...#\n\
                           #..#..#...##......###\n\
                           ##....#.#...#..##...#\n\
                           #..##...##...#.....##\n\
                           ##.#..##.....#.#.#..#\n\
                           #........#.##....#..#\n\
                           ##.#..#..#....#.#..##\n\
                           #.....#.....##...#..#\n\
                           #...#....#.....#....#\n\
                           #.#...#....#.#..#.###\n\
                           #..#.#...#..........#\n\
                           #......#.#...#..##..#\n\
                           #.##.#......#....#..#\n\
                           #......#.#..#..#....#\n\
                           #####################";

pub const STARTING_POSITION: Position = Position {
    line: 10,
    col: 10,
};