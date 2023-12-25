use std::{fs, u32, collections::HashSet};

fn main() {
    let input = read_file();
    
    let mut points: Vec<u32> = Vec::new();

    for line in input.lines() {
        let card_points = get_card_points(line.to_string());
        if card_points > 0 {
            points.push(card_points);
        }
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
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }

    points
}

fn read_file() -> String {
    let contents = fs::read_to_string("inputs/4.txt").expect("Couldn't read the file.");
    contents
}
