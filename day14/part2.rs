use std::collections::HashMap;

const FILENAME: &'static str = "day14/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Object {
    None,
    Round,
    Cube,
}

fn get_map(input: &str) -> Vec<Vec<Object>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| match x {
                    'O' => Object::Round,
                    '#' => Object::Cube,
                    _ => Object::None,
                })
                .collect()
        })
        .collect()
}

fn move_rocks_north(map: &mut Vec<Vec<Object>>) {
    for j in 0..map[0].len() {
        let mut num_rocks = 0;
        for i in (0..map.len()).rev() {
            match map[i][j] {
                Object::Round => {
                    num_rocks += 1;
                }
                Object::Cube => {
                    num_rocks = 0;
                }
                Object::None => {
                    if num_rocks > 0 {
                        map[i][j] = Object::Round;
                        map[i + num_rocks][j] = Object::None;
                    }
                }
            }
        }
    }
}

fn move_rocks_west(map: &mut Vec<Vec<Object>>) {
    for i in 0..map.len() {
        let mut num_rocks = 0;
        for j in (0..map[0].len()).rev() {
            match map[i][j] {
                Object::Round => {
                    num_rocks += 1;
                }
                Object::Cube => {
                    num_rocks = 0;
                }
                Object::None => {
                    if num_rocks > 0 {
                        map[i][j] = Object::Round;
                        map[i][j + num_rocks] = Object::None;
                    }
                }
            }
        }
    }
}

fn move_rocks_south(map: &mut Vec<Vec<Object>>) {
    for j in 0..map[0].len() {
        let mut num_rocks = 0;
        for i in 0..map.len() {
            match map[i][j] {
                Object::Round => {
                    num_rocks += 1;
                }
                Object::Cube => {
                    num_rocks = 0;
                }
                Object::None => {
                    if num_rocks > 0 {
                        map[i][j] = Object::Round;
                        map[i - num_rocks][j] = Object::None;
                    }
                }
            }
        }
    }
}

fn move_rocks_east(map: &mut Vec<Vec<Object>>) {
    for i in 0..map.len() {
        let mut num_rocks = 0;
        for j in 0..map[0].len() {
            match map[i][j] {
                Object::Round => {
                    num_rocks += 1;
                }
                Object::Cube => {
                    num_rocks = 0;
                }
                Object::None => {
                    if num_rocks > 0 {
                        map[i][j] = Object::Round;
                        map[i][j - num_rocks] = Object::None;
                    }
                }
            }
        }
    }
}

fn get_value(map: &Vec<Vec<Object>>) -> i32 {
    let mut value = 0;
    for (i, line) in map.iter().enumerate() {
        for object in line {
            if *object == Object::Round {
                value += (map.len() - i) as i32;
            }
        }
    }
    value
}

fn move_one_cycle(map: &mut Vec<Vec<Object>>) {
    move_rocks_north(map);
    move_rocks_west(map);
    move_rocks_south(map);
    move_rocks_east(map);
}

fn main() {
    let mut map = get_map(&read_file());
    let mut memory = HashMap::new();
    memory.insert(map.clone(), 0);
    for i in 1..=1000000000 {
        move_one_cycle(&mut map);
        let value = *memory.entry(map.clone()).or_insert(i);
        if value != i {
            let cycle_length = i - value;
            let remaining_cycles = (1000000000 - i) % cycle_length;
            for _ in 0..remaining_cycles {
                move_one_cycle(&mut map);
            }
            break;
        }
    }
    println!("{}", get_value(&map));
}
