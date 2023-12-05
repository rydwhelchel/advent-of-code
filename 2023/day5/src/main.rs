#![allow(unused, unused_imports, clippy::new_ret_no_self)]

use std::collections::HashMap;
use std::f64::NEG_INFINITY;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::str::Lines;
use std::u64::MIN;
use std::cmp::max;

pub trait Process {
    // Not returning anything, modify in place
    fn process(&mut self, line: &str);
}

impl Process for HashMap<(i64, i64), i64> {
    fn process(&mut self, line: &str) {
        //example input: "50 98 2"
        //example output: {(98,100):-48}
        //  explanation: the key is a range, which is 98 + 2, the value is what you modify the number by to get the output (98-48 == 50) 
        //take in a new line, process it for the current map
        // if we are talking seed to soil, use the second number as the seed range start and the first number as the soil range start
        let vals = line.split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        assert!(vals.len() == 3); // should only ever have a starting_value, starting_key and a range
        let starting_key = *vals.get(1).unwrap();
        let starting_val = *vals.first().unwrap();
        let range = *vals.last().unwrap();

        let key_range = (starting_key, starting_key+range);
        let diff = starting_val-starting_key;
        self.insert(key_range, diff);
    }
}

#[derive(Debug)]
struct SeedToLocation {
    seeds: Vec<i64>,
    seed_to_soil: HashMap<(i64, i64), i64>,
    soil_to_fertilizer: HashMap<(i64, i64), i64>,
    fertilizer_to_water: HashMap<(i64, i64), i64>,
    water_to_light: HashMap<(i64, i64), i64>,
    light_to_temperature: HashMap<(i64, i64), i64>,
    temperature_to_humidity: HashMap<(i64, i64), i64>,
    humidity_to_location: HashMap<(i64, i64), i64>
}

impl SeedToLocation {
    fn new(lines: &[String]) -> SeedToLocation {
        //placeholder vals so rust doesn't yell at me
        let mut seeds: Vec<i64> = vec![];
        let mut seed_to_soil: HashMap<(i64, i64), i64> = HashMap::new();
        let mut soil_to_fertilizer: HashMap<(i64, i64), i64> = HashMap::new();
        let mut fertilizer_to_water: HashMap<(i64, i64), i64> = HashMap::new();
        let mut water_to_light: HashMap<(i64, i64), i64> = HashMap::new();
        let mut light_to_temperature: HashMap<(i64, i64), i64> = HashMap::new();
        let mut temperature_to_humidity: HashMap<(i64, i64), i64> = HashMap::new();
        let mut humidity_to_location: HashMap<(i64, i64), i64> = HashMap::new();

        for (index, line) in lines.iter().enumerate() {
            if line.starts_with("seeds: ") {
                seeds = SeedToLocation::get_seeds(line);
            } else if (line.starts_with("seed-to-soil map:")) {
                seed_to_soil = SeedToLocation::new_map(&lines[index+1..]);
            } else if (line.starts_with("soil-to-fertilizer map:")) {
                soil_to_fertilizer = SeedToLocation::new_map(&lines[index+1..]);
            } else if (line.starts_with("fertilizer-to-water map:")) {
                fertilizer_to_water = SeedToLocation::new_map(&lines[index+1..]);
            } else if (line.starts_with("water-to-light map:")) {
                water_to_light = SeedToLocation::new_map(&lines[index+1..]);
            } else if (line.starts_with("light-to-temperature map:")) {
                light_to_temperature = SeedToLocation::new_map(&lines[index+1..]);
            } else if (line.starts_with("temperature-to-humidity map:")) {
                temperature_to_humidity = SeedToLocation::new_map(&lines[index+1..]);
            } else if (line.starts_with("humidity-to-location map:")) {
                humidity_to_location = SeedToLocation::new_map(&lines[index+1..]);
            }
        }

        SeedToLocation { seeds,
            seed_to_soil, 
            soil_to_fertilizer, 
            fertilizer_to_water, 
            water_to_light, 
            light_to_temperature, 
            temperature_to_humidity, 
            humidity_to_location 
        }
    }

