const FILENAME: &'static str = "day14/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

fn move_rocks(map: &mut Vec<Vec<Object>>) {
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

fn main() {
    let mut map = get_map(&read_file());
    move_rocks(&mut map);
    println!("{}", get_value(&map));
}
