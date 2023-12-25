use std::fs;
use crate::matrix_handler::MatrixHandler;

mod matrix_handler {
    use std::collections::HashSet;

    pub struct MatrixHandler {
        input_matrix: Vec<Vec<char>>,
        checked_numbers: HashSet<(usize, usize)>,
    }

    impl MatrixHandler {
        pub fn new(input_matrix: Vec<Vec<char>>) -> MatrixHandler { 
            MatrixHandler { 
                input_matrix: input_matrix, 
                checked_numbers: HashSet::<(usize, usize)>::new() 
            } 
        }

        pub fn get_matrix_sum(mut self) -> u32 {
            let mut x = 0;
            let mut y = 0;
            let mut sum = 0;

            while x < self.input_matrix.len() {
                while y < self.input_matrix[x].len() {
                    let cha = self.input_matrix[x][y];
                    if cha.is_ascii_digit() {
                        let cur_sum = self.get_sum(x, y, true);
                        y = cur_sum.1;

                        let result = get_result_sum(cur_sum.0);
                        sum += result;
                    }
                    y += 1;
                }
                y = 0;
                x += 1;
            }

            sum
        }

        fn get_sum(&mut self, x: usize, y: usize, is_digit: bool) -> (Vec<char>, usize) {
            let mut result_chars: Vec<char> = Vec::new();
            let mut whole_digit = get_whole_digit(&self.input_matrix[x], y, !is_digit);

            if !&self.checked_numbers.contains(&(x, whole_digit.1)) {
                self.checked_numbers.insert((x, whole_digit.1));
                result_chars.append(&mut whole_digit.0);
                
                let mut next_chars: Vec<char> = self.is_char_adjacent(x, y, whole_digit.2, !is_digit); 
                result_chars.append(&mut next_chars);
            }

            (result_chars, whole_digit.2)
        }

        fn is_char_adjacent(&mut self, x: usize, y: usize, last_y: usize, find_digit: bool) -> Vec<char> {
            let mut result_chars: Vec<char> = Vec::new();
            let mut cur_y = y;
            while cur_y <= last_y {
                if x > 0 && cur_y > 0 {
                    if  is_special_or_digit(self.input_matrix[x - 1][cur_y - 1], find_digit) {
                        result_chars.append(&mut self.get_sum(x - 1, cur_y - 1, find_digit).0);
                    }
                }

                if x > 0 {
                    if  is_special_or_digit(self.input_matrix[x - 1][cur_y], find_digit) {
                        result_chars.append(&mut self.get_sum(x - 1, cur_y, find_digit).0);
                    }
                }

                if x > 0 && self.input_matrix[x - 1].len() - 1 > cur_y {
                    if  is_special_or_digit(self.input_matrix[x - 1][cur_y + 1], find_digit) {
                        result_chars.append(&mut self.get_sum(x - 1, cur_y + 1, find_digit).0);
                    }
                }

                if self.input_matrix[x].len() - 1 > cur_y {
                    if  is_special_or_digit(self.input_matrix[x][cur_y + 1], find_digit) {
                        result_chars.append(&mut self.get_sum(x, cur_y + 1, find_digit).0);
                    }
                }

                if cur_y > 0 {
                    if  is_special_or_digit(self.input_matrix[x][cur_y - 1], find_digit) {
                        result_chars.append(&mut self.get_sum(x, cur_y - 1, find_digit).0);
                    }
                }
                
                if self.input_matrix.len() - 1 > x && self.input_matrix[x + 1].len() - 1 > cur_y {
                    if  is_special_or_digit(self.input_matrix[x + 1][cur_y + 1], find_digit) {
                        result_chars.append(&mut self.get_sum(x + 1, cur_y + 1, find_digit).0);
                    }
                }

                if self.input_matrix.len() - 1 > x {
                    if  is_special_or_digit(self.input_matrix[x + 1][cur_y], find_digit) {
                        result_chars.append(&mut self.get_sum(x + 1, cur_y, find_digit).0);
                    }
                }

                if self.input_matrix.len() - 1 > x && cur_y > 0 {
                    if  is_special_or_digit(self.input_matrix[x + 1][cur_y - 1], find_digit) {
                        result_chars.append(&mut self.get_sum(x + 1, cur_y - 1, find_digit).0);
                    }
                }

                cur_y += 1;
            }

            result_chars
        }
    }


    fn get_whole_digit(chars: &Vec<char>, index: usize, is_digit: bool) -> (Vec<char>, usize, usize) {
        if is_digit {
            return (vec![chars[index]], index, index);
        }

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
        (number.to_vec(), first_index, last_index)
    }

    fn is_special_or_digit(c: char, is_digit: bool) -> bool {
        if is_digit {
            if !c.is_ascii_digit() {
                return false;
            }
        } else {
            if c.is_ascii_digit() {
                return false;
            }
            if c == '.' {
                return false;
            }
        }

        true
    }


    fn get_result_sum (chars: Vec<char>) -> u32 {
        let mut result: Vec<String> = Vec::new();
        let mut current_number = String::new();

        for &ch in chars.iter() {
            if ch.is_digit(10) {
                current_number.push(ch);
            } else if !current_number.is_empty() {
                result.push(current_number.clone());
                current_number.clear();
            }
        }

        if !current_number.is_empty() {
            result.push(current_number);
        }

        if result.len() == 2 {
            return result[0].parse::<u32>().unwrap() * result[1].parse::<u32>().unwrap();
        }

        0
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