    fn get_location(&self, seed: i64) -> i64 {
        //TODO: Going to the movies, debug later!
        //Get Soil
        let curr = seed;
        let mut soil: i64 = i64::MIN;
        for key in self.seed_to_soil.keys() {
            if curr >= key.0 && curr < key.1 {
                soil = curr - self.seed_to_soil.get(key).unwrap();
                break;
            }
        }

        //Get Fertilizer
        let curr = if soil > i64::MIN { soil } else { curr };
        let mut fertilizer: i64 = i64::MIN;
        for key in self.soil_to_fertilizer.keys() {
            if curr >= key.0 && curr < key.1 {
                fertilizer = curr - self.soil_to_fertilizer.get(key).unwrap();
                break;
            }
        }

        //Get Water
        let curr = if fertilizer > i64::MIN { fertilizer } else { curr };
        let mut water: i64 = i64::MIN;
        for key in self.fertilizer_to_water.keys() {
            if curr >= key.0 && curr < key.1 {
                water = curr - self.fertilizer_to_water.get(key).unwrap();
                break;
            }
        }

        //Get Light
        let curr = if water > i64::MIN { water } else { curr };
        let mut light: i64 = i64::MIN;
        for key in self.water_to_light.keys() {
            if curr >= key.0 && curr < key.1 {
                light = curr - self.water_to_light.get(key).unwrap();
                break;
            }
        }

        //Get Temperature
        let curr = if light > i64::MIN { light } else { curr };
        let mut temperature: i64 = i64::MIN;
        for key in self.light_to_temperature.keys() {
            if curr >= key.0 && curr < key.1 {
                temperature = curr - self.light_to_temperature.get(key).unwrap();
                break;
            }
        }

        //Get Humidity
        let curr = if temperature > i64::MIN { temperature } else { curr };
        let mut humidity: i64 = i64::MIN;
        for key in self.temperature_to_humidity.keys() {
            if curr >= key.0 && curr < key.1 {
                humidity = curr - self.temperature_to_humidity.get(key).unwrap();
                break;
            }
        }

        //Get Location
        let curr = if humidity > i64::MIN { humidity } else { curr };
        let mut location: i64 = i64::MIN;
        for key in self.humidity_to_location.keys() {
            if curr >= key.0 && curr < key.1 {
                location = curr - self.humidity_to_location.get(key).unwrap();
                break;
            }
        }
        if location > i64::MIN {
            return location 
        }
        curr
    }

    fn new_map(lines: &[String]) -> HashMap<(i64, i64), i64> {
        //this function takes in a slice of lines which contains the range values for a given map
        //  returns the constructed map from that slice
        let mut conversion = HashMap::new();
        for line in lines {
            if line.is_empty() {
                return conversion;
            }
            conversion.process(line);
        }
        conversion
    }

    fn get_seeds(line: &str) -> Vec<i64> {
        line.split(':')
            .last().unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }
}

fn part1(lines: &[String]) {

}

//Need to construct each map and have a process method for the map which takes in additional numbers and modifies the map
fn main() -> std::io::Result<()>{
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();


    Ok(())
}

fn get_test_data() -> Vec<String> {
    let path = "example.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().flatten().collect()
}

#[test]
fn test_new_seeds() {
    let lines =  get_test_data();
    let seeds = SeedToLocation::get_seeds(lines.get(0).unwrap());

    let expected:Vec<i64> = vec![79, 14, 55, 13];

    assert_eq!(expected, seeds)
}

#[test]
fn test_process() {
    let lines = get_test_data();
    let test_line_1 = lines.get(3).unwrap();

    let mut expected: HashMap<(i64, i64), i64> = HashMap::new();
    expected.insert((98,100), -48);

    let mut result: HashMap<(i64, i64), i64> = HashMap::new();
    result.process(test_line_1);


    assert_eq!(expected, result);
}

#[test]
fn test_new_map() {
    let lines = get_test_data();
    let mut expected = HashMap::new();
    expected.insert((98, 100), -48);
    expected.insert((50, 98), 2);
    let result = SeedToLocation::new_map(&lines[3..]);
    
    assert_eq!(expected, result);
}

#[test]
fn test_get_location() {
    let lines = get_test_data();
    let seed_to_location = SeedToLocation::new(&lines);
    let seeds = &seed_to_location.seeds[..];
    let result_loc_1 = seed_to_location.get_location(seeds[0]);
    // let result_loc_2 = seed_to_location.get_location(seeds[1]);
    // let result_loc_3 = seed_to_location.get_location(seeds[2]);
    // let result_loc_4 = seed_to_location.get_location(seeds[3]);

    let expected_loc_1: i64 = 82;
    let expected_loc_2: i64 = 43;
    let expected_loc_3: i64 = 86;
    let expected_loc_4: i64 = 35;
    
    assert_eq!(expected_loc_1, result_loc_1);
//     assert_eq!(expected_loc_2, result_loc_1);
//     assert_eq!(expected_loc_3, result_loc_1);
//     assert_eq!(expected_loc_4, result_loc_1);
}
