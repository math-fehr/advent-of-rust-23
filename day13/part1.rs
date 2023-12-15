use std::collections::HashSet;

const FILENAME: &'static str = "day13/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn read_one_problem(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect()
}

fn read_problems(input: &str) -> Vec<Vec<Vec<bool>>> {
    input
        .split("\n\n")
        .map(|problem| read_one_problem(problem))
        .collect()
}

fn is_mirror(input: &[bool], split: usize) -> bool {
    let left = input[..split].iter().rev();
    let right = input[split..].iter();
    left.zip(right).all(|(l, r)| l == r)
}

fn try_mirrors(input: &[bool], candidates: &Vec<usize>) -> Vec<usize> {
    candidates
        .iter()
        .filter(|&&split| is_mirror(input, split))
        .map(|&split| split)
        .collect()
}

fn has_vertical_mirror(input: &[Vec<bool>]) -> Option<usize> {
    let mut candidates = (1..input[0].len()).collect();
    for line in input {
        candidates = try_mirrors(line, &candidates);
        if candidates.is_empty() {
            return None;
        }
    }
    assert!(candidates.len() == 1);
    Some(candidates[0])
}

fn has_horizontal_mirror(input: &[Vec<bool>]) -> Option<usize> {
    let mut candidates = (1..input.len()).collect();
    for i in 0..input[0].len() {
        let line: Vec<bool> = input.iter().map(|row| row[i]).collect();
        candidates = try_mirrors(&line, &candidates);
        if candidates.is_empty() {
            return None;
        }
    }
    assert!(candidates.len() == 1);
    Some(candidates[0])
}

fn get_mirror_value(input: &[Vec<bool>]) -> usize {
    let vertical = has_vertical_mirror(input);
    if let Some(v) = vertical {
        return v;
    }
    let horizontal = has_horizontal_mirror(input);
    100 * horizontal.unwrap()
}

fn main() {
    let inputs = read_problems(&read_file());
    let result: usize = inputs.iter().map(|input| get_mirror_value(input)).sum();
    println!("{}", result);
}
