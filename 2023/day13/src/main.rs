// Sum the number of columns left of the vertical mirrors
//  and sum the number of rows above the horizontal mirrors times 100
#![allow(unused, unused_variables)]

use std::str::Lines;
use std::{fs::File, io::BufReader};
use std::io::prelude::*;

struct Layout {
    layout: Vec<Vec<char>>
}

impl Layout {
    fn new(lines: &[String]) -> Self {
        let mut layout: Vec<Vec<char>> = Vec::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            layout.push(line.chars().collect()); 
        }
        Layout {
            layout
        }
    }

    fn sum_inflections(&self) -> usize {
        let horizontal = match self.find_horizontal_inflection() {
            Some(x) => x*100,
            None => 0
        };
        let vertical = self.find_vertical_inflection().unwrap_or(0);
        if horizontal > vertical {
            return horizontal
        }
        vertical
    }

    //returns the index of the vertical inflection, if it exists
    fn find_vertical_inflection(&self) -> Option<usize> {
        let mut column: usize = 1;
        let length = self.layout.get(0).unwrap().len();
        while column < length {
            let mut matching_cols = 0;
            while (column - matching_cols > 0)
                && (column + matching_cols < length) {
                let left_col = column - matching_cols - 1;
                let right_col = column + matching_cols;
                if !self.compare_column(left_col, right_col) {
                    matching_cols = 0;
                    break;
                }
                matching_cols += 1;
            }
            if matching_cols > 0 {
                return Some(column);
            }
            column += 1;
        }
        None
    }

    //returns the index of the vertical inflection, if it exists
    fn find_horizontal_inflection(&self) -> Option<usize> {
        let mut row: usize = 1;
        let length = self.layout.len();
        while row < length {
            let mut matching_rows = 0;
            while (row - matching_rows > 0)
                && (row + matching_rows < length) {
                let top_row = row - matching_rows - 1;
                let bottom_row = row + matching_rows;
                if !self.compare_row(top_row, bottom_row) {
                    matching_rows = 0;
                    break;
                }
                matching_rows += 1;
            }
            if matching_rows > 0 {
                return Some(row);
            }
            row += 1;
        }
        None
    }

    fn compare_column(&self, index1: usize, index2: usize) -> bool {
        let mut val_1 = self.layout.get(0).unwrap().get(index1).unwrap();
        let mut val_2 = self.layout.get(0).unwrap().get(index2).unwrap();

        let mut row = 1;
        while row < self.layout.len() {
            if val_1 != val_2 {
                return false;
            }
            val_1 = self.layout.get(row).unwrap().get(index1).unwrap();
            val_2 = self.layout.get(row).unwrap().get(index2).unwrap();
            row += 1;
        }
        true
    }

    fn compare_row(&self, index1: usize, index2: usize) -> bool {
        let mut val_1 = self.layout.get(index1).unwrap();
        let mut val_2 = self.layout.get(index2).unwrap();

        let mut column = 0;
        while column < val_1.len() {
            if val_1.get(column).unwrap() != val_2.get(column).unwrap() {
                return false;
            }
            column += 1;
        }
        true
    }


    //~~~~~~PART 2~~~~~~~~
    fn sum_inflections2(&self) -> usize {
        let horizontal = match self.find_horizontal_inflection2() {
            Some(x) => x*100,
            None => 0
        };
        let vertical = self.find_vertical_inflection2().unwrap_or(0);
        
        if horizontal == vertical {
            panic!("horizontal and vertical should not be equal");
        }
        
        if horizontal > vertical {
            return horizontal
        }
        vertical
    }

    //returns the index of the vertical inflection, if it exists
    fn find_vertical_inflection2(&self) -> Option<usize> {
        let mut column: usize = 1;
        let length = self.layout.get(0).unwrap().len();
        while column < length {
            let mut matching_cols = 0;
            let mut diffs = 0;
            while (column - matching_cols > 0)
                && (column + matching_cols < length) {
                let left_col = column - matching_cols - 1;
                let right_col = column + matching_cols;
                diffs += self.compare_column2(left_col, right_col);
                if diffs > 1 {
                    matching_cols = 0;
                    diffs += 0;
                    break;
                }
                matching_cols += 1;
            }
            if matching_cols > 0 && diffs == 1{
                return Some(column);
            }
            column += 1;
        }
        None
    }

    //returns the index of the vertical inflection, if it exists
    fn find_horizontal_inflection2(&self) -> Option<usize> {
        let mut row: usize = 1;
        let length = self.layout.len();
        while row < length {
            let mut matching_rows = 0;
            let mut diffs = 0;
            while (row - matching_rows > 0)
                && (row + matching_rows < length) {
                let top_row = row - matching_rows - 1;
                let bottom_row = row + matching_rows;
                diffs += self.compare_row2(top_row, bottom_row);
                if diffs > 1 {
                    matching_rows = 0;
                    diffs = 0;
                    break;
                }
                matching_rows += 1;
            }
            if matching_rows > 0 && diffs == 1{
                return Some(row);
            }
            row += 1;
        }
        None
    }

    fn compare_column2(&self, index1: usize, index2: usize) -> usize {
        let mut diffs = 0;

        let mut row = 0;
        while row < self.layout.len() {
            let val_1 = self.layout.get(row).unwrap().get(index1).unwrap();
            let val_2 = self.layout.get(row).unwrap().get(index2).unwrap();
            if val_1 != val_2 {
                diffs += 1;
            }
            row += 1;
        }
        diffs
    }

    fn compare_row2(&self, index1: usize, index2: usize) -> usize {
        let mut val_1 = self.layout.get(index1).unwrap();
        let mut val_2 = self.layout.get(index2).unwrap();
        let mut diffs = 0;

        let mut column = 0;
        while column < val_1.len() {
            if val_1.get(column).unwrap() != val_2.get(column).unwrap() {
                diffs += 1;
            }
            column += 1;
        }
        diffs
    }
}

