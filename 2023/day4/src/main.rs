use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

fn get_winning_set(line: &str) -> HashSet<&str> {
    line.split('|')
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<HashSet<&str>>()
}

fn get_numbers_you_have(line: &str) -> Vec<&str> {
    line.split('|')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
}

fn calc_score(winning_set: HashSet<&str>, numbers_you_have: Vec<&str>) -> u32 {
    let mut sum: u32= 0;
    for num in numbers_you_have.iter() {
        if winning_set.contains(num) {
            sum = if sum == 0 { 1 } else { sum * 2 };
        }
    }

    sum
}

fn calc_count(winning_set: HashSet<&str>, numbers_you_have: Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    for num in numbers_you_have.iter() {
        if winning_set.contains(num) {
            sum += 1;
        }
    }

    sum
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in lines {
        sum += calc_score(get_winning_set(line), get_numbers_you_have(line));
    }
    sum
}

#[derive(PartialEq, Debug)]
struct Card {
    line: String,
    instances: u32
}

impl Card {
    fn new(line: String) -> Card {
        Card {
            line,
            instances: 1
        }
    }

    fn increment(&mut self, amount: u32) {
        self.instances += amount;
    } 
}

fn initialize_cards(lines: &[String]) -> Vec<Card> {
    lines.iter()
        .map(|line| Card::new(line.clone()))
        .collect::<Vec<Card>>()
}

fn update_cards(cards: &mut Vec<Card>) {
    let mut index: usize = 0;
    while index < cards.len() {
        let mut count: u32;
        let amount: u32;
        {
            let card = cards.get_mut(index).unwrap();
            count = calc_count(get_winning_set(&card.line), get_numbers_you_have(&card.line));
            amount = card.instances;
        }

        let mut i: usize = 1;
        while count > 0 && index + i < cards.len() {
            cards.get_mut(index + i).unwrap().increment(amount);
            count -= 1;
            i += 1;
        }
        index += 1;
    }
}

///you wouldn't cheat the casino out of money, would you?
fn count_cards(cards: Vec<Card>) -> u32 {
    cards.iter()
        .map(|card| card.instances)
        .sum()
}

fn part2(lines: Vec<String>) -> u32 {
    let mut cards = initialize_cards(&lines);
    update_cards(&mut cards);

    count_cards(cards)
}

fn main() -> std::io::Result<()> {
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().flatten().collect();

    println!("The result of part1 is: {}", part1(&lines));
    println!("The result of part2 is: {}", part2(lines));

    Ok(())
}

#[test]
fn test_winning_set() {
    let test_line = "Card   4: 19 55 20 50 57 36 61 35 59 75 | 75 11 94 28 49 59 38 32 55 35 61 68 29 36 97 57 50 20 70 52 13 53 87 19 42";
    let mut expected_winning_numbers = HashSet::new();
    expected_winning_numbers.insert("19");
    expected_winning_numbers.insert("55");
    expected_winning_numbers.insert("20");
    expected_winning_numbers.insert("50");
    expected_winning_numbers.insert("57");
    expected_winning_numbers.insert("36");
    expected_winning_numbers.insert("61");
    expected_winning_numbers.insert("35");
    expected_winning_numbers.insert("59");
    expected_winning_numbers.insert("75");

    assert_eq!(expected_winning_numbers, get_winning_set(test_line));
}

#[test]
fn test_numbers_you_have() {
    let test_line = "Card   4: 19 55 20 50 57 36 61 35 59 75 | 75 11 94 28 49";
    let expected_numbers_you_have: Vec<&str> = vec!["75","11","94","28","49"];

    assert_eq!(expected_numbers_you_have, get_numbers_you_have(test_line));
}

#[test]
fn test_calc_score() {
    let mut winning_nums: HashSet<&str> = HashSet::new();
    winning_nums.insert("41");
    winning_nums.insert("48");
    winning_nums.insert("83");
    winning_nums.insert("86");
    winning_nums.insert("17");

    let numbers_you_have = vec!["83", "86", "6", "31", "17", "9", "48", "53"];

    let expected:u32 = 8;
    assert_eq!(expected, calc_score(winning_nums, numbers_you_have));
}

#[test]
fn test_initialize_cards() {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().flatten().collect();
    let cards: Vec<Card> = initialize_cards(&lines);
    let expected_cards: Vec<Card> = vec![
        Card {
            line: "Card   1: 13  4 61 82 80 41 31 53 50  2 | 38 89 26 79 94 50  2 74 31 92 80 41 13 97 61 82 68 45 64 39  4 53 90 84 54".to_string(),
            instances: 1
        },
        Card {
            line: "Card   2: 25 44 86 77 98 91 55 39 63 12 | 84 62 55 28 99 26 19 18 13 57 97 63 20 65 24 31 72 41 77 27 50 30 38  3 88".to_string(),
            instances: 1
        },
        Card {
            line: "Card   3: 96 46 60 19 82 25 41 29 38 94 | 43 82 86 74 16 15 92 46 32  3 17 30 42 98 60 12 96 38 19 35  6 29 72 25 62".to_string(),
            instances: 1
        },
    ];
    
    assert_eq!(expected_cards, cards);
}

#[test]
fn test_update_cards() {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().flatten().collect();
    let mut cards: Vec<Card> = initialize_cards(&lines);

    println!("{:?}", cards);
    update_cards(&mut cards);
    println!("{:?}", cards);
}