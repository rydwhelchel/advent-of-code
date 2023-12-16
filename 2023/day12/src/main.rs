use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::Instant;

use cached::proc_macro::cached;

struct SpringRow {
    positions: String,
    chunks: Vec<usize>
}

impl SpringRow {
    fn new(line: &str) -> Self {
        let mut split = line.split(' ');
        let positions = split.next()
            .unwrap()
            .to_string();
        let chunks = split.next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        
        SpringRow {
            positions,
            chunks
        }
    }
}

// could make this faster by processing chunks backwards (more cache hits)
#[cached]
fn get_possible_positions(remaining_pos: String, remaining_chunks: Vec<usize>) -> usize {
    match remaining_pos.chars().next() {
        // '.' basically means skip processing this character
        Some('.') => get_possible_positions(remaining_pos[1..].to_string(), remaining_chunks.clone()),
        // '#' we *must* place a chunk here, returns 1 if you can place a chunk here, 0 if you cannot
        Some('#') => process_hash(remaining_pos, remaining_chunks),
        // '?' is treated as both '.' and '#'
        Some('?') => {
            get_possible_positions(remaining_pos.replacen('?', "#", 1), remaining_chunks.clone())
                + get_possible_positions(remaining_pos[1..].to_string(), remaining_chunks.clone())
        },
        // if the string is empty and the chunks are empty, return 1, else 0
        None => remaining_chunks.is_empty() as usize,
        //dont expect anything else
        Some(_) => panic!()
    }
}

// Solution is slower with this cached, I assume it is causing values in possible_solutions cache to get evicted
// #[cached]
fn process_hash(remaining_pos: String, remaining_chunks: Vec<usize>) -> usize {
    let first_chunk = match remaining_chunks.first() {
        Some(x) => *x,
        None => 0
    };
    // if there is still string left but no chunks to fill it
    if (remaining_chunks.is_empty() && remaining_pos.contains('#')) 
        // if remaining string length is less than current chunk range
        || (remaining_pos.len() < first_chunk) 
        // if there is a '.' in the the current chunk range
        || (remaining_pos[..first_chunk].contains('.'))
        // if there is a '#' right after current chunk range (and therefore too close to next chunk)
        || (remaining_pos.chars().collect::<Vec<char>>().get(first_chunk) == Some(&'#')) {
        return 0;
    }

    // if the current chunk can fill the remaining string
    if first_chunk == remaining_pos.len() && remaining_chunks.len() == 1 {
        return 1; 
    }

    // prevent out of bounds on next iteration
    if remaining_pos.len() < first_chunk + 1 {
        return 0;
    }

    // call the recursive function on the substring from first_chunk + 1 (space between) and on the substring of chunks after current chunk
    get_possible_positions(remaining_pos[first_chunk + 1..].to_string(), remaining_chunks[1..].to_vec())
}

fn part1(lines: &Vec<String>) -> usize {
    let mut sum: usize = 0;

    for line in lines {
        let row = SpringRow::new(line);
        let solution = get_possible_positions(row.positions, row.chunks);
        // println!("For row: {}, we got solution {}", line, solution);
        sum += solution
    }

    sum
}

fn part2(lines: &Vec<String>) -> usize {
    let mut sum: usize = 0;

    for line in lines {
        let mut i = 0;
        
        let row = SpringRow::new(line);

        let mut new_positions = row.positions.clone();
        let mut new_chunks = row.chunks.clone();
        while i < 4 {
            new_positions.push('?');
            new_positions.push_str(&row.positions);
            new_chunks.extend(&row.chunks);
            i += 1;
        }
        let solution = get_possible_positions(new_positions, new_chunks);
        // println!("For row: {}, we got solution {}", line, solution);
        sum += solution
    }

    sum
}

fn main() {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .flatten()
        .collect::<Vec<String>>();

    println!("The result of part1 is: {}", part1(&lines));
    let part2_elapsed = Instant::now();
    println!("The result of part2 is: {}", part2(&lines));
    // 1.18 seconds on normal run
    // 280ms on release build
    println!("{:?} elapsed on part2", part2_elapsed.elapsed());

}

#[allow(dead_code)]
fn get_test_data() -> Vec<String> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .flatten()
        .collect::<Vec<String>>()
}

#[test]
fn test_calc_total_positions() {
    let lines = get_test_data();
    let line1 = SpringRow::new(lines.get(0).unwrap());
    assert_eq!(1, get_possible_positions(line1.positions, line1.chunks));
    let line2 = SpringRow::new(lines.get(1).unwrap());
    assert_eq!(4, get_possible_positions(line2.positions, line2.chunks));
}

#[test]
fn test_part1() {
    let lines = get_test_data();
    assert_eq!(21, part1(&lines));
}

#[test]
fn test_part2() {
    let lines = get_test_data();
    assert_eq!(525152, part2(&lines));
}
