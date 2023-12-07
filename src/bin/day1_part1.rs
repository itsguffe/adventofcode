use std::fs;

fn main() {
    let integers = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let input = read_file();
    let mut sum = 0;
    for line in input.lines() {
        let mut first_int = '0';
        for char in line.chars() {
            if integers.contains(&char) {
                first_int = char;
                break;
            }
        }
        let mut second_int = '0';
        for char in line.chars().rev() {
            if integers.contains(&char) {
                second_int = char;
                break;
            }
        }
        let number = format!("{}{}", first_int, second_int);
        sum += number.parse::<u32>().unwrap();
    }
    println!("{sum}");
}

fn read_file() -> String {
    let contents = fs::read_to_string("inputs/1.txt").expect("Couldn't read the file.");
    contents
}
