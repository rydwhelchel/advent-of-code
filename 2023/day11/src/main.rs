#![allow(unused, unused_variables, unused_imports)]

use std::{fs::File, io::BufReader, io::prelude::*};

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Coords>,
    expanded_universe: Vec<Vec<char>>
}

impl Universe {
    fn new(lines: Vec<Vec<char>>) -> Self {
        let expanded_universe = Self::expand_universe(lines);
        let galaxies = Self::get_galaxies(&expanded_universe);
        Universe { galaxies, expanded_universe }
    }

    fn expand_universe(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
        // double the vertical rows which do not contain a galaxy
        let mut x = 0;
        let mut universe = lines.clone();
        while x < universe.get(0).unwrap().len() {
            let mut y = 0;
            let mut contains_galaxy = false;
            while y < universe.len() {
                if universe.get_at(x, y) == '#' {
                    contains_galaxy = true;
                    break;
                }
                y += 1;
            }
            if !contains_galaxy {
                let mut y = 0;
                while y < universe.len() {
                    universe.get_mut(y).unwrap().insert(x, '.');
                    y += 1;
                }
                x += 1;
            }
            x += 1;
        }

        // double horizontal rows which do not contain a galaxy
        let mut y = 0;
        while y < universe.len() {
            let mut x = 0;
            let mut contains_galaxy = false;
            while x < universe.get(0).unwrap().len() {
                if universe.get_at(x, y) == '#' {
                    contains_galaxy = true;
                    break;
                }
                x += 1;
            }
            if !contains_galaxy {
                let length = universe.get(0).unwrap().len();
                universe.insert(y, vec!['.'; length]);
                y += 1;
            }
            y += 1;
        }

        universe
    }

    fn get_galaxies(universe: &Vec<Vec<char>>) -> Vec<Coords> {
        let mut y = 0;
        let mut galaxies: Vec<Coords> = Vec::new();
        while y < universe.len() {
            let mut x = 0;
            while x < universe.get(0).unwrap().len() {
                if universe.get_at(x, y) == '#' {
                    galaxies.push(Coords { x, y });
                }
                x+=1;
            }
            y+=1;
        }
        galaxies
    }

    fn part1(&self) -> usize {
        let mut sum: usize = 0;
        let galaxies = &self.galaxies;
        let mut x = 0;
        while x < galaxies.len() - 1 {
            let galaxy_one = galaxies.get(x).unwrap();
            let x_1 = galaxy_one.x as isize;
            let y_1 = galaxy_one.y as isize;
            let mut y = x + 1;
            while y < galaxies.len() {
                let galaxy_two = galaxies.get(y).unwrap();
                let x_2 = galaxy_two.x as isize;
                let y_2 = galaxy_two.y as isize;
                let dist = abs(x_2-x_1) + abs(y_2-y_1);
                sum += dist;
                // println!("The distance between ({}, {}) and ({}, {}) is {}", x_1, y_1, x_2, y_2, dist);
                y += 1;
            }
            x += 1;
        }
        sum
    }
}

fn get_expanded_columns(universe: &Vec<Vec<char>>) -> Vec<usize> {
    let mut expanses: Vec<usize> = Vec::new();
    let mut x = 0;
    while x < universe.get(0).unwrap().len() {
        let mut y = 0;
        let mut contains_galaxy = false;
        while y < universe.len() {
            if universe.get_at(x, y) == '#' {
                contains_galaxy = true;
                break;
            }
            y += 1;
        }
        if !contains_galaxy {
            expanses.push(x);
        }
        x += 1;
    }
    expanses
}

fn get_expanded_rows(universe: &Vec<Vec<char>>) -> Vec<usize> {
    let mut expanses: Vec<usize> = Vec::new();
    let mut y = 0;
    while y < universe.len() {
        let mut x = 0;
        let mut contains_galaxy = false;
        while x < universe.get(0).unwrap().len() {
            if universe.get_at(x, y) == '#' {
                contains_galaxy = true;
                break;
            }
            x += 1;
        }
        if !contains_galaxy {
            expanses.push(y);
        }
        y += 1;
    }
    expanses
}

