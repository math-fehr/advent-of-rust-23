const FILENAME: &'static str = "day11/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    y: i64,
    x: i64,
}

fn read_map(map: &str) -> Vec<Pos> {
    map.lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(x, c)| *c == '#')
                .map(move |(x, _)| Pos {
                    y: y as i64,
                    x: x as i64,
                })
        })
        .flatten()
        .collect()
}

fn main() {
    let mut map = read_map(&read_file());
    let min_x = map.iter().map(|pos| pos.x).min().unwrap();
    let min_y = map.iter().map(|pos| pos.y).min().unwrap();
    let mut max_x = map.iter().map(|pos| pos.x).max().unwrap();
    let mut max_y = map.iter().map(|pos| pos.y).max().unwrap();

    let galaxy_size = 1000000 - 1;

    let mut x = min_x + 1;
    while x < max_x {
        if map.iter().any(|pos| pos.x == x) {
            x += 1;
            continue;
        }
        map.iter_mut()
            .filter(|pos| pos.x > x)
            .for_each(|pos| pos.x += galaxy_size);
        x += galaxy_size + 1;
        max_x += galaxy_size;
    }

    let mut y = min_y + 1;
    while y < max_y {
        if map.iter().any(|pos| pos.y == y) {
            y += 1;
            continue;
        }
        map.iter_mut()
            .filter(|pos| pos.y > y)
            .for_each(|pos| pos.y += galaxy_size);
        y += galaxy_size + 1;
        max_y += galaxy_size;
    }

    let mut res = 0;
    for pos_1 in map.iter() {
        for pos_2 in map.iter() {
            res += (pos_1.y - pos_2.y).abs() + (pos_1.x - pos_2.x).abs();
        }
    }
    println!("{}", res / 2);
}
