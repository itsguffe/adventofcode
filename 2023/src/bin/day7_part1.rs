use std::fs;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let input = read_file();

    let cards = get_cards(&input);
    let test = score_cards(cards);

    let end_time = Instant::now();
    println!("Elapsed time: {:?}", end_time - start_time);
}

fn score_cards(cards: Vec<(String, u32)>) -> Vec<Vec<(String, u32)>> {
    let mut scored_cards: Vec<Vec<(String, u32)>> = Vec::new();

    for card in cards {
        let test1 = card.0;
        let test2 = card.1;

        println!("{test1} - {test2}");
    }

    scored_cards
}

fn get_cards(input: &String) -> Vec<(String, u32)> {
    let mut cards: Vec<(String, u32)> = Vec::new();
    for line in input.lines() {
        let mut card = line.split(' ');
        cards.push((card.nth(0).unwrap().to_string(), card.nth(0).unwrap().parse::<u32>().unwrap()));
    }

    cards
}

fn read_file() -> String {
    let contents = fs::read_to_string("inputs/7.txt").expect("Couldn't read the file.");
    contents
}
