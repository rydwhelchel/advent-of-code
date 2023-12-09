#![allow(unused, unused_imports, unused_variables, clippy::map_entry, clippy::vec_init_then_push, clippy::comparison_chain)]

use std::{collections::HashMap, fs::File, io::BufReader};
use std::io::prelude::*;
use std::cmp::Ordering;

// I want to use the word type to refer to this but its a keyword and I hate it 😡
#[derive(Clone, PartialEq, Debug, PartialOrd, Eq)]
enum Value {
    FiveOak,
    FourOak,
    FullHouse,
    ThreeOak,
    TwoPair,
    OnePair,
    HighCard
}

impl Value {
    fn power(&self) -> u8 {
        //0 indexed
        match self {
            Self::FiveOak => 6,
            Self::FourOak => 5,
            Self::FullHouse => 4,
            Self::ThreeOak => 3,
            Self::TwoPair => 2,
            Self::OnePair => 1,
            Self::HighCard => 0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Hand {
    hand: String,
    value: Value,
    bid: u16
}

impl Hand {
    fn new(line: &str) -> Self {
        let mut split = line.split(' ');
        let hand = split.next().unwrap().to_string();
        let bid = split.next().unwrap().parse::<u16>().unwrap();
        //TODO: rename to get_hand_value to get part 1
        let value = get_hand_value_2(&hand);

        Hand {
            hand,
            value,
            bid
        }
    }
    
    fn get_value(&self) -> usize {
        self.value.power().into()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        let our: Vec<char> = self.hand.chars().collect::<Vec<char>>();
        let their: Vec<char> = other.hand.chars().collect::<Vec<char>>();
        for (index, card) in our.iter().enumerate() {
            //TODO: Rename get_card_power_2 to get_card_power to get part 1
            if get_card_power_2(*card) < get_card_power_2(*their.get(index).unwrap()) {
                return Some(Ordering::Less);
            } else if get_card_power_2(*card) > get_card_power_2(*their.get(index).unwrap()) {
                return Some(Ordering::Greater);
            }
        }
        Some(Ordering::Equal)
    }
}


fn get_hand_value(hand: &str) -> Value {
    let card_count = parse_hand(hand);
    let vals = card_count.values()
        .copied()
        .collect::<Vec<u8>>();
    let max = vals.max_card();
    let min = vals.min_card();
    match max {
        5 => Value::FiveOak,
        4 => Value::FourOak,
        3 => match min {
            2 => Value::FullHouse,
            _ => Value::ThreeOak
        },
        2 => match vals.len() {
            3 => Value::TwoPair,
            _ => Value::OnePair
        }
        _ => Value::HighCard
    }
}

fn get_hand_value_2(hand: &str) -> Value {
    let mut card_count = parse_hand(hand);

    let mut vals: Vec<u8>;
    let mut max: u8;
    let mut min: u8;

    if card_count.contains_key(&'J') {
        let joker_count = card_count.remove(&'J').unwrap();
        vals = card_count.values()
            .copied()
            .collect::<Vec<u8>>();
        max = vals.max_card()+joker_count;
        min = vals.min_card();
    } else {
        vals = card_count.values()
            .copied()
            .collect::<Vec<u8>>();
        max = vals.max_card();
        min = vals.min_card();
    }

    match max {
        5 => Value::FiveOak,
        4 => Value::FourOak,
        3 => match min {
            2 => Value::FullHouse,
            _ => Value::ThreeOak
        },
        2 => match vals.len() {
            3 => Value::TwoPair,
            _ => Value::OnePair
        }
        _ => Value::HighCard
    }
}

fn parse_hand(hand: &str) -> HashMap<char, u8> {
    let mut cards = hand.chars();
    let mut checked: HashMap<char, u8> = HashMap::new();

    for card in cards {
        if !checked.contains_key(&card) {
            checked.insert(card, 1);
            continue;
        }
        checked.increment(card);
    }

    checked
}

trait Increment {
    fn increment(&mut self, card: char);
}

impl Increment for HashMap<char, u8> {
    fn increment(&mut self, card: char) {
        let curr = self.get(&card).unwrap();
        self.insert(card, curr+1);
    }
}

trait MinMax {
    fn min_card(&self) -> u8;
    fn max_card(&self) -> u8;
}

impl MinMax for Vec<u8> {
    fn min_card(&self) -> u8 {
        let mut curr_min = u8::MAX;
        for num in self {
            if num < &curr_min {
                curr_min = *num;
            }
        }
        curr_min
    }
    fn max_card(&self) -> u8 {
        let mut curr_max = u8::MIN;
        for num in self {
            if num > &curr_max {
                curr_max = *num;
            }
        }
        curr_max
    }
}

fn get_card_power(card: char) -> u8 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0
    }
}

fn get_card_power_2(card: char) -> u8 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0
    }
}

fn runner(lines: &[String]) -> u64 {
    let mut ginormous: u64 = 0;
    let mut hands: Vec<Hand> = Vec::new();
    
    for line in lines {
        hands.push(Hand::new(line));
    }

    let mut ranked_hands: [Vec<Hand>; 7] = [
        Vec::new(),
        Vec::new(),
        Vec::new(), 
        Vec::new(), 
        Vec::new(), 
        Vec::new(),
        Vec::new()
    ];

    for hand in hands {
        ranked_hands[hand.get_value()].push(hand);
    }

    let mut boogaloo: Vec<Vec<Hand>> = Vec::new();

    for mut hands in ranked_hands {
        hands.sort_by(|hand1, hand2| hand1.partial_cmp(hand2).unwrap());
        boogaloo.push(hands);
    }

    let ranked_hands = boogaloo;

    let mut rank: u64 = 1;
    for hands in ranked_hands{
        for hand in hands {
            let bid: u64 = hand.bid.into();
            ginormous += rank * bid;
            rank += 1;
        }
    }

    ginormous
}

fn main() -> std::io::Result<()> {
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file); 
    let lines = reader.lines().flatten().collect::<Vec<String>>();

    // Need to make code changes above in TODO sections to get part 1
    // println!("The result for part1 is: {}", runner(&lines));
    println!("The result for part2 is: {}", runner(&lines));

    Ok(())
}

fn get_test_data() -> Vec<String> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file); 
    reader.lines().flatten().collect::<Vec<String>>()
}

#[test]
fn test_get_hand_value_2() {
    let hand = "JJ225";
    assert_eq!(get_hand_value_2(hand), Value::FourOak);
    let hand = "22334";
    assert_eq!(get_hand_value_2(hand), Value::TwoPair);
    let hand = "JJ223";
    assert_eq!(get_hand_value_2(hand), Value::FourOak);
    let hand = "JJ222";
    assert_eq!(get_hand_value_2(hand), Value::FiveOak);
    let hand = "J3334";
    assert_eq!(get_hand_value_2(hand), Value::FourOak);
    let hand = "2J543";
    assert_eq!(get_hand_value_2(hand), Value::OnePair);
    let hand = "JJJJJ";
    assert_eq!(get_hand_value_2(hand), Value::FiveOak);
    let hand = "J1J23";
    assert_eq!(get_hand_value_2(hand), Value::ThreeOak);
    let hand = "J1221";
    assert_eq!(get_hand_value_2(hand), Value::FullHouse);
}

#[test]
fn test_runner() {
    let lines = get_test_data();
    assert_eq!(5905, runner(&lines))
}
