use std::{fs, u32};

fn main() {
    let integers = vec!("one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9");

    let input = read_file();
    let mut sum = 0;
    for line in input.lines() {
        let first_int = find_first_instance(line.to_string(), integers.clone(), false);
        let second_int = find_first_instance(line.chars().rev().collect(), integers.clone(), true);
        
        let number = format!("{}{}", parse_string_to_number(first_int), parse_string_to_number(second_int));
        sum += number.parse::<u32>().unwrap();
    }
    println!("{sum}");
}

fn parse_string_to_number(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    }
}

fn find_first_instance(s: String, patterns: Vec<&str>, reverse: bool) -> &str {
    let mut first_instance = "";
    let mut last_index = usize::MAX;
    for pattern in patterns {
        if reverse {
            if let Some(index) = s.find(&pattern.chars().rev().collect::<String>()) {
                if index < last_index {
                    first_instance = pattern;
                    last_index = index;
                }
            }
        } else {
            if let Some(index) = s.find(&pattern) {
                if index < last_index {
                    first_instance = pattern;
                    last_index = index;
                }
            }
        }
    }

    first_instance
}

fn read_file() -> String {
    let contents = fs::read_to_string("inputs/1_2.txt").expect("Couldn't read the file.");
    contents
}
