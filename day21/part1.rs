use std::collections::{HashMap, VecDeque};

const FILENAME: &'static str = "day21/part1.in";

fn read_file() -> (usize, usize, Vec<Vec<bool>>) {
    let file = std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    let mut y = 0;
    let mut x = 0;
    let mut res = Vec::new();
    for (line_idx, line) in file.lines().enumerate() {
        let mut line_res = Vec::new();
        for (c_idx, c) in line.chars().enumerate() {
            match c {
                '.' => line_res.push(false),
                '#' => line_res.push(true),
                'S' => {
                    line_res.push(false);
                    y = line_idx;
                    x = c_idx;
                }
                _ => panic!("Incorrect input {:?}", c),
            }
        }
        res.push(line_res);
    }
    (y, x, res)
}

fn get_distances(y: usize, x: usize, map: &Vec<Vec<bool>>) -> Vec<Vec<Option<i64>>> {
    let mut res = vec![vec![None; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    queue.push_back((y, x, 0));
    while let Some((y, x, dist)) = queue.pop_front() {
        if res[y][x].is_some() {
            continue;
        }
        res[y][x] = Some(dist);
        if y > 0 && !map[y - 1][x] {
            queue.push_back((y - 1, x, dist + 1));
        }
        if y < map.len() - 1 && !map[y + 1][x] {
            queue.push_back((y + 1, x, dist + 1));
        }
        if x > 0 && !map[y][x - 1] {
            queue.push_back((y, x - 1, dist + 1));
        }
        if x < map[0].len() - 1 && !map[y][x + 1] {
            queue.push_back((y, x + 1, dist + 1));
        }
    }
    res
}

fn main() {
    let (y, x, map) = read_file();
    let distances = get_distances(y, x, &map);

    let n = 64;
    let num_places = distances
        .iter()
        .flatten()
        .filter(|x| x.is_some() && x.unwrap() % 2 == n % 2 && x.unwrap() <= n)
        .count();
    println!("{}", num_places);
}
