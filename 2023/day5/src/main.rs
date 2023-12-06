#![allow(unused, unused_imports, clippy::new_ret_no_self,clippy::collapsible_if)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map;
use std::time::Instant;
use std::hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::f64::NEG_INFINITY;
use std::str::Lines;
use std::u64::MIN;
use std::cmp::max;

use map_macro::hash_map;


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
        let diff = starting_key-starting_val;
        self.insert(key_range, diff);
    }
}

#[derive(Debug, PartialEq)]
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

    fn get_location_w_range(&self, seed_range: (i64, i64)) -> i64 {
        //Get Soil Ranges
        let soil_ranges: Vec<(i64, i64)> = get_output_from_range(seed_range, &self.seed_to_soil);

        let mut fertilizer_ranges: Vec<(i64, i64)> = Vec::new();
        for range in &soil_ranges {
            fertilizer_ranges.extend(&get_output_from_range(*range, &self.soil_to_fertilizer));
        }

        let mut water_ranges: Vec<(i64, i64)> = Vec::new();
        for range in &fertilizer_ranges {
            water_ranges.extend(&get_output_from_range(*range, &self.fertilizer_to_water));
        }

        let mut light_ranges: Vec<(i64, i64)> = Vec::new();
        for range in &water_ranges {
            light_ranges.extend(&get_output_from_range(*range, &self.water_to_light));
        }

        let mut temperature_ranges: Vec<(i64, i64)> = Vec::new();
        for range in &light_ranges {
            temperature_ranges.extend(&get_output_from_range(*range, &self.light_to_temperature));
        }

        let mut humidity_ranges: Vec<(i64, i64)> = Vec::new();
        for range in &temperature_ranges {
            humidity_ranges.extend(&get_output_from_range(*range, &self.temperature_to_humidity));
        }

        let mut location_ranges: Vec<(i64, i64)> = Vec::new();
        for range in &humidity_ranges {
            location_ranges.extend(&get_output_from_range(*range, &self.humidity_to_location));
        }

        let mut locations: Vec<i64> = Vec::new();
        for range in &location_ranges {
            locations.push(range.0);
        }

        let locations = locations.iter().filter(|x| x != &&i64::MIN);

        *locations.min().unwrap()
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

    fn get_seed_ranges(line: &str) -> Vec<(i64, i64)> {
        let raw_seeds = line.split(':')
            .last().unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        assert!(raw_seeds.len() % 2 == 0);

        let mut seed_ranges: Vec<(i64, i64)> = Vec::new();
        let mut index = 0;
        while index < raw_seeds.len() {
            let start = raw_seeds.get(index).unwrap();
            let range = raw_seeds.get(index + 1).unwrap();
            seed_ranges.push((*start, *start+range));
            index += 2;
        }

        seed_ranges
    }
}

fn get_output_from_range(seed_range: (i64, i64), conversions: &HashMap<(i64, i64), i64>) -> Vec<(i64, i64)> {
    //inputs a seed range, outputs a set of ranges from the given conversion map
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    let mut seed_range = seed_range;
    for conversion in conversions.iter() {
        let conversion_range = conversion.0;
        let conversion_val = conversion.1;
        
        // if bottom of seed range is below conversion range
        if seed_range.0 < conversion_range.0 {
            if seed_range.1 == conversion_range.1 {
                //if seed range encompasses conversion range, but does not go above
                // convert within conversion range, and set seed_range to the bottom half
                let new_range = (conversion_range.0 - conversion_val, conversion_range.1 - conversion_val);
                ranges.push(new_range);
                seed_range = (seed_range.0, conversion_range.0);
            } else if seed_range.1 > conversion_range.1 {
                //most complicated case, seed range encompasses conversion range with extra ranges on both sides
                let new_range = (conversion_range.0 - conversion_val, conversion_range.1 - conversion_val);
                let below_range = get_output_from_range((seed_range.0, conversion_range.0), conversions);
                let above_range = get_output_from_range((conversion_range.1, seed_range.1), conversions);
                ranges.push(new_range);
                ranges.extend(below_range);
                ranges.extend(above_range);
                seed_range = (i64::MAX, i64::MAX);
            } else if seed_range.1 > conversion_range.0 {
                // if top end of seed range is within the conversion range
                // convert from bottom of conversion range up to seed_range
                let new_range = (conversion_range.0 - conversion_val, seed_range.1 - conversion_val);
                // set seed range to be from bottom of seed_range to bottom of conversion range
                seed_range = (seed_range.0, conversion_range.0);
                ranges.push(new_range);
            }
            //else we're out of the conversion range entirely
            continue;
        }
        
        //if top of seed range is above conversion range
        if seed_range.1 > conversion_range.1 {
            if seed_range.0 >= conversion_range.1 {
                continue;
            } else if seed_range.0 > conversion_range.1 {
                let new_range = (seed_range.0 - conversion_val, conversion_range.1 - conversion_val);
                ranges.push(new_range);
                seed_range = (conversion_range.1, seed_range.1);
            }
        }

        //if seed range is completely within the conversion range
        if seed_range.0 >= conversion_range.0 {
            if seed_range.1 <= conversion_range.1 {
                let new_range = (seed_range.0 - conversion_val, seed_range.1 - conversion_val);
                ranges.push(new_range);
                seed_range = (i64::MAX, i64::MAX);
            }
        }
    }
    if seed_range.0 != seed_range.1 {
        ranges.push(seed_range);
    }

    ranges
}

