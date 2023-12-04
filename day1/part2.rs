const FILENAME: &'static str = "day1/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn is_digit(s: &str, index: usize) -> Option<u32> {
    let c = s.chars().nth(index).unwrap();
    if c.is_digit(10) {
        return Some(c.to_digit(10).unwrap());
    }
    if index == s.len() - 1 {
        return None;
    }
    if index + 2 < s.len() {
        if &s[index..index + 3] == "one" {
            return Some(1);
        }
        if &s[index..index + 3] == "two" {
            return Some(2);
        }
        if &s[index..index + 3] == "six" {
            return Some(6);
        }
    }
    if index + 3 < s.len() {
        if &s[index..index + 4] == "four" {
            return Some(4);
        }
        if &s[index..index + 4] == "five" {
            return Some(5);
        }
        if &s[index..index + 4] == "nine" {
            return Some(9);
        }
    }
    if index + 4 < s.len() {
        if &s[index..index + 5] == "three" {
            return Some(3);
        }
        if &s[index..index + 5] == "seven" {
            return Some(7);
        }
        if &s[index..index + 5] == "eight" {
            return Some(8);
        }
    }
    None
}

fn get_first_digit(s: &str) -> u32 {
    for i in 0..s.len() {
        if let Some(digit) = is_digit(s, i) {
            return digit;
        }
    }
    unreachable!("No integer found in the line");
}

fn get_last_digit(s: &str) -> u32 {
    for i in (0..s.len()).rev() {
        if let Some(digit) = is_digit(s, i) {
            return digit;
        }
    }
    unreachable!("No integer found in the line");
}

fn main() {
    let mut sum = 0;
    for line in read_file().lines() {
        sum += get_first_digit(line) * 10 + get_last_digit(line);
        println!(
            "{} {}",
            line,
            get_first_digit(line) * 10 + get_last_digit(line)
        );
    }
    println!("{}", sum);
}
