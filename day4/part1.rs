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

fn main() {
    let games = get_numbers_from_file(&read_file());
    let mut res = 0;
    for (winning_numbers, numbers) in games {
        let mut count = 0;
        for number in numbers {
            if winning_numbers.contains(&number) {
                count += 1;
            }
        }
        res += if count == 0 { 0 } else { 1 << (count - 1) };
    }
    println!("{}", res);
}