fn part1(lines: &[String]) -> i64 {
    let start = Instant::now();
    let seed_to_location = SeedToLocation::new(lines);
    let mut locations: Vec<i64> = Vec::new();
    for seed in &seed_to_location.seeds {
        locations.push(seed_to_location.get_location(*seed));
    }
    let duration = start.elapsed();
    println!("The duration of part1 is : {:?}", duration);
    *locations.iter().min().unwrap()
}

fn part2(lines: &[String]) -> i64 {
    let start = Instant::now();
    let seed_to_location = SeedToLocation::new(lines);
    let seeds = SeedToLocation::get_seed_ranges(lines.get(0).unwrap().as_str());
    let mut locations: Vec<i64> = Vec::new();
    for seed_range in seeds {
        locations.push(seed_to_location.get_location_w_range(seed_range));
    }

    let duration = start.elapsed();
    println!("The duration of part2 is : {:?}", duration);

    *locations.iter().min().unwrap()
}

//Need to construct each map and have a process method for the map which takes in additional numbers and modifies the map
fn main() -> std::io::Result<()>{
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().flatten().collect();

    println!("The solution to part1 is {}", part1(&lines));

    println!("The solution to part2 is {}", part2(&lines));
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
    expected.insert((98,100), 48);

    let mut result: HashMap<(i64, i64), i64> = HashMap::new();
    result.process(test_line_1);


    assert_eq!(expected, result);
}

#[test]
fn test_new_map() {
    let lines = get_test_data();
    let mut expected = HashMap::new();
    expected.insert((98, 100), 48);
    expected.insert((50, 98), -2);
    let result = SeedToLocation::new_map(&lines[3..]);
    
    assert_eq!(expected, result);
}

#[test]
fn test_get_location() {
    let lines = get_test_data();
    let seed_to_location = SeedToLocation::new(&lines);
    let seeds = &seed_to_location.seeds[..];
    let result_loc_1 = seed_to_location.get_location(seeds[0]);
    let result_loc_2 = seed_to_location.get_location(seeds[1]);
    let result_loc_3 = seed_to_location.get_location(seeds[2]);
    let result_loc_4 = seed_to_location.get_location(seeds[3]);

    let expected_loc_1: i64 = 82;
    let expected_loc_2: i64 = 43;
    let expected_loc_3: i64 = 86;
    let expected_loc_4: i64 = 35;
    
    assert_eq!(expected_loc_1, result_loc_1);
    assert_eq!(expected_loc_2, result_loc_2);
    assert_eq!(expected_loc_3, result_loc_3);
    assert_eq!(expected_loc_4, result_loc_4);
}

#[test]
fn test_seed_equality() {
    let lines = get_test_data();
    let seed_to_location = SeedToLocation::new(&lines);
    let expected_seed = SeedToLocation {
        seeds: vec![79, 14, 55, 13],
        seed_to_soil: hash_map! {
            (98, 100) => 48,
            (50, 98) => -2,
        },
        soil_to_fertilizer: hash_map! {
            (52, 54) => 15,
            (0, 15) => -39,
            (15, 52) => 15
        },
        fertilizer_to_water: hash_map! {
            (7, 11) => -50,
            (0, 7) => -42, 
            (11, 53) => 11, 
            (53, 61) => 4
        }, 
        water_to_light: hash_map! {
            (25, 95) => 7,
            (18, 25) => -70
        }, 
        light_to_temperature: hash_map! {
            (77, 100) => 32,
            (64, 77) => -4, 
            (45, 64) => -36
        }, 
        temperature_to_humidity: hash_map! {
            (69, 70) => 69,
            (0, 69) => -1
        }, 
        humidity_to_location: hash_map! {
            (93, 97) => 37,
            (56, 93) => -4
        }
    };
    assert_eq!(seed_to_location, expected_seed);
}

#[test]
fn test_get_location_from_range() {
    let lines = get_test_data();
    let seed_to_location = SeedToLocation::new(&lines);

    let location = seed_to_location.get_location_w_range((79, 93));
    println!("{}", location);
    let location2 = seed_to_location.get_location_w_range((55, 68));
    println!("{}", location2);

    println!("{}", if location < location2 { location } else { location2 })
}
