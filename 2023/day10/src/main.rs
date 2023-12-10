#![allow(unused, unused_variables, unused_imports)]

use std::{fs::File, io::BufReader, io::prelude::*, clone, collections::HashSet};

#[derive(Debug, Clone, Eq)]
struct Coords {
    x: usize,
    y: usize
}

impl Coords {
    fn forward(&self) -> Coords {
        Coords {
            x: self.x + 1,
            y: self.y
        }
    }

    fn back(&self) -> Coords {
        Coords {
            x: self.x - 1,
            y: self.y
        }
    }

    fn up(&self) -> Coords{
        Coords {
            x: self.x,
            y: self.y - 1
        }
    }

    fn down(&self) -> Coords {
        Coords {
            x: self.x,
            y: self.y + 1
        }
    }
}

impl PartialEq for Coords {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn get_next_indices(pipe: char, index: &Coords) -> Result<(Coords, Coords), &'static str> {
    match pipe {
        'F' => Ok((index.forward(), index.down())),
        'L' => Ok((index.up(), index.forward())),
        '7' => Ok((index.back(), index.down())),
        'J' => Ok((index.back(), index.up())),
        '-' => Ok((index.back(), index.forward())),
        '|' => Ok((index.up(), index.down())),
        _ => Err("Non-pipe character encountered")
    }
}

fn is_up_down(pipe: char, up: bool) -> bool {
    // observing the input data, S is in the position of F
    let pipe = if pipe == 'S' { 'F' } else { pipe };
    if (up) {
        return matches!(pipe, '7' | '|' | 'F')
    } else {
        return matches!(pipe, '|' | 'F' | '7')
    }
    false
}

trait GetAt {
    fn get_pipe_at(&self, coords: &Coords) -> char;
    fn get_starting_pos(&self) -> Result<Coords, &'static str>;
    fn get_starting_pipe(&self, starting_index: &Coords) -> Result<Coords, &'static str>;
}

impl GetAt for Vec<Vec<char>> {
    fn get_pipe_at(&self, coords: &Coords) -> char {
        *self.get(coords.y).unwrap()
            .get(coords.x).unwrap()
    }

    fn get_starting_pos(&self) -> Result<Coords, &'static str> {
        for (i, y) in self.iter().enumerate() {
            for (j, x) in y.iter().enumerate() {
                if *x == 'S' {
                    return Ok(Coords { x: j, y: i })
                }
            }
        }

        Err("Did not find the starting pos")
    }

    fn get_starting_pipe(&self, starting_coords: &Coords) -> Result<Coords, &'static str> {
        if ['J', '7', '-'].contains(&self.get_pipe_at(&starting_coords.forward())) {
            return Ok(starting_coords.forward());
        } else if ['J', 'L', '|'].contains(&self.get_pipe_at(&starting_coords.down())) {
            return Ok(starting_coords.down());
        } else if ['F', 'L', '-'].contains(&self.get_pipe_at(&starting_coords.back())) {
            return Ok(starting_coords.back());
        } else if ['F', '7', '|'].contains(&self.get_pipe_at(&starting_coords.up())) {
            return Ok(starting_coords.up());
        }

        Err("Could not get starting pipe")
    }
}

fn part1(lines: &Vec<Vec<char>>) -> u32 {
    let starting_index = lines.get_starting_pos().unwrap();
    let mut prev_index = starting_index.clone();
    let mut steps = 1;
    let mut curr_index = lines.get_starting_pipe(&starting_index).unwrap();

    while curr_index != starting_index {
        let next_indices = get_next_indices(lines.get_pipe_at(&curr_index), &curr_index).unwrap();
        let temp = curr_index.clone();
        curr_index = if next_indices.0 == prev_index { next_indices.1 } else { next_indices.0 };
        prev_index = temp;
        steps += 1;
    }

    steps / 2
}

