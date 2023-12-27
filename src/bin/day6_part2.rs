use std::fs;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let input = read_file();

    let time = get_number(&input, 0);
    let distance = get_number(&input, 1);
    drop(input);

    let mut result: u64 = 0;
    let mut y = 1;
    while y < time {
        let calculated_distance = (time - y) * y;
        if calculated_distance > distance {
            result += 1;
        }
        y += 1;
    }

    println!("{result}");
    
    let end_time = Instant::now();
    println!("Elapsed time: {:?}", end_time - start_time);
}

fn get_number(input: &String, index: usize) -> u64 {
    let numbers = input.lines().nth(index).unwrap_or("")
        .split(":").nth(1).unwrap_or("")
        .trim();

    let number_str: String = numbers
        .split_whitespace()
        .collect();

    number_str.parse::<u64>().unwrap()
}

fn read_file() -> String {
    let contents = fs::read_to_string("inputs/6.txt").expect("Couldn't read the file.");
    contents
}
