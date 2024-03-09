#![allow(unused, unused_variables)]

use std::{cmp::Ordering, collections::HashMap, fs::File, io::prelude::*, io::BufReader};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn move_dir(&self, direction: Direction) -> Coords {
        match direction {
            Direction::Left => Coords {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coords {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Up => Coords {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Coords {
                x: self.x,
                y: self.y + 1,
            },
        }
    }

    fn max() -> usize {
        // 141 columns and rows, saying max is 140 to simplify comparison to index
        // 12 for test
        12 
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

    fn get_neighbor_coords(&self, directions: Vec<Direction>) -> Vec<Coords> {
        let mut coords: Vec<Coords> = Vec::new();
        for direction in directions {
            let possible_coord: Option<Coords> = match direction {
                Direction::Up => {
                    if !self.min_y() {
                        Some(Coords {
                            x: self.x,
                            y: self.y - 1,
                        })
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if !self.max_y() {
                        Some(Coords {
                            x: self.x,
                            y: self.y + 1,
                        })
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if !self.min_x() {
                        Some(Coords {
                            x: self.x - 1,
                            y: self.y,
                        })
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if !self.max_x() {
                        Some(Coords {
                            x: self.x + 1,
                            y: self.y,
                        })
                    } else {
                        None
                    }
                }
            };
            if possible_coord != None {
                coords.push(possible_coord.unwrap());
            }
        }
        coords
    }
}

#[derive(Clone, Copy)]
struct Path {
    direction: Direction,
    consecutive: usize,
}

impl Path {
    fn new() -> Self {
        Path {
            consecutive: 0,
            direction: Direction::Right,
        }
    }

    fn get_neighbor_dirs(&self) -> Vec<Direction> {
        let mut directions = match self.direction {
            Direction::Up => {
                vec![Direction::Left, Direction::Right]
            }
            Direction::Down => {
                vec![Direction::Left, Direction::Right]
            }
            Direction::Left => {
                vec![Direction::Up, Direction::Down]
            }
            Direction::Right => {
                vec![Direction::Up, Direction::Down]
            }
        };
        if self.consecutive < 3 {
            directions.push(self.direction);
        }
        directions
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
                _ => None,
            },
            _ => None,
        }
    }
}

// at each node,
//      check if on bounds, if on bounds don't add that direction to neighbors (min, max x or y)
//      check if path.consecutive = 3, if so, don't add path.direction to neighbors
//      calculate neighbors not in visited (update distance to chart for neighbors)
//          recalculate Path (change direction or add to consecutive)
//          Mark current node as the previous node for neighbors if updating distance to
//      go to cheapest neighbor
//      if no cheapest neighbor to go to, go to next element in not visited

fn part1(input: Vec<Vec<char>>) -> usize {
    let max_num = input.len();
    let keys = (0..max_num)
        .map(|y| (0..max_num).map(|x| Coords { x, y }).collect::<Vec<Coords>>())
        .flatten()
        .collect::<Vec<Coords>>();
    let mut visited: Vec<Coords> = Vec::new();
    let mut node_map: HashMap<Coords, usize> = keys
        .iter()
        .map(|x| (*x, usize::MAX))
        .collect::<HashMap<Coords, usize>>();
    let mut node_queue: Vec<Coords> = vec![Coords { x: 0, y: 0 }];
    let mut path: Path = Path::new();

    while !node_queue.is_empty() {
        let curr_node = node_queue.remove(0);
        // Add neighbors to nodes to be processed next
        let neighbors = curr_node.get_neighbor_coords(path.get_neighbor_dirs());
        let mut sorted_neighbors: Vec<(usize, Coords)> = Vec::new();
        for neighbor in neighbors {
            if visited.contains(&neighbor) {
                continue;
            }
            let value_at = input.get_at(&neighbor).unwrap() as usize;
            // add current path length to the path length of the node in question
            // if it is shorter than the previous path length to the node, then update
            let value = input.get_at(&curr_node).unwrap() as usize + value_at;
            if value < *node_map.get(&neighbor).unwrap() {
                node_map.insert(neighbor, value);
            }
            sorted_neighbors.push((value, neighbor))
        }
        visited.push(curr_node);
        sorted_neighbors.sort_by(|a, b| {
            if a.0 < b.0 {
                Ordering::Less
            } else if a.0 > b.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        node_queue.extend(sorted_neighbors.iter().map(|x| x.1));
    }
    *node_map.get(&Coords { x: max_num - 1, y: max_num - 1 }).unwrap()
}

fn main() {
    let input = get_input_data();
    let part1 = part1(input);
    println!("The result of part1 is: {}", part1)
}

fn get_input_data() -> Vec<Vec<char>> {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .flatten()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    input
}

fn get_test_data() -> Vec<Vec<char>> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .flatten()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    input
}

#[test]
fn test_part_1() {
    let input = get_test_data();
    let part1 = part1(input);
    assert_eq!(part1, 123);
}

