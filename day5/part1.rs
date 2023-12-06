use std::collections::HashSet;

const FILENAME: &'static str = "day5/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

#[derive(PartialEq, Eq)]
struct SingleRangeMap {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

impl SingleRangeMap {
    fn map(&self, source: i64) -> Option<i64> {
        if source >= self.source_start && source < self.source_start + self.range {
            Some(self.destination_start + (source - self.source_start))
        } else {
            None
        }
    }
}

struct RangeMap(Vec<SingleRangeMap>);

impl RangeMap {
    fn map(&self, source: i64) -> i64 {
        for map in &self.0 {
            if let Some(destination) = map.map(source) {
                return destination;
            }
        }
        source
    }
}

fn read_three_numbers(s: &str) -> SingleRangeMap {
    let res = s
        .trim()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    SingleRangeMap {
        destination_start: res[0],
        source_start: res[1],
        range: res[2],
    }
}

fn read_seed_inputs(s: &str) -> Vec<i64> {
    s.trim()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn read_one_rangemap(s: &str) -> RangeMap {
    let mut res = Vec::new();
    for line in s.lines().skip(1) {
        res.push(read_three_numbers(line));
    }
    RangeMap(res)
}

struct RangeMaps(Vec<RangeMap>);

impl RangeMaps {
    fn map(&self, mut source: i64) -> i64 {
        for map in &self.0 {
            source = map.map(source);
        }
        source
    }
}

fn main() {
    let input = read_file();
    let seeds = read_seed_inputs(input.lines().nth(0).unwrap());
    let rangemaps = RangeMaps(
        input
            .split("\n\n")
            .skip(1)
            .map(|x| read_one_rangemap(x))
            .collect(),
    );

    let mut min_value = i64::MAX;
    for seed in seeds {
        min_value = std::cmp::min(min_value, rangemaps.map(seed));
    }
    println!("{}", min_value);
}
