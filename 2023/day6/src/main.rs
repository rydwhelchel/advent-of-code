#![allow(unused, unused_variables, unused_imports)]

use core::time;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::result;
use itertools::Itertools;

// Can use isodd to speed up search process --- doesn't really seem necessary here though
trait IsOdd {
    fn is_odd(&self) -> bool;
}

impl IsOdd for u64 {
    fn is_odd(&self) -> bool {
        self % 2 == 1
    }
}

fn parse_lines(lines: &[String]) -> Vec<(u64, u64)> {
    let mut time_dists: Vec<(u64, u64)> = Vec::new();
    let times: Vec<u64> = lines.get(0).unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let dists: Vec<u64> = lines.get(1).unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for (index, time) in times.iter().enumerate() {
        time_dists.push((*times.get(index).unwrap(), *dists.get(index).unwrap()))
    }

    time_dists
}

fn parse_lines_2(lines: &[String]) -> (u64, u64) {
    let time: u64 = lines.get(0).unwrap()
        .split(':')
        .last().unwrap()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| !s.is_empty())
        .copied()
        .join("")
        .parse::<u64>().unwrap();
    let dist = lines.get(1).unwrap()
        .split(':')
        .last().unwrap()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| !s.is_empty())
        .copied()
        .join("");
    println!("{:?}", dist);
    let dist = dist.parse::<u64>().unwrap();
    (time, dist)
}

fn get_faster_times(time_dist: (u64, u64)) -> u64 {
    let mut ways_to_win: u64 = 0;
    let mut index = 0;
    let time = time_dist.0;
    let distance = time_dist.1;
    while index < time {
        if (time - index) * index > distance {
            ways_to_win += 1;
        }
        index += 1;
    }

    ways_to_win
}

fn part1(lines: &[String]) -> u64 {
    let time_dists = parse_lines(lines);
    let mut result: u64 = 1;
    for time_dist in time_dists {
        result *= get_faster_times(time_dist);
    }
    result
}

fn part2(lines: &[String]) -> u64 {
    let time_dist = parse_lines_2(lines);
    get_faster_times(time_dist)
}

fn main() {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten().collect::<Vec<String>>();

    println!("The result of part1 is: {}", part1(&lines));
    println!("The result of part2 is: {}", part2(&lines));
}

fn get_test_data() -> Vec<String> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect::<Vec<String>>()
}

#[test]
fn test_parse_lines() {
    let lines = get_test_data();
    let result_time_dists = parse_lines(&lines);
    let expected_time_dists: Vec<(u64, u64)> = vec![(7, 9), (15, 40), (30, 200)];

    assert_eq!(expected_time_dists, result_time_dists)
}

#[test]
fn test_parse_lines_2() {
    let lines = get_test_data();
    let result_time_dist = parse_lines_2(&lines);
    let expected_time_dist: (u64, u64) = (71530, 940200);

    assert_eq!(expected_time_dist, result_time_dist)
}

#[test]
fn test_get_faster_times() {
    let test_data = (7, 9);
    let result_faster_times = get_faster_times(test_data);
    let expected_faster_times = 4;

    assert_eq!(expected_faster_times, result_faster_times);
}