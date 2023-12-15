use std::fs;
use crate::matrix_handler::MatrixHandler;

mod matrix_handler {
    use std::collections::HashMap;

    pub struct MatrixHandler {
        input_matrix: Vec<Vec<char>>,
        checked_numbers: HashMap<u32, u32>,
    }

    impl MatrixHandler {
        pub fn new(input_matrix: Vec<Vec<char>>) -> MatrixHandler { 
            MatrixHandler { 
                input_matrix: input_matrix, 
                checked_numbers: HashMap::<u32, u32>::new() 
            } 
        }

        pub fn get_matrix_sum(&self) -> u32 {
            let mut x = 0;
            let mut y = 0;
            let mut sum = 0;

            while x < self.input_matrix.len() {
                while y < self.input_matrix[x].len() {
                    let cha = self.input_matrix[x][y];
                    if cha.is_ascii_digit() && is_special_char_adjacent(&self.input_matrix, x, y) {
                        let whole_digit = get_whole_digit(&self.input_matrix[x], y);
                        y = whole_digit.1;
                        sum += whole_digit.0;
                    }
                    y += 1;
                }
                y = 0;
                x += 1;
            }

            sum
        }
    }

    fn get_whole_digit(chars: &Vec<char>, index: usize) -> (u32, usize) {
        let mut first_index = index;
        let mut last_index = index;
        loop {
            if !chars[first_index].is_ascii_digit() {
                first_index += 1;
                break;
            }

            if  first_index == 0 {
                break;
            }
            first_index -= 1;
        }

        while last_index < chars.len() {
            if !chars[last_index].is_ascii_digit() {
                last_index -= 1;
                break;
            }

            if last_index == chars.len() - 1 {
                break;
            }

            if last_index < chars.len() {
                last_index += 1;
            }
        }

        let number = &chars[first_index..=last_index];
        let result = number.iter().collect::<String>().parse::<u32>().unwrap();
        (result, last_index)
    }

    fn is_special_char_adjacent(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if x > 0 && y > 0 {
            if  is_not_dot_or_digit(matrix[x - 1][y - 1]) {
                return true;
            }
        }

        if x > 0 {
            if  is_not_dot_or_digit(matrix[x - 1][y]) {
                return true;
            }
        }

        if x > 0 && matrix[x - 1].len() - 1 > y {
            if  is_not_dot_or_digit(matrix[x - 1][y + 1]) {
                return true;
            }
        }

        if matrix[x].len() - 1 > y {
            if  is_not_dot_or_digit(matrix[x][y + 1]) {
                return true;
            }
        }

        if y > 0 {
            if  is_not_dot_or_digit(matrix[x][y - 1]) {
                return true;
            }
        }
        
        if matrix.len() - 1 > x && matrix[x + 1].len() - 1 > y {
            if  is_not_dot_or_digit(matrix[x + 1][y + 1]) {
                return true;
            }
        }

        if matrix.len() - 1 > x {
            if  is_not_dot_or_digit(matrix[x + 1][y]) {
                return true;
            }
        }

        if matrix.len() - 1 > x && y > 0 {
            if  is_not_dot_or_digit(matrix[x + 1][y - 1]) {
                return true;
            }
        }

        false
    }

    fn is_not_dot_or_digit(c: char) -> bool {
        if c.is_ascii_digit() {
            return false;
        }
        if c == '.' {
            return false;
        }

        true
    }
}

fn main() {
    let input = read_file();

    let input_matrix: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let matrix_handler = MatrixHandler::new(input_matrix);
    let sum = matrix_handler.get_matrix_sum();
    println!("{sum}");
}


fn read_file() -> String {
    let contents = fs::read_to_string("inputs/3.txt").expect("Couldn't read the file.");
    contents
}
