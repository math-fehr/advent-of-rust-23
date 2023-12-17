use std::collections::{BinaryHeap, HashMap, HashSet};

const FILENAME: &'static str = "day17/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn read_map(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as i8 - '0' as i8).collect())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    y: i32,
    x: i32,
    vertical: bool,
}

fn neighbors(pos: &Position, map: &Vec<Vec<i8>>) -> Vec<(Position, i32)> {
    let mut result = Vec::new();
    let max_distance: i32 = 3;
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    if pos.vertical {
        let mut distance = 0;
        for y in (pos.y + 1)..=(pos.y + max_distance).min(height - 1) {
            distance += map[y as usize][pos.x as usize] as i32;
            result.push((
                Position {
                    y,
                    x: pos.x,
                    vertical: false,
                },
                distance,
            ));
        }
        distance = 0;
        for y in ((pos.y - max_distance).max(0)..=(pos.y - 1)).rev() {
            distance += map[y as usize][pos.x as usize] as i32;
            result.push((
                Position {
                    y,
                    x: pos.x,
                    vertical: false,
                },
                distance,
            ));
        }
    } else {
        let mut distance = 0;
        for x in (pos.x + 1)..=(pos.x + max_distance).min(width - 1) {
            distance += map[pos.y as usize][x as usize] as i32;
            result.push((
                Position {
                    y: pos.y,
                    x,
                    vertical: true,
                },
                distance,
            ));
        }
        distance = 0;
        for x in ((pos.x - max_distance).max(0)..=(pos.x - 1)).rev() {
            distance += map[pos.y as usize][x as usize] as i32;
            result.push((
                Position {
                    y: pos.y,
                    x,
                    vertical: true,
                },
                distance,
            ));
        }
    }
    result
}

#[derive(Debug, Hash, Clone, Copy)]
struct DistPos {
    distance: i32,
    position: Position,
}

impl PartialEq for DistPos {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for DistPos {}

impl PartialOrd for DistPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance).reverse())
    }
}

impl Ord for DistPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn djikstra(map: &Vec<Vec<i8>>) -> i32 {
    let height = map.len();
    let width = map[0].len();
    let mut distances = vec![vec![vec![i32::MAX; 2]; width]; height];
    let mut queue = BinaryHeap::new();
    queue.push(DistPos {
        distance: 0,
        position: Position {
            y: 0,
            x: 0,
            vertical: false,
        },
    });
    queue.push(DistPos {
        distance: 0,
        position: Position {
            y: 0,
            x: 0,
            vertical: true,
        },
    });

    while let Some(DistPos {
        position: u,
        distance: u_dist,
    }) = queue.pop()
    {
        if u.y == (height - 1) as i32 && u.x == (width - 1) as i32 {
            return u_dist;
        }
        if u_dist > distances[u.y as usize][u.x as usize][u.vertical as usize] {
            continue;
        }
        for (v, uv_dist) in neighbors(&u, map) {
            let alt = u_dist + uv_dist;
            if alt < distances[v.y as usize][v.x as usize][v.vertical as usize] {
                distances[v.y as usize][v.x as usize][v.vertical as usize] = alt;
                queue.push(DistPos {
                    distance: alt,
                    position: v,
                });
            }
        }
    }
    unreachable!()
}

fn main() {
    let map = read_map(&read_file());
    println!("{}", djikstra(&map));
}
