// need to make a data structure for coordinate
// need to make a method to get coordinate north from current coordinate
// process the layout by going through each rock, setting its current pos to .
//  and getting the north-most coordinate it can roll to, this must be processed
//  top to bottom
// calculate sum by counting each rock as having the value (lines.len() - y)
// if can_roll -> roll
#![allow(unused, unused_variables, unused_imports)]

use std::fmt::Display;
use std::{fs::File, io::BufReader};
use std::io::prelude::*;

#[derive(Debug)]
struct BoulderMap {
    layout: Vec<Vec<char>>
}

impl BoulderMap {
    fn new(lines: &Vec<Vec<char>>) -> Self {
        BoulderMap {
            layout: lines.clone()
        }
    }

    fn can_roll_up(&mut self, coords: &Coords) -> bool {
        let north = &coords.up();
        match north {
            Some(x) => match self.get_at(x) {
                Some('.') => return true,
                _ => return false
            },
            None => return false
        }
    }

    fn can_roll_down(&mut self, coords: &Coords) -> bool {
        let south = &coords.down();
        match south {
            Some(x) => match self.get_at(x) {
                Some('.') => return true,
                _ => return false
            },
            None => return false
        }
    }

    fn can_roll_left(&mut self, coords: &Coords) -> bool {
        let west = &coords.left();
        match west {
            Some(x) => match self.get_at(x) {
                Some('.') => return true,
                _ => return false
            },
            None => return false
        }
    }

    fn can_roll_right(&mut self, coords: &Coords) -> bool {
        let east = &coords.right();
        match east {
            Some(x) => match self.get_at(x) {
                Some('.') => return true,
                _ => return false
            },
            None => return false
        }
    }

    fn get_at(&mut self, coords: &Coords) -> Option<char> {
        match self.layout.get(coords.y) {
            Some(x) => x.get(coords.x).copied(),
            None => None
        }
    }

    fn set_at(&mut self, coords: &Coords, new: char) {
        self.layout.get_mut(coords.y).unwrap().remove(coords.x);
        self.layout.get_mut(coords.y).unwrap().insert(coords.x, new);
    }

    fn roll_up(&mut self, coords: Coords) {
        // coords is the boulder that needs to roll
        assert!(self.get_at(&coords).unwrap() == 'O');
        self.set_at(&coords, '.');
        let mut curr = coords;
        while self.can_roll_up(&curr) {
            curr = curr.up().unwrap();
        }
        self.set_at(&curr, 'O');
    }

    fn roll_down(&mut self, coords: Coords) {
        // coords is the boulder that needs to roll
        assert!(self.get_at(&coords).unwrap() == 'O');
        self.set_at(&coords, '.');
        let mut curr = coords;
        while self.can_roll_down(&curr) {
            curr = curr.down().unwrap();
        }
        self.set_at(&curr, 'O');
    }

    fn roll_left(&mut self, coords: Coords) {
        // coords is the boulder that needs to roll
        assert!(self.get_at(&coords).unwrap() == 'O');
        self.set_at(&coords, '.');
        let mut curr = coords;
        while self.can_roll_left(&curr) {
            curr = curr.left().unwrap();
        }
        self.set_at(&curr, 'O');
    }

    fn roll_right(&mut self, coords: Coords) {
        // coords is the boulder that needs to roll
        assert!(self.get_at(&coords).unwrap() == 'O');
        self.set_at(&coords, '.');
        let mut curr = coords;
        while self.can_roll_right(&curr) {
            curr = curr.right().unwrap();
        }
        self.set_at(&curr, 'O');
    }

    fn roll_all_up(&mut self) {
        let length = self.layout.len();
        let depth = self.layout.get(0).unwrap().len();
        let mut y = 0;
        while y < length {
            let mut x = 0;
            while x < depth {
                let coords = Coords {
                    x,
                    y
                };
                if self.get_at(&coords) == Some('O') {
                    self.roll_up(coords);
                }
                x += 1;
            } 
            y += 1;
        }
    }

