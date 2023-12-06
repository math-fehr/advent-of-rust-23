use itertools::Itertools;
const FILENAME: &'static str = "day5/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Range {
    begin: i64,
    end: i64,
}

impl Range {
    fn intersect(self, other: Range) -> Option<Range> {
        if self.begin > other.end || self.end < other.begin {
            return None;
        }
        Some(Range {
            begin: std::cmp::max(self.begin, other.begin),
            end: std::cmp::min(self.end, other.end),
        })
    }

    fn subtract(self, other: Range) -> Vec<Range> {
        let intersect = self.intersect(other);
        if intersect.is_none() {
            return vec![self];
        }
        let intersect = intersect.unwrap();
        let mut res = Vec::new();
        if self.begin < intersect.begin {
            res.push(Range {
                begin: self.begin,
                end: intersect.begin - 1,
            });
        }
        if self.end > intersect.end {
            res.push(Range {
                begin: intersect.end + 1,
                end: self.end,
            });
        }
        res
    }
}

#[derive(PartialEq, Eq, Debug)]
struct SingleRangeMap {
    source_range: Range,
    offset: i64,
}

impl SingleRangeMap {
    fn map(&self, source: Range) -> (Option<Range>, Vec<Range>) {
        let domain_range = self.source_range.intersect(source);
        if let Some(domain_range) = domain_range {
            (
                Some(Range {
                    begin: domain_range.begin + self.offset,
                    end: domain_range.end + self.offset,
                }),
                source.subtract(domain_range),
            )
        } else {
            (None, vec![source])
        }
    }

    fn map_range(&self, source: Vec<Range>) -> (Vec<Range>, Vec<Range>) {
        let mut res_output = Vec::new();
        let mut res_remaining = Vec::new();
        for source in source.into_iter() {
            let (output, mut remaining) = self.map(source);
            if output.is_some() {
                res_output.push(output.unwrap());
            }
            res_remaining.append(&mut remaining);
        }
        (res_output, res_remaining)
    }
}

struct RangeMap(Vec<SingleRangeMap>);

impl RangeMap {
    fn map_range(&self, mut source: Vec<Range>) -> Vec<Range> {
        let mut res = Vec::new();
        for map in &self.0 {
            let (output, remaining) = map.map_range(source);
            res.append(&mut output.clone());
            source = remaining;
        }
        res.append(&mut source);
        res
    }
}

fn read_three_numbers(s: &str) -> SingleRangeMap {
    let res = s
        .trim()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    SingleRangeMap {
        source_range: Range {
            begin: res[1],
            end: res[1] + res[2] - 1,
        },
        offset: res[0] - res[1],
    }
}

fn read_seed_inputs(s: &str) -> Vec<Range> {
    s.trim()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .tuples::<(_, _)>()
        .map(|(x, y)| Range {
            begin: x,
            end: x + y - 1,
        })
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
    fn map(&self, mut source: Vec<Range>) -> Vec<Range> {
        for map in &self.0 {
            source = map.map_range(source);
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

    let end_positions = &mut rangemaps.map(seeds);
    let mut min = i64::MAX;
    for position in end_positions {
        min = std::cmp::min(min, position.begin);
    }
    println!("{}", min);
}
