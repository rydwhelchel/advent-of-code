use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32
}

impl Game {
    fn parse_game(line: String) -> Game {
        let mut split_line: Vec<&str> = line.split(|c| c == ':' || c == ','|| c == ';').collect();
        
        let game_index= split_line.remove(0)
            .split(' ')
            .collect::<Vec<&str>>()
            .remove(1)
            .parse::<u32>()
            .unwrap();
        let mut cur_red: u32 = 0;
        let mut cur_blue: u32 = 0;
        let mut cur_green: u32 = 0;

        for e in split_line {
            let elems: Vec<&str> = e.trim().split(' ').collect();
            let color = *elems.get(1).unwrap();
            let count: u32 = elems.first()
                .unwrap()
                .to_string()
                .parse::<u32>()
                .unwrap();
            match color {
                "red" => {
                    if count > cur_red {
                        cur_red = count;
                    }
                },
                "blue" => {
                    if count > cur_blue {
                        cur_blue = count;
                    }
                },
                "green" => {
                    if count > cur_green {
                        cur_green = count;
                    }
                },
                &_ => {
                    panic!("Unexpected color");
                }
            }
        }

        Game {
            id: game_index,
            red: cur_red,
            blue: cur_blue,
            green: cur_green
        }
    }

    fn get_min_power(&self) -> u32 {
        self.blue * self.green * self.red
    }
 
    //returns 0 if invalid game, otherwise returns index
    fn check_game(&self) -> u32 {
        let red_max: u32 = 12;
        let green_max: u32 = 13;
        let blue_max: u32 = 14;

        if self.blue > blue_max || self.green > green_max || self.red > red_max {
            return 0;
        }
        self.id
    }
}

fn part1(lines: impl Iterator<Item = String>) -> u32 {
    let mut sum: u32 = 0;

    for line in lines {
        let game: Game =  Game::parse_game(line);
        sum += game.check_game();
    }
    sum
}

fn part2(lines: impl Iterator<Item = String>) -> u32 {
    let mut sum: u32 = 0;

    for line in lines {
        let game: Game =  Game::parse_game(line);
        sum += game.get_min_power();
    }
    sum
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines().flatten();

    // println!("The total sum of valid games (part1) is: {}", part1(lines));
    println!("The total sum of powers (part2) is: {}", part2(lines));

    Ok(())
}

#[test]
fn test_parse_game() {
    let expected = Game {
        id: 1,
        red: 12,
        green: 13,
        blue: 14
    };
    let input_string = "Game 1: 14 blue, 12 red; 10 blue, 2 green, 1 red; 12 red, 11 blue, 13 green".to_string();
    let result = Game::parse_game(input_string);
}

#[test]
fn test_check_game() {
    let bad_game = Game {
        id: 1,
        red: 12,
        green: 14,
        blue: 14
    };
    let good_game = Game {
        id: 1,
        red: 12,
        green: 13,
        blue: 14
    };
    let bad_result = bad_game.check_game();
    let good_result = good_game.check_game();
    assert_eq!(0, bad_result);
    assert_eq!(1, good_result);
}