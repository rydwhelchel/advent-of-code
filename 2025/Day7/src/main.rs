use std::collections::HashSet;
#[allow(unused)]
use std::env;
use std::fs;

// 0 indexed, positive values going down and right
#[derive(PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
}

impl Coord {
    fn new(x: usize) -> Self {
        Coord { x }
    }

    fn left(&self) -> Self {
        Self::new(self.x - 1)
    }

    fn right(&self) -> Self {
        Self::new(self.x + 1)
    }
}

fn get_splitters(line: &str) -> Vec<Coord> {
    let mut splitters = Vec::new();
    for (ind, ch) in line.chars().enumerate() {
        if ch == '^' {
            splitters.push(Coord::new(ind));
        }
    }

    splitters
}

fn get_next_beams(beams: HashSet<Coord>, splitters: Vec<Coord>) -> (HashSet<Coord>, usize) {
    let mut next_beams = HashSet::new();
    let mut splits: usize = 0;

    for beam in beams {
        if splitters.contains(&beam) {
            next_beams.insert(beam.left());
            next_beams.insert(beam.right());
            splits += 1;
        } else {
            next_beams.insert(beam);
        }
    }

    (next_beams, splits)
}

// returns number of splits according to p1 rules
fn run_part1(input: String) -> usize {
    // find first beam
    let mut content_iter = input.split("\n");
    let line = content_iter.next().expect("Content has no lines?: ");
    let mut beams = HashSet::new();
    let mut beamidx: usize = 0; // not correct to 0 but don't feel like adding error stuff
    for (ind, ch) in line.chars().enumerate() {
        if ch == 'S' {
            beamidx = ind;
        }
    }
    beams.insert(Coord::new(beamidx));
    let mut splits = 0;

    for line in content_iter {
        let next_splits;
        (beams, next_splits) = get_next_beams(beams, get_splitters(line));
        splits += next_splits;
    }

    splits
}

fn main() {
    let file_path = "input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let p1 = run_part1(contents);

    println!("Part1: {p1}");
}

#[test]
fn test_part1() {
    let file_path = "testinput";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let res = run_part1(contents);
    assert_eq!(res, 21);
}
