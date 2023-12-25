use std::{fs, u32, collections::{HashSet, HashMap}};

fn main() {
    let input = read_file();
    
    let mut points: Vec<u32> = Vec::new();
    let mut extra_points: HashMap<u32, u32> = HashMap::new();

    let mut index = 0;

    for line in input.lines() {
        let mut cur_points: u32 = 1;
        if let Some(value) = extra_points.get(&index) {
            cur_points += *value;
        }
        points.push(cur_points);
        
        let card_points = get_card_points(line.to_string());
        let mut cur_index = 1;

        while cur_index <= card_points {
            if let Some(test) = extra_points.get(&(index + cur_index)) {
                extra_points.insert(index + cur_index, test + cur_points);
            } else {
                extra_points.insert(index + cur_index, cur_points);
            }

            cur_index += 1;
        }

        index += 1;
    }

    let sum: u32 = points.iter().sum();

    println!("{sum}");
}

fn get_card_points(card: String) -> u32 {
    let result_string = card.splitn(2, ':').nth(1).unwrap_or("").to_string();

    let mut points = 0;
    
    let split_card = result_string.split_once('|').unwrap();
    let winning_numbers: HashSet<String> = split_card.0
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let drawn_numbers = split_card.1.split_whitespace();

    for number in drawn_numbers {
        if winning_numbers.contains(number) {
            points += 1;
        }
    }

    points
}

fn read_file() -> String {
    let contents = fs::read_to_string("inputs/4.txt").expect("Couldn't read the file.");
    contents
}
