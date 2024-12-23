use std::fs;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let input = read_file();

    let times = get_numbers(&input, 0);
    let distances = get_numbers(&input, 1);
    drop(input);

    let mut results: Vec<u32> = Vec::new();

    let mut i = 0;
    while i < times.len() {
        let cur_time = times[i];
        let cur_distance = distances[i];

        let mut result: u32 = 0;
        let mut y = 1;
        while y < cur_time {
            let calculated_distance = (cur_time - y) * y;
            if calculated_distance > cur_distance {
                result += 1;
            }
            y += 1;
        }
        results.push(result);
        i += 1;
    }

    let result: u32 = results.iter().product();
    println!("{result}");
    
    let end_time = Instant::now();
    println!("Elapsed time: {:?}", end_time - start_time);
}

fn get_numbers(input: &String, index: usize) -> Vec<u32> {
    let numbers = input.lines().nth(index).unwrap_or("")
        .split(":").nth(1).unwrap_or("")
        .trim();

    numbers
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect()
}

fn read_file() -> String {
    let contents = fs::read_to_string("inputs/6.txt").expect("Couldn't read the file.");
    contents
}