//returns the number of expanses it crosses
fn expanse_crossings(x1: usize, x2: usize, expanse_list: &[usize]) -> usize {
    let mut min = min(x1, x2);
    let max = max(x1, x2);

    let mut crossings = 0;
    while min < max {
        if expanse_list.contains(&min) {
            crossings += 1;
        }
        min += 1;
    }
    crossings
}

fn min(x1: usize, x2: usize) -> usize {
    if x1 < x2 {
        return x1;
    }
    x2
}

fn max(x1: usize, x2: usize) -> usize {
    if x1 > x2 {
        return x1;
    }
    x2
}

fn abs(x: isize) -> usize { 
    if x < 0 {
        return (-x).try_into().unwrap();
    }
    x.try_into().unwrap()
}

trait GetAt {
    fn get_at(&self, x: usize, y: usize) -> char;
}

impl GetAt for Vec<Vec<char>> {
    fn get_at(&self, x: usize, y: usize) -> char {
        *self.get(y).unwrap().get(x).unwrap()
    }
}

fn part2(universe: Vec<Vec<char>>) -> usize {
    let galaxies = Universe::get_galaxies(&universe);
    let expanded_columns = get_expanded_columns(&universe);
    let expanded_rows = get_expanded_rows(&universe);

    let mut sum: usize = 0;
    let mut x = 0;
    while x < galaxies.len() - 1 {
        let galaxy_one = galaxies.get(x).unwrap();
        let x_1 = galaxy_one.x as isize;
        let y_1 = galaxy_one.y as isize;
        let mut y = x + 1;
        while y < galaxies.len() {
            let galaxy_two = galaxies.get(y).unwrap();
            let x_2 = galaxy_two.x as isize;
            let y_2 = galaxy_two.y as isize;

            let x_crossings = expanse_crossings(galaxy_one.x, galaxy_two.x, &expanded_columns);
            let y_crossings = expanse_crossings(galaxy_one.y, galaxy_two.y, &expanded_rows);

            let mut dist = abs(x_2-x_1) + abs(y_2-y_1);
            dist = (dist - x_crossings) + x_crossings*1000000;
            dist = (dist - y_crossings) + y_crossings*1000000;

            sum += dist;
            // println!("The distance between ({}, {}) and ({}, {}) is {}", x_1, y_1, x_2, y_2, dist);
            y += 1;
        }
        x += 1;
    }
    sum

}

fn main() -> std::io::Result<()> {
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines()
        .flatten()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let universe = Universe::new(lines.clone());
    println!("The result of part1 is: {}", universe.part1());
    println!("The result of part1 is: {}", part2(lines));


    Ok(())
}

fn get_test_data() -> Vec<Vec<char>> {
    let path: &str = "example.txt";
    let file: File = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    reader.lines()
        .flatten()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[test]
fn test_expand_universe() {
    let lines = get_test_data();
    let result_universe = Universe::expand_universe(lines.clone());
    let expected_line_1:Vec<char> = "....#........".chars().collect();
    let expected_line_11: Vec<char> = ".........#...".chars().collect();
    let expected_length = lines.len()+2;
    let expected_width = lines.get(0).unwrap().len()+3;


    assert_eq!(&expected_line_1, result_universe.get(0).unwrap());
    assert_eq!(&expected_line_11, result_universe.get(10).unwrap());
    assert_eq!(expected_width, result_universe.get(0).unwrap().len());
    assert_eq!(expected_length, result_universe.len());
}

#[test]
fn test_get_galaxies() {
    let lines = get_test_data();
    let universe = Universe::expand_universe(lines);
    let galaxies = Universe::get_galaxies(&universe);

    println!("The galaxies are: {:?}", galaxies);

    assert_eq!(9, galaxies.len());
}

#[test]
fn test_part1() {
    let lines = get_test_data();
    let universe = Universe::new(lines);
    let expected = 374;

    assert_eq!(expected, universe.part1())
}
