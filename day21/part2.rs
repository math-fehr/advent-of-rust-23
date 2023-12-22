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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum XPositions {
    Left,
    Right,
    Mid,
}

impl XPositions {
    fn to_i64(&self, n: i64) -> i64 {
        match self {
            XPositions::Left => 0,
            XPositions::Right => n - 1,
            XPositions::Mid => n / 2,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum YPositions {
    Top,
    Bottom,
    Mid,
}

impl YPositions {
    fn to_i64(&self, n: i64) -> i64 {
        match self {
            YPositions::Top => 0,
            YPositions::Bottom => n - 1,
            YPositions::Mid => n / 2,
        }
    }
}

fn try_with(map: &Vec<Vec<bool>>, steps: i64) {
    let n = map.len() as i64;
    let multiplier = (steps / n) * 2 + 1;
    let mut new_map: Vec<Vec<bool>> = Vec::new();
    for _ in 0..multiplier {
        for line in map {
            let mut new_line = Vec::new();
            for _ in 0..multiplier {
                new_line.extend(line.clone());
            }
            new_map.push(new_line);
        }
    }
    let map = new_map;

    let n = (n * multiplier) as usize;

    let distances = get_distances(n / 2, n / 2, &map);

    let num_places = distances
        .iter()
        .flatten()
        .filter(|x| x.is_some() && x.unwrap() % 2 == steps % 2 && x.unwrap() <= steps)
        .count();
    println!("manual: {}", num_places);
}

fn main() {
    let (y, x, map) = read_file();
    let n = map.len() as i64;
    let half_n = n / 2;
    let steps = 26501365;
    assert_eq!(map.len(), map[0].len());

    //try_with(&map, steps);

    let mut res = 0;

    let mut distances_from = HashMap::new();
    for ypos in vec![YPositions::Top, YPositions::Bottom, YPositions::Mid] {
        for xpos in vec![XPositions::Left, XPositions::Right, XPositions::Mid] {
            let y = ypos.to_i64(n);
            let x = xpos.to_i64(n);
            distances_from.insert((ypos, xpos), get_distances(y as usize, x as usize, &map));
        }
    }

    let mut max_distance_from = HashMap::new();
    for (pos, distances) in &distances_from {
        max_distance_from.insert(
            *pos,
            distances.iter().flatten().filter_map(|x| *x).max().unwrap(),
        );
    }
    println!("max_distance: {:?}", max_distance_from);

    let mut border_steps = 0;

    let starting_distance_from_mid = (steps - half_n - 1) % n;
    dbg!(starting_distance_from_mid);

    for pos in vec![
        (YPositions::Top, XPositions::Mid),
        (YPositions::Bottom, XPositions::Mid),
        (YPositions::Mid, XPositions::Left),
        (YPositions::Mid, XPositions::Right),
    ] {
        border_steps += distances_from[&pos]
            .iter()
            .flatten()
            .filter_map(|x| *x)
            .filter(|x| {
                *x <= starting_distance_from_mid && *x % 2 == starting_distance_from_mid % 2
            })
            .count() as i64;
    }

    let starting_distance_from_corner = (steps - half_n - half_n - 2) % n;
    let starting_distance_from_corner_larger = starting_distance_from_corner + n;
    dbg!(starting_distance_from_corner);
    dbg!(starting_distance_from_corner_larger);

    let mut angle_border = 0;
    let mut angle_border_larger = 0;
    for pos in vec![
        (YPositions::Top, XPositions::Left),
        (YPositions::Top, XPositions::Right),
        (YPositions::Bottom, XPositions::Left),
        (YPositions::Bottom, XPositions::Right),
    ] {
        angle_border += distances_from[&pos]
            .iter()
            .flatten()
            .filter_map(|x| *x)
            .filter(|x| {
                *x <= starting_distance_from_corner && *x % 2 == starting_distance_from_corner % 2
            })
            .count() as i64;
        angle_border_larger += distances_from[&pos]
            .iter()
            .flatten()
            .filter_map(|x| *x)
            .filter(|x| {
                *x <= starting_distance_from_corner_larger
                    && *x % 2 == starting_distance_from_corner_larger % 2
            })
            .count() as i64;
    }

    let n_angle_blocks_larger = (steps - half_n - half_n - 2) / n;
    let n_angle_blocks = n_angle_blocks_larger + 1;

    border_steps += angle_border * n_angle_blocks;
    border_steps += angle_border_larger * n_angle_blocks_larger;

    let mut n_odd = 0;
    let mut n_even = 0;

    let distance_odd = distances_from[&(YPositions::Mid, XPositions::Left)]
        .iter()
        .flatten()
        .filter_map(|x| *x)
        .filter(|x| *x % 2 == 1)
        .count() as i64;
    let distance_even = distances_from[&(YPositions::Mid, XPositions::Left)]
        .iter()
        .flatten()
        .filter_map(|x| *x)
        .filter(|x| *x % 2 == 0)
        .count() as i64;
    let distance_mid = distances_from[&(YPositions::Mid, XPositions::Mid)]
        .iter()
        .flatten()
        .filter_map(|x| *x)
        .filter(|x| *x % 2 == steps % 2)
        .count() as i64;

    println!("distance_odd: {}", distance_odd);
    println!("distance_even: {}", distance_even);

    for i in 1..n_angle_blocks {
        if i % 2 == 0 {
            n_even += i * 4;
        } else {
            n_odd += i * 4;
        }
    }

    println!("n_odd: {}", n_odd);
    println!("n_even: {}", n_even);

    res += border_steps;
    res += distance_odd * n_odd;
    res += distance_even * n_even;
    res += distance_mid;

    println!("res: {}", res);
}
