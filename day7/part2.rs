use std::{cmp::Ordering, collections::HashMap};

const FILENAME: &'static str = "day7/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn kind_to_value(kind: char) -> i64 {
    match kind {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => kind.to_digit(10).unwrap() as i64,
    }
}

fn read_hand(kind: &str) -> [i64; 5] {
    let mut hand = [0; 5];
    for (i, card) in kind.chars().enumerate() {
        hand[i] = kind_to_value(card);
    }
    hand
}

fn read_hands(s: &str) -> Vec<([i64; 5], i64)> {
    s.lines()
        .map(|line| {
            let hand = read_hand(line.split_whitespace().nth(0).unwrap());
            let bid = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i64>()
                .unwrap();
            (hand, bid)
        })
        .collect()
}

fn get_hand_type(hand: [i64; 5]) -> i64 {
    let mut values = HashMap::new();
    for card in hand.iter() {
        let count = values.entry(*card).or_insert(0);
        *count += 1;
    }
    let j_value = values.remove(&1).unwrap_or(0);
    let max_value = *values.values().max().unwrap_or(&0) + j_value;
    let min_value = *values.values().min().unwrap_or(&0);
    if max_value == 5 {
        return 6;
    }
    if max_value == 4 {
        return 5;
    }
    if max_value == 3 && min_value == 2 {
        return 4;
    }
    if max_value == 3 {
        return 3;
    }
    if values.values().filter(|&&x| x == 2).count() == 2 {
        return 2;
    }
    if max_value == 2 {
        return 1;
    }
    0
}

fn is_greater_best_card(hand: [i64; 5], other: [i64; 5]) -> Ordering {
    for i in 0..5 {
        if hand[i] > other[i] {
            return Ordering::Greater;
        }
        if hand[i] < other[i] {
            return Ordering::Less;
        }
    }
    unreachable!("Two hands can't be equal")
}

fn is_greater(hand: [i64; 5], other: [i64; 5]) -> Ordering {
    let hand_type = get_hand_type(hand);
    let other_type = get_hand_type(other);
    if hand_type > other_type {
        return Ordering::Greater;
    }
    if hand_type < other_type {
        return Ordering::Less;
    }
    return is_greater_best_card(hand, other);
}

fn main() {
    let mut hands = read_hands(&read_file());
    hands.sort_by(|(hand, _), (other, _)| is_greater(*hand, *other));
    let res = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as i64 * bid)
        .sum::<i64>();
    println!("{}", res);
}
