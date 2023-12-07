use std::collections::HashSet;

const FILENAME: &'static str = "day6/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn read_number_vector(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
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
    let times = read_number_vector(file.lines().nth(0).unwrap());
    let distances = read_number_vector(file.lines().nth(1).unwrap());

    let res: i64 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| how_many_ways_win(*time, *distance))
        .product();
    println!("{}", res);
}
