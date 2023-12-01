#![allow(unused_imports,unused_variables,dead_code)]

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()>{
    let f = File::open("input1.txt")?;
    let reader = BufReader::new(f);
    let mut sum: u32 = 0;

    for line in reader.lines().flatten() {
        let line: Vec<char> = line.chars().collect();
        let one:&[char] = ['o','n','e'].as_slice();
        let two = ['t','w','o'].as_slice();
        let three = ['t','h','r','e','e'].as_slice();
        let four = ['f','o','u','r'].as_slice();
        let five = ['f','i','v','e'].as_slice();
        let six = ['s','i','x'].as_slice();
        let seven = ['s','e','v','e','n'].as_slice();
        let eight = ['e','i','g','h','t'].as_slice();
        let nine = ['n','i','n','e'].as_slice();

        let mut first_num: char = 'a';
        let mut second_num: char = 'a';
        let length = line.len();
        for (i, e) in line.clone().into_iter().enumerate() {
            match e {
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    if !first_num.is_numeric() {
                        first_num = e;
                    }
                    second_num = e;
                },
                'o' => {
                    if i+2 < length {
                        let pot = &line[i..=i+2];
                        if one.eq(pot) {
                            if !first_num.is_numeric() {
                                first_num = '1';
                            }
                            second_num = '1';
                        }
                    }
                },
                't' => {
                    if i+2 < length {
                        let pot = &line[i..=i+2];
                        if two.eq(pot) {
                            if !first_num.is_numeric() {
                                first_num = '2';
                            }
                            second_num = '2';
                        }
                    }
                    if i+4 < length {
                        let pot = &line[i..=i+4];
                        if three.eq(pot) {
                            if !first_num.is_numeric() {
                                first_num = '3';
                            }
                            second_num = '3';
                        }
                    }
                },
                'f' => {
                    if i+3 < length {
                        let pot = &line[i..=i+3];
                        if four.eq(pot) {
                            if !first_num.is_numeric() {
                                first_num = '4';
                            }
                            second_num = '4';
                        }
                    }
                    if i+3 < length {
                        let pot = &line[i..=i+3];
                        if five.eq(pot) {
                            if !first_num.is_numeric() {
                                first_num = '5';
                            }
                            second_num = '5';
                        }
                    }
                },
                's' => {
                    if i+2 < length {
                        let pot = &line[i..=i+2];
                        if six.eq(pot) {
                            if !first_num.is_numeric() {
                                first_num = '6';
                            }
                            second_num = '6';
                        }
                    }
                    if i+4 < length {
                        let pot = &line[i..=i+4];
                        if seven.eq(pot) {
                            if !first_num.is_numeric() {
                                first_num = '7';
                            }
                            second_num = '7';
                        }
                    }
                },
                'e' => {
                    if i+4 < length {
                        let pot = &line[i..=i+4];
                        if eight.eq(pot) {
                            if !first_num.is_numeric() {
                                first_num = '8';
                            }
                            second_num = '8';
                        }
                    }
                },
                'n' => {
                    if i+3 < length {
                        let pot = &line[i..=i+3];
                        if nine.eq(pot) {
                            if !first_num.is_numeric() {
                                first_num = '9';
                            }
                            second_num = '9';
                        }
                    }
                },
                _ => {}
            }
        }
        sum += vec![first_num,second_num].into_iter().collect::<String>().parse::<u32>().unwrap();
    }
    println!("{}",sum);
    Ok(())
}
