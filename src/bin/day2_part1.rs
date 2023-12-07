use std::{fs, u32};

fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let input = read_file();

    let mut sum = 0;
    let mut current_game = 0;

    for line in input.lines() {
        current_game += 1;
        let trimmed_line = trim_game_text(line);

        let tosses = trimmed_line.split(";");

        let mut is_valid = true;

        for toss in tosses {
            let dices: Vec<&str> = toss.split(",").collect();
            if get_dices_thrown(dices.clone(), "red") > max_red 
                || get_dices_thrown(dices.clone(), "green") > max_green 
                || get_dices_thrown(dices.clone(), "blue") > max_blue {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            sum += current_game;
        }
    }
    println!("{sum}")
}

fn get_dices_thrown(dices: Vec<&str>, color: &str) -> u32 {
    if let Some(correct_color) = dices.iter().find(|dice| dice.contains(color)) {
        let amount = correct_color.replace(color, "");
        return amount.trim().parse::<u32>().unwrap();
    };
    0
}

fn trim_game_text(s: &str) -> &str {
    if let Some(mut index) = s.find(":") {
        index += 2;
        &s[index..]
    } else {
        s
    }
}

fn read_file() -> String {
    let contents = fs::read_to_string("inputs/2.txt").expect("Couldn't read the file.");
    contents
}
