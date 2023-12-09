use std::collections::{HashMap, VecDeque};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ExtendedNode {
    position: i32,
    instruction: i32,
}

#[derive(Debug, Clone)]
struct Graph {
    start_nodes: Vec<i32>,
    end_nodes: Vec<i32>,
    instructions: Vec<Direction>,
    nodes: Vec<(i32, i32)>,
}

impl Graph {
    fn get_next_node(&self, extended_node: ExtendedNode) -> ExtendedNode {
        let node = extended_node.position;
        let instruction = extended_node.instruction;
        let (left, right) = self.nodes[node as usize];
        let next_instruction: i32 = (instruction + 1) % (self.instructions.len() as i32);
        let position = match self.instructions[instruction as usize] {
            Direction::Left => left,
            Direction::Right => right,
        };
        ExtendedNode {
            position,
            instruction: next_instruction,
        }
    }

    fn get_cycle_for_node(&self, starting_node: i32) -> (i64, i64, Vec<i64>) {
        let mut current_node = ExtendedNode {
            position: starting_node,
            instruction: 0,
        };
        let mut visited = HashMap::new();
        visited.insert(current_node.clone(), 0);
        let mut cycle = vec![current_node];
        loop {
            let next_node = self.get_next_node(current_node);
            if visited.contains_key(&next_node) {
                break;
            }
            visited.insert(next_node, cycle.len() as i32);
            cycle.push(next_node);
            current_node = next_node;
        }
        let next_node = self.get_next_node(current_node);
        let offset = visited[&next_node];

        let mut new_cycle = Vec::new();
        for node in cycle.iter().skip(offset as usize) {
            if self.end_nodes.contains(&node.position) {
                new_cycle.push(visited[node] as i64);
            }
        }

        (
            offset as i64,
            (cycle.len() - offset as usize) as i64,
            new_cycle,
        )
    }

    fn get_cycles(&self) -> Vec<(i64, i64, Vec<i64>)> {
        let mut cycles = vec![];
        for node in self.start_nodes.iter() {
            let cycle = self.get_cycle_for_node(*node);
            cycles.push(cycle);
        }
        cycles
    }

    fn is_end_node(&self, node: ExtendedNode) -> bool {
        self.end_nodes.contains(&node.position)
    }
}

fn parse_graph(file: &str) -> Graph {
    let instructions = parse_instructions(file);
    let nodes: Vec<(&str, &str, &str)> = file.lines().skip(2).map(parse_line).collect();
    let names_to_node: HashMap<_, _> = nodes
        .iter()
        .enumerate()
        .map(|(i, (k, _, _))| (*k, i as i32))
        .collect();
    let start_nodes = nodes
        .iter()
        .filter(|(n, _, _)| n.chars().nth(2).unwrap() == 'A')
        .map(|(k, _, _)| names_to_node[*k])
        .collect();
    let end_nodes = nodes
        .iter()
        .filter(|(n, _, _)| n.chars().nth(2).unwrap() == 'Z')
        .map(|(k, _, _)| names_to_node[*k])
        .collect();
    Graph {
        start_nodes,
        end_nodes,
        instructions,
        nodes: nodes
            .iter()
            .map(|(k, l, r)| (names_to_node[*l], names_to_node[*r]))
            .collect(),
    }
}

fn is_finished(state: &Vec<(i64, VecDeque<i64>)>) -> bool {
    let min = state.iter().map(|v| v.1[0]).min().unwrap();
    let n = state.iter().filter(|v| v.1[0] == min).count();
    if n > 4 {
        println!("{} {}", n, min);
    }
    state.iter().map(|v| v.1[0]).all_equal()
}

fn next_stop(state: &mut Vec<(i64, VecDeque<i64>)>) {
    let maxi = state.iter().map(|v| v.1[0]).max().unwrap();
    for (size, v) in state.iter_mut() {
        while v[0] < maxi {
            let removed = v.pop_front().unwrap();
            v.push_back(removed + *size);
        }
    }
}

/// extended_gcd(x, y) = gcd(x, y), u, v such that ux + vy = gcd(x, y)
fn extended_gcd(x: i128, y: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_r, old_s, old_t)
}

fn main() {
    let graph = parse_graph(&read_file());

    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        let cycles = graph.get_cycles();
        let mut dequeue_cycles: Vec<_> = cycles
            .iter()
            .map(|(_, size, cycle)| (*size, VecDeque::from(cycle.clone())))
            .collect();

        while !is_finished(&dequeue_cycles) {
            next_stop(&mut dequeue_cycles);
            // println!("{}", dequeue_cycles[0].1[0]);
        }
        println!("{:?}", dequeue_cycles);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
