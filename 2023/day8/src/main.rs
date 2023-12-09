#![allow(unused, unused_imports, unused_variables, clippy::explicit_counter_loop)]

use std::time::Instant;
use std::{collections::HashMap, fs::File};
use std::io::{prelude::*, BufReader};

struct Nodes {
    map: HashMap<String, (String, String)>
}

impl Nodes {
    fn new(lines: &[String]) -> Self {
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        for line in lines {
            let (key, value) = parse_line(line);
            map.insert(key, value);
        }

        Nodes { map }
    }
}

fn parse_line(line: &str) -> (String, (String, String)) {
    let x = *line.split('=').map(|x| x.trim()).collect::<Vec<&str>>().first().unwrap();
    let y = line.split('=')
        .map(|x| x.trim())
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.trim())
        .map(|x| x.replace('(', ""))
        .map(|x| x.replace(')', ""))
        .collect::<Vec<String>>();

    (x.to_string(), (y.get(0).unwrap().to_string(), y.get(1).unwrap().to_string()))
}

fn part1(lines: &[String]) -> u64 {
    let instructions = lines.get(0).unwrap().chars().collect::<Vec<char>>();
    let nodes = Nodes::new(&lines[2..]);

    let map = nodes.map;

    let mut curr_node = "AAA".to_string();
    let destination_node = "ZZZ".to_string();
    let mut steps = 0;

    loop {
        for instruction in &instructions {
            let curr_vals = map.get(&curr_node).unwrap();
            let next_val = match instruction {
                'L' => Ok(&curr_vals.0),
                'R' => Ok(&curr_vals.1),
                _ => Err("Invalid instruction")
            }.unwrap();
            steps += 1;
            if next_val.eq("ZZZ") {
                return steps;
            }
            curr_node = next_val.to_string();
        }
    }
    1
}

fn get_last_char(node: &str) -> char {
    *node.chars()
        .collect::<Vec<char>>()
        .last().unwrap()
}

fn get_starting_pos(map: &HashMap<String, (String, String)>) -> Vec<String> {
    map.keys()
        .filter(|x| get_last_char(x) == 'A')
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn check_done(curr_pos: &Vec<String>) -> usize {
    curr_pos.len() - curr_pos.iter().filter(|x| get_last_char(x) == 'Z').collect::<Vec<_>>().len()
}

//attempted this, it runs.... a long time
fn part2_brute_force(lines: &[String]) -> u64 {
    let instructions = lines.get(0).unwrap().chars().collect::<Vec<char>>();
    let nodes = Nodes::new(&lines[2..]);

    let map = nodes.map;
    let mut curr_pos = get_starting_pos(&map);
    println!("Starting Positions: {:?}", curr_pos);
    let mut steps = 0;
    let mut curr_best: usize = usize::MAX;
    let start = Instant::now();
    loop {
        for instruction in &instructions {
            let mut index = 0;
            // println!("Curr_pos prior to being modified: {:?}", curr_pos);
            while index < curr_pos.len() {
                let curr_vals = map.get(&curr_pos.remove(index)).unwrap();
                let next_val = match instruction {
                    'L' => Ok(&curr_vals.0),
                    'R' => Ok(&curr_vals.1),
                    _ => Err("Invalid instruction")
                }.unwrap();
                curr_pos.insert(index, next_val.to_string());
                
                index += 1;
            }
            // println!("Curr_pos after being modified: {:?}", curr_pos);
            steps += 1;
            let done = check_done(&curr_pos);
            curr_best = if done < curr_best { done } else { curr_best };
            if curr_best == 0 {
                return steps;
            }
            if steps % 10000000 == 0 {
                println!("Current steps {}, Current best is: {}", steps, curr_best);
                println!("We are about {}% done.", steps/25000000000000*100);
                println!("{:?} elapsed.", start.elapsed());
            }
        }
    }


    1
}

//trying different approach
//for each starting node, find each cycle from starting node to an ending node (any node ending with Z)
//find the least common multiple of each cycle for each starting ndoe
//  this works because each starting node should be part of its own cycle, the cycles should not intersect (or else they would be the same cycle)
//          pretty sure this is true
//  potentially they could be on the same cycle, but on different offsets, I believe that this will still work in that case but I can't put it into words
fn part2(lines: &[String]) -> u64 {
    let instructions = lines.get(0).unwrap().chars().collect::<Vec<char>>();
    let nodes = Nodes::new(&lines[2..]);

    let map = nodes.map;
    let curr_pos = get_starting_pos(&map);
    println!("Starting Positions: {:?}", curr_pos);
    let mut steps: Vec<u64> = Vec::new();
    for pos in curr_pos {
        steps.push(get_steps_in_cycle(pos, &instructions, &map));
    }

    steps.iter().fold(1, |x, y| least_common_multiple(x, *y))
}

fn least_common_multiple(mut step1: u64, mut step2: u64) -> u64 {
    (step1 * step2) / greatest_common_denominator(step1, step2)
}

fn greatest_common_denominator(mut step1: u64, mut step2: u64) -> u64 {
    //long division method
    while step2 > 0 {
        let tmp = step1;
        step1 = step2;
        step2 = tmp % step1;
    }
    step1
}

fn get_steps_in_cycle(node: String, instructions: &Vec<char>, map: &HashMap<String, (String, String)>) -> u64 {
    let mut steps = 0;
    let mut curr_node = &node;
    loop {
        for instruction in instructions {
            let curr_vals = map.get(curr_node).unwrap();
            let next_val = match instruction {
                'L' => Ok(&curr_vals.0),
                'R' => Ok(&curr_vals.1),
                _ => Err("Invalid instruction")
            }.unwrap();
            steps += 1;
            if is_last(next_val) {
                return steps;
            }
            curr_node = next_val;
        }
    }
    1
}

fn is_last(node: &str) -> bool {
    node.ends_with('Z')
}

fn main() -> std::io::Result<()> {
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten().collect::<Vec<String>>();

    let start1 = Instant::now();
    let part1 = part1(&lines);
    let duration1 = start1.elapsed();

    let start2 = Instant::now();
    let part2 = part2(&lines);
    let duration2 = start2.elapsed();

    let start3 = Instant::now();
    let part2_brute_force = part2_brute_force(&lines);
    let duration3 = start3.elapsed();

    println!("The result of part1 is: {}", part1);
    println!("The time elapsed for part 1 is: {:?}", duration1);
    println!("The result of part2 is: {}", part2);
    println!("The time elapsed for part 1 is: {:?}", duration2);
    // println!("The result of part2 is: {}", part2_brute_force);   // est. over 100 hours to finish
    // println!("The time elapsed for part 1 is: {:?}", duration3);
    Ok(())
}

fn get_test_data() -> Vec<String> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect::<Vec<String>>()
}

fn get_test_data_2() -> Vec<String> {
    let path = "example2.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect::<Vec<String>>()
}

#[test]
fn test_parse_line() {
    let lines = get_test_data();
    let result = parse_line(lines.get(2).unwrap());

    let expected = ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string()));

    assert_eq!(expected, result);
}

#[test]
fn test_part1() {
    let lines = get_test_data();
    let result = part1(&lines);

    let expected = 6;
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let lines = get_test_data_2();
    let result = part2(&lines);

    let expected = 6;
    assert_eq!(expected, result);
}