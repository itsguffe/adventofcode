use std::{fs, u32};

fn main() {
    let input = read_file();

    let mut sum = 0;

    for line in input.lines() {
        let trimmed_line = trim_game_text(line);

        let tosses = trimmed_line.split(";");

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for toss in tosses {
            let dices: Vec<&str> = toss.split(",").collect();
            let red = get_dices_thrown(dices.clone(), "red");
            if red > max_red {
                max_red = red;
            }
            let blue = get_dices_thrown(dices.clone(), "blue");
            if blue > max_blue {
                max_blue = blue;
            }
            let green = get_dices_thrown(dices.clone(), "green");
            if green > max_green {
                max_green = green;
            }
        }
        sum += max_red * max_green * max_blue;
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