//recursive solution is no go
fn part2(lines: &Vec<Vec<char>>) -> u32 {
    let starting_index = lines.get_starting_pos().unwrap();
    let mut prev_index = starting_index.clone();
    let mut curr_index = lines.get_starting_pipe(&starting_index).unwrap();
    let mut circuit: Vec<Coords> = vec![prev_index.clone()];
    let mut inside_count: u32 = 0;

    //construct circuit
    while curr_index != starting_index {
        circuit.push(curr_index.clone());
        let next_indices = get_next_indices(lines.get_pipe_at(&curr_index), &curr_index).unwrap();
        let temp = curr_index.clone();
        curr_index = if next_indices.0 == prev_index { next_indices.1 } else { next_indices.0 };
        prev_index = temp;
    }
    
    //true is up, false is down
    // if currently last is down, then we're outside (false == outside)
    let mut up: bool = false;
    for (y, line) in lines.iter().enumerate() {
        for (x, elem) in line.iter().enumerate() {
            let coords = Coords { x, y };
            if circuit.contains(&coords) {
                if is_up_down(lines.get_pipe_at(&coords), up) {
                    up = !up;
                } else {
                    continue;
                }
            } else if up {
                inside_count += 1;
            }
        }
    }
    inside_count
}

fn main() -> std::io::Result<()> {
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .map(|x| x.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("The result for part1 is: {}", part1(&lines));
    println!("The result for part2 is: {}", part2(&lines));
    
    Ok(())
}

fn get_test_data() -> Vec<Vec<char>> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .map(|x| x.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_test_data_2() -> Vec<Vec<char>> {
    let path = "example2.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .map(|x| x.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[test]
fn test_get_starting_pos() {
    let lines = get_test_data();
    let result_coords = lines.get_starting_pos();
    let expected_coords = Coords {
        x: 0,
        y: 2
    };

    assert_eq!(&expected_coords.x, &result_coords.as_ref().unwrap().x);
    assert_eq!(&expected_coords.y, &result_coords.as_ref().unwrap().y);
}

#[test]
fn test_get_next_indices() {
    let lines = get_test_data();
    let f_pipe = Coords {
        x: 2,
        y: 0
    };
    println!("F-pipe: start {:?}", f_pipe);
    println!("F-pipe next index: {:?}", get_next_indices('F', &f_pipe).unwrap());
    let j_pipe = Coords {
        x: 2,
        y: 1
    };
    println!("J-pipe: start {:?}", j_pipe);
    println!("J-pipe next index: {:?}", get_next_indices('J', &j_pipe).unwrap());
    let pipe_pipe = Coords {
        x: 3,
        y: 1
    };
    println!("Pipe-pipe: start {:?}", pipe_pipe);
    println!("Pipe-pipe next index: {:?}", get_next_indices('|', &pipe_pipe).unwrap());
    let horiz_pipe = Coords {
        x: 2,
        y: 3
    };
    println!("Horiz-pipe: start {:?}", horiz_pipe);
    println!("Horiz-pipe next index: {:?}", get_next_indices('-', &horiz_pipe).unwrap());
    let l_pipe = Coords {
        x: 3,
        y: 2
    };
    println!("L-pipe: start {:?}", l_pipe);
    println!("L-pipe next index: {:?}", get_next_indices('L', &l_pipe).unwrap());
    let seven_pipe = Coords {
        x: 4,
        y: 2
    };
    println!("7-pipe: start {:?}", seven_pipe);
    println!("7-pipe next index: {:?}", get_next_indices('7', &seven_pipe).unwrap());
}

#[test]
fn test_coord_equality() {
    let coord1 = Coords {
        x: 1,
        y: 1
    };
    let coord2 = Coords {
        y: 1,
        x: 1
    };
    let coord3 = Coords {
        x: 2,
        y: 1
    };

    assert!(coord1 == coord2);
    assert!(coord3 != coord2);
    assert!(coord2 == coord1);
    assert!(coord3 != coord1);
}

#[test]
fn test_part2() {
    let lines = get_test_data_2();
    let result = part2(&lines);
    let expected = 10;

    assert_eq!(expected, result)
}
