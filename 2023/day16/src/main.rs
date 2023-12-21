// Part1: Goal -- create a second grid which represents the tiles which are energized and count up
// the resulting tiles
//  methodology: Create a beam object which has a direction (up, down left or right), and a forward
//  method. On every tile, the beam energizes its current tile and then moves forward  
//  If a beam hits a mirror on a forward action, it's direction is manipulated.
//  If a beam hits the flat side of a splitter on a forward action, then another instance of Beam
//  is created and continues the opposite direction from the original
#![allow(clippy::single_match, clippy::unnecessary_unwrap)]

use std::{fs::File, io::BufReader, io::prelude::*};

#[derive(Clone, PartialEq)]
struct Coords {
    x: usize,
    y: usize
}

struct EnergizedTiles {
    map: Vec<Vec<char>>
}

impl EnergizedTiles {
    fn new(input: &Vec<Vec<char>>) -> Self {
        let length = input.len();
        let width = input.get(0).unwrap().len();
        let map = vec![vec!['.'; width]; length];

        EnergizedTiles {
            map
        }
    }

    fn energize(&mut self, coords: &Coords) {
        if self.map.get_at(coords) == Some('.') {
            self.map.get_mut(coords.y).unwrap().remove(coords.x);
            self.map.get_mut(coords.y).unwrap().insert(coords.x, '#');
        }
    }

    fn sum(&self) -> usize {
        let mut sum = 0;
        for row in &self.map {
            for elem in row {
                if *elem == '#' {
                    sum += 1;
                }
            }
        }
        sum
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up, Down, Left, Right
}

#[derive(PartialEq, Clone)]
struct Beam {
    coords: Coords,
    direction: Direction,
    alive: bool
}

impl Beam {
    fn new(coords: Coords, direction: Direction) -> Self {
        Beam {
            coords,
            direction,
            alive: true
        }
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }

    fn forward(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.coords.y == 0 {
                    self.kill();
                    return;
                }
                self.coords.y -= 1
            },
            Direction::Down => self.coords.y += 1,
            Direction::Left => {
                if self.coords.x == 0 {
                    self.kill();
                    return;
                }
                self.coords.x -= 1
            },
            Direction::Right => self.coords.x += 1
        } 
    }

    fn right(&mut self) -> Option<Beam> {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Up,
            Direction::Down => self.direction = Direction::Left,
            Direction::Right => self.direction = Direction::Down
        };
        None
    }

    fn left(&mut self) -> Option<Beam> {
        match self.direction {
            Direction::Up => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Up
        };
        None
    }

    fn check_loc(&mut self, input: &mut Vec<Vec<char>>, energized: &mut EnergizedTiles) -> Option<Beam> {
        let curr = input.get_at(&self.coords);
        if curr.is_some() {
            energized.energize(&self.coords);
        }
        match curr {
            Some(val) => match val {
                '/' => {
                    match self.direction {
                        Direction::Right => self.left(),
                        Direction::Up => self.right(),
                        Direction::Down => self.right(),
                        Direction::Left => self.left()
                    }
                },
                '\\' => {
                    match self.direction {
                        Direction::Right => self.right(),
                        Direction::Up => self.left(),
                        Direction::Down => self.left(),
                        Direction::Left => self.right()
                    }
                },
                '|' => {
                    match self.direction {
                        Direction::Right => {
                            let mut new_beam = Beam::new(self.coords.clone(), Direction::Up);
                            self.direction = Direction::Down;
                            new_beam.forward();
                            Some(new_beam)
                        }
                        Direction::Left => {
                            let mut new_beam = Beam::new(self.coords.clone(), Direction::Up);
                            self.direction = Direction::Down;
                            new_beam.forward();
                            Some(new_beam)
                        },
                        _ => None
                    }
                },
                '-' => {
                    match self.direction {
                        Direction::Up => {
                            let mut new_beam = Beam::new(self.coords.clone(), Direction::Left);
                            self.direction = Direction::Right;
                            new_beam.forward();
                            Some(new_beam)
                        }
                        Direction::Down => {
                            let mut new_beam = Beam::new(self.coords.clone(), Direction::Left);
                            self.direction = Direction::Right;
                            new_beam.forward();
                            Some(new_beam)
                        },
                        _ => None
                    }
                },
                _ => None
            },
            _ => {
                self.kill();
                None
            }
        }
    }
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

fn part1(input: &mut Vec<Vec<char>>, x: usize, y: usize, direction: Direction) -> usize {
    let mut beams: Vec<Beam> =  Vec::new();
    let mut seen = beams.clone();
    let initial_beam: Beam = Beam::new(Coords { x, y }, direction);
    beams.push(initial_beam);
    let energized: &mut EnergizedTiles = &mut EnergizedTiles::new(input);
    while let Some(mut beam) = beams.pop() {
        while beam.is_alive() {
            let new_beam = beam.check_loc(input, energized);
            if new_beam.is_some() {
                let new_beam = new_beam.unwrap();
                beams.push(new_beam);
            }

            if seen.contains(&beam) {
                break;
            }

            if !beam.is_alive() {
                break;
            }
            seen.push(beam.clone());
            beam.forward();
        }
    }

    energized.sum()
}

fn part2(input: &mut Vec<Vec<char>>) -> usize {
    let mut sums = Vec::new();
    //top row
    let y = 0;
    let mut x = 0;
    while x < input[y].len() {
        sums.push(part1(input, x, y, Direction::Down));
        x += 1;
    }

    //bottom row
    let y = input.len() - 1;
    let mut x = 0;
    while x < input[y].len() {
        sums.push(part1(input, x, y, Direction::Up));
        x += 1;
    }

    //left column
    let x = 0;
    let mut y = 0;
    while y < input.len() {
        sums.push(part1(input, x, y, Direction::Right));
        y += 1;
    }

    //bottom row
    let x = input[0].len() - 1;
    let mut y = 0;
    while y < input.len() {
        sums.push(part1(input, x, y, Direction::Left));
        y += 1;
    }
    sums.iter().fold(std::usize::MIN, |a, b| a.max(*b))
}

fn main() {
    let mut input = get_input_data();
    let part1 = part1(&mut input, 0, 0, Direction::Right);
    println!("The result of part1 is {}", part1);
    let mut input = get_input_data();
    println!("The result of part2 is {}", part2(&mut input))
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

#[allow(dead_code)]
fn get_test_data() -> Vec<Vec<char>> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let input = reader.lines()
        .flatten()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    input
}

#[test]
fn test_part1() {
    let mut input = get_test_data();
    let part1 = part1(&mut input);
    assert_eq!(part1, 46)
}

