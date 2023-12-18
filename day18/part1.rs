use itertools::Itertools;

const FILENAME: &'static str = "day18/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn read_plan(input: &str) -> Vec<(char, i32, &str)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let action = it.next().unwrap().chars().next().unwrap();
            let value = it.next().unwrap().parse::<i32>().unwrap();
            let rest = it.next().unwrap();
            (action, value, rest)
        })
        .collect()
}

fn move_position(y: i32, x: i32, direction: char, value: i32) -> (i32, i32) {
    match direction {
        'U' => (y - value, x),
        'D' => (y + value, x),
        'R' => (y, x + value),
        'L' => (y, x - value),
        _ => panic!("Invalid direction"),
    }
}

fn get_max_height_width(plan: &Vec<(char, i32, &str)>) -> (i32, i32, i32, i32) {
    let mut min_height = 0;
    let mut max_height = 0;
    let mut min_width = 0;
    let mut max_width = 0;
    let mut y = 0;
    let mut x = 0;
    for value in plan {
        (y, x) = move_position(y, x, value.0, value.1);
        min_height = min_height.min(y);
        max_height = max_height.max(y);
        min_width = min_width.min(x);
        max_width = max_width.max(x);
    }
    (min_height, max_height, min_width, max_width)
}

fn draw_map(plan: &Vec<(char, i32, &str)>) -> Vec<Vec<bool>> {
    let (min_height, max_height, min_width, max_width) = get_max_height_width(&plan);
    let height = (max_height - min_height + 1) as usize;
    let width = (max_width - min_width + 1) as usize;
    let mut map = vec![vec![false; width]; height];
    let mut y = -min_height;
    let mut x = -min_width;
    for (direction, value, _) in plan {
        for _ in 0..*value {
            (y, x) = move_position(y, x, *direction, 1);
            map[(y) as usize][(x) as usize] = true;
        }
    }
    map
}

fn walk_inside(y: usize, x: usize, plan: &Vec<Vec<bool>>, outside: &mut Vec<Vec<bool>>) {
    if plan[y][x] || outside[y][x] {
        return;
    }
    outside[y][x] = true;
    let height = plan.len();
    let width = plan[0].len();
    if y > 0 {
        walk_inside(y - 1, x, plan, outside);
    }
    if y < height - 1 {
        walk_inside(y + 1, x, plan, outside);
    }
    if x > 0 {
        walk_inside(y, x - 1, plan, outside);
    }
    if x < width - 1 {
        walk_inside(y, x + 1, plan, outside);
    }
}

fn num_inside(plan: &Vec<Vec<bool>>) -> i32 {
    let height = plan.len();
    let width = plan[0].len();
    let mut outside = vec![vec![false; width]; height];
    for y in 0..height {
        walk_inside(y, 0, plan, &mut outside);
        walk_inside(y, width - 1, plan, &mut outside);
    }
    for x in 0..width {
        walk_inside(0, x, plan, &mut outside);
        walk_inside(height - 1, x, plan, &mut outside);
    }
    outside.iter().flatten().filter(|&&x| !x).count() as i32
}

fn main() {
    let input = read_file();
    let plan = read_plan(&input);
    let map = draw_map(&plan);
    let num = num_inside(&map);
    println!("{}", num);
}