fn get_all_layouts(lines: &Vec<String>) -> Vec<Layout> {
    let mut layouts: Vec<Layout> = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let line = lines.get(index).unwrap();
        if line.is_empty() {
            index += 1;
            continue;
        }

        let new_layout = Layout::new(&lines[index..]); 
        index += new_layout.layout.len();
        layouts.push(new_layout); 
    }
    layouts
}

fn part1(lines: &Vec<String>) -> usize {
    let layouts = get_all_layouts(lines);
    let mut sum = 0;
    for layout in layouts {
        sum += layout.sum_inflections();
    }
    sum
}

fn part2(lines: &Vec<String>) -> usize {
    let layouts = get_all_layouts(lines);
    let mut sum = 0;
    for layout in layouts {
        sum += layout.sum_inflections2();
    }
    sum
}

fn main() {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten().collect::<Vec<String>>();

    println!("The result of part1 is {}", part1(&lines));
    println!("The result of part2 is {}", part2(&lines));
}

fn get_test_data() -> Vec<String> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect::<Vec<String>>()
}

fn get_test_data2() -> Vec<String> {
    let path = "example2.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect::<Vec<String>>()
}

fn get_test_data3() -> Vec<String> {
    let path = "example3.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect::<Vec<String>>()
}

fn get_test_data4() -> Vec<String> {
    let path = "example4.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect::<Vec<String>>()
}

#[test]
fn test_new_layout() {
    let lines = get_test_data();
    let layout = Layout::new(&lines);
    assert_eq!(layout.layout.len(), 7);
}

#[test]
fn test_new_layouts() {
    let lines = get_test_data();
    let layouts = get_all_layouts(&lines); 
    assert_eq!(layouts.len(), 2);
    assert_eq!(layouts.get(0).unwrap().layout.len(), 7);
    assert_eq!(layouts.get(1).unwrap().layout.len(), 7);
}

#[test]
fn test_find_vertical_inflection() {
    let lines = get_test_data();
    let layouts = get_all_layouts(&lines); 
    let inflection1 = layouts.get(0).unwrap().find_vertical_inflection();
    let inflection2 = layouts.get(1).unwrap().find_vertical_inflection();

    assert_eq!(Some(5), inflection1);
    assert_eq!(None, inflection2);
}

#[test]
fn test_find_horizontal_inflection() {
    let lines = get_test_data();
    let layouts = get_all_layouts(&lines); 
    let inflection1 = layouts.get(0).unwrap().find_horizontal_inflection();
    let inflection2 = layouts.get(1).unwrap().find_horizontal_inflection();

    assert_eq!(None, inflection1);
    assert_eq!(Some(4), inflection2);
}

#[test]
fn test_part1() {
    let lines = get_test_data2();
    let result = part1(&lines);
    assert_eq!(1107, result);
}

#[test]
fn test_edge_case() {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().flatten().collect::<Vec<String>>();
    let layouts = get_all_layouts(&lines);
    let layout = layouts.get(10).unwrap();

    let horizontal = layout.find_horizontal_inflection();
    let vertical = layout.find_vertical_inflection();
    println!("For index 10, h:{:?} v:{:?}", horizontal, vertical);

    let layout = layouts.get(24).unwrap();

    let horizontal = layout.find_horizontal_inflection();
    let vertical = layout.find_vertical_inflection();
    println!("For index 24, h:{:?} v:{:?}", horizontal, vertical);
}

#[test]
fn test_part2() {
    let lines = get_test_data();
    let result = part2(&lines);
    assert_eq!(400, result)
}

#[test]
fn test_part2_edge_case() {
    let lines = get_test_data3();
    let result = part2(&lines);
    assert_eq!(13,result)
}

#[test]
fn test_part2_edge_case2() {
    let lines = get_test_data4();
    let result = part2(&lines);
    assert_eq!(12, result);
}