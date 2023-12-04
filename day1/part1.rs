const FILENAME: &'static str = "day1/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn get_first_digit(s: &str) -> u32 {
    for c in s.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    unreachable!("No integer found in the line");
}

fn get_last_digit(s: &str) -> u32 {
    for c in s.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    unreachable!("No integer found in the line");
}

fn main() {
    let mut sum = 0;
    for line in read_file().lines() {
        sum += get_first_digit(line) * 10 + get_last_digit(line);
    }
    println!("{}", sum);
}
