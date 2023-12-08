use std::collections::HashMap;

use itertools::Itertools;

const FILENAME: &'static str = "day8/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn parse_line(line: &str) -> (&str, &str, &str) {
    let (a, _, b, c) = line.split_whitespace().collect_tuple().unwrap();
    (a, &b[1..b.len() - 1], &c[..c.len() - 1])
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

fn parse_instructions(file: &str) -> Vec<Direction> {
    file.lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| {
            if (c == 'L') {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Graph {
    start_node: i32,
    end_node: i32,
    instructions: Vec<Direction>,
    nodes: Vec<(i32, i32)>,
}

fn parse_graph(file: &str) -> Graph {
    let instructions = parse_instructions(file);
    let nodes: Vec<(&str, &str, &str)> = file.lines().skip(2).map(parse_line).collect();
    let names_to_node: HashMap<_, _> = nodes
        .iter()
        .enumerate()
        .map(|(i, (k, _, _))| (*k, i as i32))
        .collect();
    Graph {
        start_node: names_to_node["AAA"],
        end_node: names_to_node["ZZZ"],
        instructions,
        nodes: nodes
            .iter()
            .map(|(k, l, r)| (names_to_node[*l], names_to_node[*r]))
            .collect(),
    }
}

fn main() {
    let graph = parse_graph(&read_file());
    let mut current_node = graph.start_node;
    let mut current_instruction = 0;
    while current_node != graph.end_node {
        let (left, right) = graph.nodes[current_node as usize];
        let direction =
            &graph.instructions[(current_instruction as usize) % graph.instructions.len()];
        current_node = match direction {
            Direction::Left => left,
            Direction::Right => right,
        };
        current_instruction += 1;
    }
    println!("{}", current_instruction);
}
