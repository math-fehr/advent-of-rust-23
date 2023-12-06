use std::collections::HashSet;

const FILENAME: &'static str = "day4/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn get_numbers_from_file(input: &str) -> Vec<(HashSet<i32>, Vec<i32>)> {
    let mut res = Vec::new();
    for line in input.lines() {
        let line = line.split(":").nth(1).unwrap().trim();
        let left = line.split("|").nth(0).unwrap().trim();
        let right = line.split("|").nth(1).unwrap().trim();
        let left_values = left
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| x != &"")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<HashSet<i32>>();
        let right_values = right
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| x != &"")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        res.push((left_values, right_values));
    }
    res
}

fn add_winning_card_to_collection(
    cards: &mut Vec<i32>,
    start: i32,
    num_cards: i32,
    multiplier: i32,
) {
    for i in start..(start + num_cards) {
        cards[i as usize] += multiplier;
    }
}

fn get_num_winning_numbers(winning_numbers: &HashSet<i32>, numbers: &Vec<i32>) -> i32 {
    let mut res = 0;
    for number in numbers {
        if winning_numbers.contains(number) {
            res += 1;
        }
    }
    res
}

fn main() {
    let cards = get_numbers_from_file(&read_file());
    let mut num_cards = vec![1; cards.len()];

    for (idx, (winning_numbers, numbers)) in cards.iter().enumerate() {
        let num_winning = get_num_winning_numbers(winning_numbers, numbers);
        let multiplier = num_cards[idx];
        add_winning_card_to_collection(&mut num_cards, (idx + 1) as i32, num_winning, multiplier);
    }
    println!("{}", num_cards.iter().sum::<i32>());
}
