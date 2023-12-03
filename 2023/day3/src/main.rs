use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::u32;

use std::ops::Range;

//part1 - Two pass plan
//First pass makes a set of valid spots
//second pass iterates the input, adding numbers which are in valid spots (check all digits)
//  to the sum

fn part1(valid_spots: HashSet<String>, lines: Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    let mut is_valid = false;

    for (i, line) in lines.iter().enumerate() {
        let da_line = line.chars().collect::<Vec<char>>();
        let mut curr_num = "".to_string();
        for (j, e) in line.chars().enumerate() {
            if !e.is_ascii_digit() {
                continue;
            }
            if valid_spots.contains(&format!("{} {}", i, j)) {
                is_valid = true;
            }
            curr_num.push(e);
            if da_line.len() > j+1 && da_line.get(j+1).unwrap().is_ascii_digit() {
                continue;
            }
            if is_valid {
                // println!("Logging curr_num {}", curr_num);
                sum += curr_num.parse::<u32>().unwrap();
            }
            curr_num = "".to_string();
            is_valid = false;
        }
    }

    sum
}

///First pass
fn get_valid_spots(lines: Vec<String>) -> HashSet<String> {
    //There are no symbols in the first line, last line or in the first index/last index of any line
    // Thanks for protecting us from out of bounds errors :)
    let mut valid_spots: HashSet<String> = HashSet::new();
    let lines = lines.iter();

    for (index, line) in lines.enumerate() {
        let symbols = line.chars()
            .enumerate()
            .filter(|(_, elem)| !elem.is_ascii_digit() && !elem.eq(&'.'))
            .collect::<Vec<_>>();
        for symbol in symbols.iter() {
            valid_spots.insert(format!("{} {}", index - 1, symbol.0 - 1));
            valid_spots.insert(format!("{} {}", index - 1, symbol.0));
            valid_spots.insert(format!("{} {}", index - 1, symbol.0 + 1));
            valid_spots.insert(format!("{} {}", index, symbol.0 - 1));
            valid_spots.insert(format!("{} {}", index, symbol.0 + 1));
            valid_spots.insert(format!("{} {}", index + 1, symbol.0 - 1));
            valid_spots.insert(format!("{} {}", index + 1, symbol.0));
            valid_spots.insert(format!("{} {}", index + 1, symbol.0 + 1));
        }
    }

    valid_spots
} 

#[derive(PartialEq, Eq, Debug)]
pub struct Number {
    value: u32,
    range: Range<usize>,
    y: usize
}

impl Number {
    fn new(index: usize, line: &str, y: usize) -> Number {
        let line: Vec<char> = line.chars().collect();
        let mut i: usize = index;
        let mut leader: usize = 0;
        while line.get(i).unwrap().is_ascii_digit() {
            if i == 0 {
                leader = 0;
                break;
            }
            if !line.get(i-1).unwrap().is_ascii_digit() {
                leader = i;
                break;
            }
            i -= 1;
        }

        i = index;
        while i < line.len() && line.get(i).unwrap().is_ascii_digit() {
            i += 1;
        }
        let value: u32 = line[leader..i].iter().collect::<String>().parse::<u32>().unwrap();

        Number {value, range: leader..i, y}
    }
}

///Just calling this on gears, which are never in the border 
fn get_adjacent_num(lines: &[String], coords: (usize, usize)) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let top_row = lines.get(coords.0-1).unwrap();
    let mid_row = lines.get(coords.0).unwrap();
    let bot_row = lines.get(coords.0+1).unwrap();

    for x in [coords.1-1, coords.1, coords.1+1] {
        let top_el = *top_row.chars().collect::<Vec<_>>().get(x).unwrap();
        if top_el.is_ascii_digit() {
            let num = Number::new(x, top_row, coords.0-1);
            if !numbers.contains(&num) {
                numbers.push(num);
            }
        }

        let mid_el = *mid_row.chars().collect::<Vec<_>>().get(x).unwrap();
        if mid_el.is_ascii_digit() {
            let num = Number::new(x, mid_row, coords.0);
            if !numbers.contains(&num) {
                numbers.push(num);
            }
        }

        let bot_el = *bot_row.chars().collect::<Vec<_>>().get(x).unwrap();
        if  bot_el.is_ascii_digit() {
            let num = Number::new(x, bot_row, coords.0+1);
            if !numbers.contains(&num) {
                numbers.push(num);
            }
        }
    }

    numbers
}

fn get_gear_indexes(lines: &[String]) -> Vec<(usize, usize)> {
    let mut gear_coords: Vec<(usize, usize)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if line.contains('*') {
            for (j, elem) in line.chars().enumerate() {
                if elem.eq(&'*') {
                    gear_coords.push((i,j));
                }
            }
        }
    }

    gear_coords
}

fn part2(lines: Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    let gear_coords = get_gear_indexes(&lines);
    for gear in gear_coords {
        let adj_nums = get_adjacent_num(&lines, gear);
        if adj_nums.len() == 2 {
            //now its officially a gear
            sum += adj_nums.get(0).unwrap().value * adj_nums.get(1).unwrap().value;
        }
    }

    sum
}

fn main() -> std::io::Result<()>{
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten().collect::<Vec<String>>();

    let valid_spots = get_valid_spots(lines.clone());

    let part1_result = part1(valid_spots, lines.clone());

    println!("The part1 result is: {}", part1_result);

    let part2_result: u32 = part2(lines);

    println!("The part2 result is: {}", part2_result);

    Ok(())
}

#[test]
fn test_get_valid_spots() {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten().collect();

    let result_valid_spots = get_valid_spots(lines);
    let mut expected_valid_spots: HashSet<String> = HashSet::new();
    expected_valid_spots.insert("0 5".to_string());
    expected_valid_spots.insert("0 6".to_string());
    expected_valid_spots.insert("0 7".to_string());
    expected_valid_spots.insert("1 5".to_string());
    expected_valid_spots.insert("1 7".to_string());
    expected_valid_spots.insert("2 5".to_string());
    expected_valid_spots.insert("2 6".to_string());
    expected_valid_spots.insert("2 7".to_string());
    println!("{:?}", result_valid_spots);
    assert_eq!(expected_valid_spots, result_valid_spots);
}

#[test]
fn test_part1() {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();

    let valid_spots = get_valid_spots(lines.clone());
    let result_part_1 = part1(valid_spots, lines);
    let expected_sum: u32 = 168;

    assert_eq!(expected_sum, result_part_1);
}

#[test]
fn test_new_num() {
    let line = "12............45....67".to_string();
    let index1 = 1;
    let index2 = 14;
    let index3 = 21;
    let num1: Number = Number { value: 12, range: 0..2, y: 0 };
    let num2: Number = Number { value: 45, range: 14..16, y: 0 };
    let num3: Number = Number { value: 67, range: 20..22, y: 0 };

    let result1 = Number::new(index1, &line, 0);
    let result2 = Number::new(index2, &line, 0);
    let result3 = Number::new(index3, &line, 0);

    assert_eq!(result1, num1);
    assert_eq!(result2, num2);
    assert_eq!(result3, num3);
}

#[test]
fn test_part2() {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();

    let result_part_2 = part2(lines);
    let expected_sum: u32 = 5535;

    assert_eq!(expected_sum, result_part_2);
}
