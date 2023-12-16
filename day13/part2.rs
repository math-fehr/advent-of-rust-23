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

fn num_differences(input: &[bool], split: usize) -> i32 {
    let left = input[..split].iter().rev();
    let right = input[split..].iter();
    let mut num_diff = 0;
    for _ in left.zip(right).filter(|(l, r)| l != r) {
        num_diff += 1;
        if num_diff > 1 {
            return 2;
        }
    }
    num_diff
}

fn try_mirrors(input: &[bool], candidates: &Vec<(usize, bool)>) -> Vec<(usize, bool)> {
    let mut res = Vec::new();
    for (candidate, has_mirrored) in candidates {
        let num_diff = num_differences(input, *candidate);
        match (has_mirrored, num_diff) {
            (false, num_diff) if num_diff < 2 => res.push((*candidate, num_diff == 1)),
            (true, 0) => res.push((*candidate, true)),
            _ => (),
        };
    }
    res
}

fn has_vertical_mirror(input: &[Vec<bool>]) -> Option<usize> {
    let mut candidates = (1..input[0].len()).map(|x| (x, false)).collect();
    for line in input {
        candidates = try_mirrors(line, &candidates);
        if candidates.is_empty() {
            return None;
        }
    }
    for (candidate, has_difference) in candidates {
        if has_difference {
            return Some(candidate);
        }
    }
    None
}

fn has_horizontal_mirror(input: &[Vec<bool>]) -> Option<usize> {
    let mut candidates = (1..input.len()).map(|x| (x, false)).collect();
    for i in 0..input[0].len() {
        let line: Vec<bool> = input.iter().map(|row| row[i]).collect();
        candidates = try_mirrors(&line, &candidates);
        if candidates.is_empty() {
            return None;
        }
    }
    for (candidate, has_difference) in candidates {
        if has_difference {
            return Some(candidate);
        }
    }
    None
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
    let now = std::time::Instant::now();
    let result: usize = inputs.iter().map(|input| get_mirror_value(input)).sum();
    let elapsed = now.elapsed();
    println!("{} {:?}", result, elapsed);
}
