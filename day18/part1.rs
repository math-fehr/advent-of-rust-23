use itertools::Itertools;

const FILENAME: &'static str = "day18/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn read_plan(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let action = it.next().unwrap().chars().next().unwrap();
            let value = it.next().unwrap().parse::<i32>().unwrap();
            (action, value)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Pos {
    y: i32,
    x: i32,
}

impl Pos {
    fn move_to(&self, direction: char, value: i32) -> Pos {
        match direction {
            'U' => Pos {
                y: self.y - value,
                x: self.x,
            },
            'D' => Pos {
                y: self.y + value,
                x: self.x,
            },
            'R' => Pos {
                y: self.y,
                x: self.x + value,
            },
            'L' => Pos {
                y: self.y,
                x: self.x - value,
            },
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct HorizontalLine {
    y: i32,
    x1: i32,
    x2: i32,
    same_endpoints_dir: bool,
}

impl HorizontalLine {
    fn intersect(&self, y: i32) -> bool {
        y == self.y
    }
}

#[derive(Debug, Clone, Copy)]
struct VerticalLine {
    x: i32,
    y1: i32,
    y2: i32,
    same_endpoints_dir: bool,
}

impl VerticalLine {
    fn intersect(&self, y: i32) -> bool {
        self.y1 < y && y < self.y2
    }
}

#[derive(Debug, Clone, Copy)]
enum Line {
    Horizontal(HorizontalLine),
    Vertical(VerticalLine),
}

fn get_vertical_horizontal_lines(plan: &Vec<(char, i32)>) -> Vec<Line> {
    let mut pos = Pos { y: 0, x: 0 };
    let mut lines = Vec::new();

    for i in 0..plan.len() {
        let (direction, value) = plan[i];
        let previous_direction = plan[(i + plan.len() - 1) % plan.len()].0;
        let next_direction = plan[(i + 1) % plan.len()].0;
        let same_endpoints_dir = previous_direction != next_direction;
        let new_pos = pos.move_to(direction, value);
        match direction {
            'U' | 'D' => {
                let x = pos.x;
                let y1 = std::cmp::min(pos.y, new_pos.y);
                let y2 = std::cmp::max(pos.y, new_pos.y);
                lines.push(Line::Vertical(VerticalLine {
                    x,
                    y1,
                    y2,
                    same_endpoints_dir,
                }));
            }
            'R' | 'L' => {
                let y = pos.y;
                let x1 = std::cmp::min(pos.x, new_pos.x);
                let x2 = std::cmp::max(pos.x, new_pos.x);
                lines.push(Line::Horizontal(HorizontalLine {
                    y,
                    x1,
                    x2,
                    same_endpoints_dir,
                }));
            }
            _ => panic!("Invalid direction"),
        }
        pos = new_pos;
    }
    lines
}

#[derive(Debug, Clone, Copy)]
struct InterestingVerticalRange {
    y: i32,
    distance: i32,
}

fn get_interesting_vertical_points(lines: &Vec<Line>) -> Vec<InterestingVerticalRange> {
    let mut points = Vec::new();
    lines
        .iter()
        .filter_map(|line| match line {
            Line::Vertical(v) => Some(v),
            _ => None,
        })
        .for_each(|line| {
            points.push(line.y1);
            points.push(line.y1 + 1);
            points.push(line.y2);
            points.push(line.y2 + 1);
        });

    points.sort();
    points.dedup();

    let mut interesting_points = Vec::new();
    for i in 0..points.len() - 1 {
        let y = points[i];
        let next_y = points[i + 1];
        interesting_points.push(InterestingVerticalRange {
            y,
            distance: next_y - y,
        });
    }
    interesting_points
}

fn num_inside_values(range: InterestingVerticalRange, lines: &Vec<Line>) -> i64 {
    let y = range.y;
    let mut last_intersection = 0;
    let mut inside = false;
    let mut num_inside = 0;
    for line in lines {
        match line {
            Line::Horizontal(h) => {
                if h.intersect(y) {
                    if h.same_endpoints_dir && !inside {
                        num_inside += (h.x2 - h.x1 + 1) as i64;
                    }
                    if !h.same_endpoints_dir {
                        if inside {
                            num_inside += (h.x2 - last_intersection + 1) as i64;
                        } else {
                            last_intersection = h.x1;
                        }
                        inside = !inside;
                    }
                }
            }
            Line::Vertical(v) => {
                if v.intersect(y) {
                    if inside {
                        num_inside += (v.x - last_intersection + 1) as i64;
                    }
                    inside = !inside;
                    last_intersection = v.x;
                }
            }
        }
    }
    num_inside * range.distance as i64
}

fn main() {
    let input = read_file();
    let plan = read_plan(&input);
    let mut lines = get_vertical_horizontal_lines(&plan);
    let interesting_vertical_range = get_interesting_vertical_points(&lines);

    lines.sort_by(|x, y| match (x, y) {
        (Line::Horizontal(h1), Line::Horizontal(h2)) => h1.x1.cmp(&h2.x1),
        (Line::Vertical(v1), Line::Vertical(v2)) => v1.x.cmp(&v2.x),
        (Line::Horizontal(h), Line::Vertical(v)) => h.x1.cmp(&v.x),
        (Line::Vertical(v), Line::Horizontal(h)) => v.x.cmp(&h.x1),
    });

    let mut res = 0;
    for range in interesting_vertical_range {
        res += num_inside_values(range, &lines);
    }
    println!("{}", res);
}
