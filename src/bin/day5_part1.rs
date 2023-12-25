use std::{fs, u64};

fn main() {
    let input = read_file();
    let seeds = get_seeds(&input);
    let mappings = get_ranges(&input);

    let mut lowest_location = u64::MAX;

    for seed in seeds {
        let mut cur_value = seed as u64;

        for map in &mappings {
            for range in map {
                let range_low = range.1 as u64;
                let range_high = range_low + range.2 as u64 - 1;

                if cur_value >= range_low && cur_value <= range_high {
                    cur_value = range.0 as u64 + (cur_value - range_low);
                    break;
                }
            }
        }

        if cur_value < lowest_location {
            lowest_location = cur_value;
        }
    }

    println!("{lowest_location}");
}

fn get_seeds(input: &String) -> Vec<u32> {
    let mut seeds: Vec<u32> = Vec::new();

    if let Some(first_line) = input.lines().next() {
        if let Some(numbers_part) = first_line.split(':').nth(1) {
            seeds = numbers_part
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
        }
    }

    seeds
}

fn get_ranges(input: &String) -> Vec<Vec<(u32, u32, u32)>> {
    let mut ranges: Vec<Vec<(u32, u32, u32)>> = Vec::new();

    let mut range: Vec<(u32, u32, u32)> = Vec::new();

    for line in input.lines() {
        if let Some(first_char) = line.chars().next() {
            if first_char.is_digit(10) {
                let cur_range: Vec<u32> = line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                range.push((cur_range[0], cur_range[1], cur_range[2]));
            } else {
                if !range.is_empty() {
                    ranges.push(range);
                    range = Vec::new();
                }
            }
        }
    }
    if !range.is_empty() {
        ranges.push(range);
    }

    ranges
}

fn read_file() -> String {
    let contents = fs::read_to_string("inputs/5.txt").expect("Couldn't read the file.");
    contents
}
