use std::collections::{HashMap, HashSet};

const FILENAME: &'static str = "day3/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NumberPosition {
    pub line: i32,
    pub col_start: i32,
    pub col_end: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub line: i32,
    pub col: i32,
}

impl NumberPosition {
    fn get_gears(&self, map: &Vec<Vec<char>>) -> Vec<Position> {
        let max_lines = (map.len() - 1) as i32;
        let max_columns = (map[0].len() - 1) as i32;

        let line_start = i32::max(0, self.line - 1);
        let line_end = i32::min(self.line + 1, max_lines);
        let col_start = i32::max(0, self.col_start - 1);
        let col_end = i32::min(self.col_end + 1, max_columns);

        let mut res = Vec::new();

        for line in line_start..=line_end {
            for col in col_start..=col_end {
                if map[line as usize][col as usize] == '*' {
                    res.push(Position { line, col });
                }
            }
        }
        res
    }

    fn get_value(&self, map: &Vec<Vec<char>>) -> i32 {
        let mut value = 0;
        for col in self.col_start..=self.col_end {
            let c = map[self.line as usize][col as usize];
            value = value * 10 + c.to_digit(10).unwrap() as i32;
        }
        value
    }
}

fn get_numbers(map: &Vec<Vec<char>>) -> Vec<NumberPosition> {
    let mut numbers: Vec<NumberPosition> = Vec::new();

    let mut start_col = Option::<i32>::None;
    for (l_idx, line) in map.iter().enumerate() {
        for (c_idx, c) in line.iter().enumerate() {
            // Start or continue the number
            if c.is_digit(10) {
                if start_col.is_none() {
                    start_col = Some(c_idx as i32)
                }
            }
            // End the number
            else {
                if let Some(start_col_val) = start_col {
                    numbers.push(NumberPosition {
                        line: l_idx as i32,
                        col_start: start_col_val,
                        col_end: (c_idx - 1) as i32,
                    });
                    start_col = None;
                }
            }
        }
        if let Some(start_col_val) = start_col {
            numbers.push(NumberPosition {
                line: l_idx as i32,
                col_start: start_col_val,
                col_end: (line.len() - 1) as i32,
            });
            start_col = None;
        }
    }

    numbers
}

fn main() {
    let input = read_file();
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut gears: HashMap<Position, HashSet<NumberPosition>> = HashMap::new();
    for number in get_numbers(&map) {
        for gear in number.get_gears(&map) {
            gears
                .entry(gear)
                .or_insert(HashSet::<NumberPosition>::new())
                .insert(number);
        }
    }

    let mut res = 0;
    for gear in gears {
        if gear.1.len() == 2 {
            let mut gears = gear.1.iter();
            let first = gears.next().unwrap();
            let second = gears.next().unwrap();
            res += first.get_value(&map) * second.get_value(&map);
        }
    }

    println!("{}", res);
}