    fn roll_all_down(&mut self) {
        let length = self.layout.len();
        let depth = self.layout.get(0).unwrap().len();
        let mut y = 0;
        while y < length {
            let mut x = 0;
            while x < depth {
                let coords = Coords {
                    x,
                    y
                };
                if self.get_at(&coords) == Some('O') {
                    self.roll_down(coords);
                }
                x += 1;
            } 
            y += 1;
        }
    }

    fn roll_all_left(&mut self) {
        let length = self.layout.len();
        let depth = self.layout.get(0).unwrap().len();
        let mut y = 0;
        while y < length {
            let mut x = 0;
            while x < depth {
                let coords = Coords {
                    x,
                    y
                };
                if self.get_at(&coords) == Some('O') {
                    self.roll_left(coords);
                }
                x += 1;
            } 
            y += 1;
        }
    }

    fn roll_all_right(&mut self) {
        let length = self.layout.len();
        let depth = self.layout.get(0).unwrap().len();
        let mut y = 0;
        while y < length {
            let mut x = 0;
            while x < depth {
                let coords = Coords {
                    x,
                    y
                };
                if self.get_at(&coords) == Some('O') {
                    self.roll_right(coords);
                }
                x += 1;
            } 
            y += 1;
        }
    }

    fn cycle(&mut self) {
        self.roll_all_up();
        self.roll_all_left();
        self.roll_all_right();
        self.roll_all_down();
    }

    fn count_score(&self) -> usize {
        let length = self.layout.len();
        let mut curr_multiplier = length;
        let mut sum = 0;
        while curr_multiplier > 0 {
            let curr_row = self.layout.get(length - curr_multiplier).unwrap();
            for elem in curr_row {
                if elem == &'O' {
                    sum += curr_multiplier;
                }
            }
            curr_multiplier -= 1;
        }
        sum
    }
}

impl Display for BoulderMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Values:\n")?;
        for line in &self.layout {
            writeln!(f, "{}", line.iter().collect::<String>());
        }
        Ok(())
    }
}

struct Coords {
    x: usize,
    y: usize
}

impl Coords {
    fn up(&self) -> Option<Self> {
        if self.y == 0 {
            return None;
        }
        Some(Coords { x: self.x, y: self.y - 1 })
    }

    fn down(&self) -> Option<Self> {
        Some(Coords { x: self.x, y: self.y + 1 })
    }

    fn left(&self) -> Option<Self> {
        if self.x == 0 {
            return None;
        }
        Some(Coords { x: self.x - 1, y: self.y })
    }

    fn right(&self) -> Option<Self> {
        Some(Coords { x: self.x + 1, y: self.y })
    }
}

fn part1(lines: &Vec<Vec<char>>) -> usize {
    let mut boulder_map = BoulderMap::new(lines);
    boulder_map.roll_all_up();
    boulder_map.count_score()
}

fn part2(lines: &Vec<Vec<char>>) -> usize {
    let mut boulder_map = BoulderMap::new(lines);
    let mut repeats = 1000000000;
    while repeats > 0 {
        let so_far = 1000000000 - repeats;
        if so_far % 100000 == 0 {
            println!("Repeated {} times. {}% done", so_far, so_far/1000000000);
        }
        boulder_map.cycle();
        repeats -= 1;
    }
    boulder_map.count_score()
}

fn main() {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .flatten()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("The result of part1 is: {}", part1(&lines));
    println!("The result of part2 is: {}", part2(&lines));
}

fn get_test_data() -> Vec<Vec<char>> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .flatten()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[test]
fn test_roll() {
    let lines = get_test_data();
    let mut boulder_map = BoulderMap::new(&lines);
    println!("The unmodified boulder map is :\n{}", boulder_map);
    boulder_map.roll_up(Coords {
        x: 2,
        y: 1
    });
    println!("The new boulder map is :\n{}", boulder_map);
    boulder_map.roll_up(Coords {
        x: 5,
        y: 5
    });
    println!("The new boulder map is :\n{}", boulder_map);
}

#[test]
fn test_part1() {
    let lines = get_test_data();
    let result = part1(&lines);
    assert_eq!(136, result);
}

#[test]
fn test_part2() {
    let lines = get_test_data();
    let result = part2(&lines);
    assert_eq!(64, result)
}
