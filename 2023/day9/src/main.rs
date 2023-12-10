#![allow(unused, unused_variables, unused_imports)]

use std::{fs::File, io::BufReader, io::prelude::*};

struct History {
    histories: Vec<Vec<i64>>
}

impl History {
    fn new(line: &str) -> Self {
        let mut nums = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        //ultimate goal is to add the last value in each diff vec on the way up
        let mut histories: Vec<Vec<i64>> = vec![nums.clone()];

        while !all_zeroes(&nums) {
            //generate differences between each element in nums
            nums = get_diffs(&nums);
            //push to histories
            histories.push(nums.clone());
        }

        History { histories }
    }

    fn get_next_val(&self) -> i64 {
        let length = self.histories.len();
        let mut index = 0;
        let mut sum = 0;
        while index < length {
            sum += self.histories.get(index).unwrap().last().unwrap();
            index += 1;
        }
        sum
    }

    fn get_prev_val(&self) -> i64 {
        let mut index = self.histories.len();
        let mut prev = 0;
        while index > 0 {
            index -= 1;
            prev = self.histories.get(index).unwrap().first().unwrap() - prev;
        }
        prev
    }
}

fn all_zeroes(nums: &[i64]) -> bool {
    nums.iter().sum::<i64>() == 0
}

fn get_diffs(nums: &[i64]) -> Vec<i64> {
    let mut index = 1;
    let mut diffs = Vec::new();
    while index < nums.len() {
        diffs.push(nums[index] - nums[index - 1]);
        index += 1;
    }
    diffs
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    for line in lines {
        let history = History::new(line);
        sum += history.get_next_val();
    }
    sum
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    for line in lines {
        let history = History::new(line);
        sum += history.get_prev_val();
    }
    sum
}

fn main() -> std::io::Result<()> {
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten().collect::<Vec<String>>();

    println!("The result of part1 is: {}", part1(&lines));
    println!("The result of part1 is: {}", part2(&lines));


    Ok(())
}

fn get_test_data() -> Vec<String> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect::<Vec<String>>()
}

#[test]
fn test_new_hist() {
    let lines = get_test_data();
    let expected_history = History {
        histories: vec![
            vec![10, 13, 16, 21, 30, 45],
            vec![3,  3,  5,  9, 15],
            vec![0,  2,  4,  6],
            vec![2,  2,  2],
            vec![0, 0]
        ]
    };
    let result_history = History::new(lines.get(2).unwrap());

    assert_eq!(expected_history.histories, result_history.histories)
}

#[test]
fn test_all_zeroes() {
    let nums: &[i64; 3] = &[0, 0, 0];
    let nums2: &[i64; 3] = &[1,0,3];

    assert!(all_zeroes(nums));
    assert!(!all_zeroes(nums2));
}

#[test]
fn test_get_next_val() {
    let lines = get_test_data();
    let result_next_val = History::new(lines.get(2).unwrap()).get_next_val();
    assert_eq!(68, result_next_val);
}

#[test]
fn test_get_prev_val() {
    let lines = get_test_data();
    let result_next_val = History::new(lines.get(2).unwrap()).get_prev_val();
    assert_eq!(5, result_next_val);
}
