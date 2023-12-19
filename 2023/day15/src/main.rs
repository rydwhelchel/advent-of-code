// part 2 appears to be a hashmap of hashmaps
// outer hashmap is the 0..256 value of the label (string prior to the operator (-,=))
// inner hashmap is just the label *** NOTE INNER HASHMAP IS ORDERED, USING A HASHMAP IS PROBABLY
// INCORRECT***, probably better to just use a vec of tuples
// value of inner hashmap is the lens #
// if you run into a key that already exists in inner hashmap, update
#![allow(unused, unused_imports, dead_code)]
use std::{fs::File, io::BufReader, io::prelude::*, usize};

fn get_hashed_val(hashed: &String) -> usize {
    let vect = hashed.chars().map(|a| a as usize);
    let mut sum = 0;
    for asc in vect {
        sum += asc;
        sum *= 17;
        sum %= 256;
    }
    sum
}

#[derive(Clone)]
enum Operator {
    Minus, Equals
}

#[derive(Clone)]
struct Lens {
    operator: Operator,
    label: String,
    value: u8
}

impl Lens {
    fn new(input: &String) -> Self {
        if input.contains('-') {
            let operator = Operator::Minus;
            let label = input.split('-')
                .next()
                .unwrap()
                .to_string();
            let value = 0;
            Lens {
                operator,
                label,
                value
            }
        } else {
            let operator = Operator::Equals;
            let label = input.split('=')
                .next()
                .unwrap()
                .to_string();
            let value = input.split('=')
                .last()
                .unwrap()
                .parse::<u8>()
                .unwrap();
            Lens {
                operator,
                label,
                value
            }
        }
    }
    
    fn hash(&self) -> (usize, (String, u8)) {
        let hashed = get_hashed_val(&self.label);
        (hashed, (self.label.clone(), self.value))
    }
}

fn parse_input(input: Vec<String>) -> Vec<Lens> {
    input.iter()
        .map(|x| Lens::new(x))
        .collect::<Vec<Lens>>()
}

fn part1(hashed: Vec<String>) -> usize {
    let mut sum = 0;
    for hash in hashed {
        sum += get_hashed_val(&hash);
    }
    sum
}

fn part2(input: Vec<String>) -> usize {
    let mut hash_table: [Vec<(String, u8)>; 256] = vec![Vec::new(); 256]
        .try_into()
        .expect("did not work");
    let lenses = parse_input(input);
    for lens in lenses {
        let hashed = lens.hash();
        let mut collision_map: &mut Vec<(String, u8)> = &mut hash_table[hashed.0];
        match lens.operator {
            Operator::Minus => {
                let mut i = 0;
                while i < collision_map.len() {
                    if collision_map.get(i).unwrap().0 == lens.label {
                        collision_map.remove(i);
                    }
                    i += 1;
                }
            },
            Operator::Equals => {
                let mut updated = false;
                let mut i = 0;
                while i < collision_map.len() {
                    if collision_map.get(i).unwrap().0 == lens.label {
                        collision_map.remove(i);
                        collision_map.insert(i, hashed.1.clone());
                        updated = true;
                    }
                    i += 1;
                }
                if !updated {
                    collision_map.push(hashed.1.clone());
                }
            }
        }
    }
    // now need count score of hash_table
    let mut sum = 0;
    let mut i = 0;
    while i < hash_table.len() {
        let mut j = 0;
        let curr = hash_table[i].clone();
        while j < curr.len() {
            sum += (i + 1) * (j + 1) * (curr.get(j).unwrap().1 as usize); 
            j += 1;
        }
        i += 1;
    }
    sum
}

fn main() {
    let hashed = get_input_data();
    println!("The result of part 1 is {}", part1(hashed.clone()));
    println!("The result of part 2 is {}", part2(hashed));
}

fn get_input_data() -> Vec<String> {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let hashed = reader.lines()
        .flatten()
        .collect::<String>()
        .split(',')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    hashed
}

fn get_test_data() -> Vec<String> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let hashed = reader.lines()
        .flatten()
        .collect::<String>()
        .split(',')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    hashed
}

#[test]
fn test_hash() {
    let input = "rn".to_string();
    let hashed = get_hashed_val(&input);
    assert_eq!(0, hashed)
}

#[test]
fn test_part2() {
    let input = get_test_data();
    let result = part2(input);
    assert_eq!(145, result);
}

