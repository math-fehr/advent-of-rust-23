use std::collections::{HashMap, HashSet};

const FILENAME: &'static str = "day16/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn get_map(file: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        map.push(line.chars().collect());
    }
    map
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_vertical(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            _ => false,
        }
    }

    fn get_index(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 3,
            Direction::Down => 1,
            Direction::Left => 2,
        }
    }
}

#[derive(Copy, Clone, Hash)]
struct Position {
    y: i32,
    x: i32,
}

impl Position {
    fn next_pos(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                y: self.y - 1,
                x: self.x,
            },
            Direction::Down => Position {
                y: self.y + 1,
                x: self.x,
            },
            Direction::Left => Position {
                y: self.y,
                x: self.x - 1,
            },
            Direction::Right => Position {
                y: self.y,
                x: self.x + 1,
            },
        }
    }
}

fn get_next_directions(mirror: char, direction: Direction) -> Vec<Direction> {
    match mirror {
        '|' if direction.is_vertical() => vec![direction],
        '|' if !direction.is_vertical() => vec![Direction::Up, Direction::Down],
        '-' if direction.is_vertical() => vec![Direction::Left, Direction::Right],
        '-' if !direction.is_vertical() => vec![direction],
        '/' if direction == Direction::Up => vec![Direction::Right],
        '/' if direction == Direction::Down => vec![Direction::Left],
        '/' if direction == Direction::Left => vec![Direction::Down],
        '/' if direction == Direction::Right => vec![Direction::Up],
        '\\' if direction == Direction::Up => vec![Direction::Left],
        '\\' if direction == Direction::Down => vec![Direction::Right],
        '\\' if direction == Direction::Left => vec![Direction::Up],
        '\\' if direction == Direction::Right => vec![Direction::Down],
        _ => vec![direction],
    }
}

fn get_next_pos_and_directions(
    mirror: char,
    direction: Direction,
    pos: Position,
    map: &Vec<Vec<char>>,
) -> Vec<(Position, Direction)> {
    get_next_directions(mirror, direction)
        .into_iter()
        .filter_map(|d| {
            let next_pos = pos.next_pos(d);
            if next_pos.y < 0 || next_pos.y >= map.len() as i32 {
                None
            } else if next_pos.x < 0 || next_pos.x >= map[0].len() as i32 {
                None
            } else {
                Some((next_pos, d))
            }
        })
        .collect()
}

fn compute_energized(map: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut energized = vec![vec![[false; 4]; map[0].len()]; map.len()];
    let mut worklist = vec![(Position { y: 0, x: 0 }, Direction::Right)];

    while let Some((pos, direction)) = worklist.pop() {
        let dir_index = direction.get_index();
        if energized[pos.y as usize][pos.x as usize][dir_index] {
            continue;
        } else {
            energized[pos.y as usize][pos.x as usize][dir_index] = true;
        }
        let next_pos =
            get_next_pos_and_directions(map[pos.y as usize][pos.x as usize], direction, pos, map);
        worklist.extend(next_pos.into_iter());
    }

    energized
        .into_iter()
        .map(|row| row.into_iter().map(|v| v.iter().any(|&b| b)).collect())
        .collect()
}

fn main() {
    let file = get_map(&read_file());
    let energized = compute_energized(&file);
    println!("{}", energized.iter().flatten().filter(|&&b| b).count());
}
