#![allow(unused, unused_variables)]

use std::{fs::File, io::BufReader, io::prelude::*};

enum Direction {
    Up, Down, Left, Right
}

struct Coords {
    x: usize,
    y: usize
}

impl Coords {
    fn move_dir(&self, direction: Direction) -> Coords {
        match direction {
            Direction::Left => Coords { x: self.x - 1, y: self.y },
            Direction::Right => Coords { x: self.x + 1, y: self.y },
            Direction::Up => Coords { x: self.x, y: self.y - 1 },
            Direction::Down => Coords { x: self.x, y: self.y + 1 }  
        }
    }

    fn max() -> usize {
        // 141 columns and rows, saying max is 140 to simplify comparison to index
        140
    }

    fn max_x(&self) -> bool {
        self.x == Coords::max()
    }
    
    fn max_y(&self) -> bool {
        self.y == Coords::max()
    }

    fn min_x(&self) -> bool {
        self.x == 0
    }
    
    fn min_y(&self) -> bool {
        self.y == 0
    }
}

struct Path {
    direction: Direction,
    consecutive: usize
}

trait GetAt {
    fn get_at(&self, coords: &Coords) -> Option<char>;
}

impl GetAt for Vec<Vec<char>> {
    fn get_at(&self, coords: &Coords) -> Option<char> {
        match self.get(coords.y) {
            Some(row) => match row.get(coords.x) {
                Some(val) => Some(*val),
                _ => None
            },
            _ => None
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn get_input_data() -> Vec<Vec<char>> {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let input = reader.lines()
        .flatten()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    input
}
