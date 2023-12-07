use itertools::Itertools;
const FILENAME: &'static str = "day6/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn read_number(s: &str) -> i64 {
    s.split_whitespace()
        .skip(1)
        .join("")
        .parse::<i64>()
        .unwrap()
}

fn will_win(time: i64, distance: i64, time_pressed: i64) -> bool {
    (time - time_pressed) * time_pressed > distance
}

fn how_many_ways_win(time: i64, distance: i64) -> i64 {
    let mut ways = 0;
    for time_pressed in 1..time {
        if will_win(time, distance, time_pressed) {
            ways += 1;
        }
    }
    ways
}

fn main() {
    let file = read_file();
    let time = read_number(file.lines().nth(0).unwrap());
    let distance = read_number(file.lines().nth(1).unwrap());
    println!("{}", how_many_ways_win(time, distance));
}
