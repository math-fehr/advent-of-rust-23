use std::collections::HashMap;

const FILENAME: &'static str = "day19/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn variable_to_index(variable: char) -> i8 {
    match variable {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("Invalid variable"),
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule<'a> {
    variable: i8,
    less_operator: bool,
    value: i32,
    exit_name: &'a str,
}

type ADomain = [(i32, i32); 4];

impl<'a> Rule<'a> {
    fn run(&self, parts: &ADomain) -> (Option<(&str, ADomain)>, Option<ADomain>) {
        let (min_v, max_v) = parts[self.variable as usize];
        if self.less_operator {
            if max_v < self.value {
                (Some((self.exit_name, parts.clone())), None)
            } else if min_v < self.value {
                let mut exit_parts = parts.clone();
                exit_parts[self.variable as usize] = (min_v, self.value - 1);
                let mut stay_parts = parts.clone();
                stay_parts[self.variable as usize] = (self.value, max_v);
                (Some((self.exit_name, exit_parts)), Some(stay_parts))
            } else {
                (None, Some(parts.clone()))
            }
        } else {
            if min_v > self.value {
                (Some((self.exit_name, parts.clone())), None)
            } else if max_v > self.value {
                let mut exit_parts = parts.clone();
                exit_parts[self.variable as usize] = (self.value + 1, max_v);
                let mut stay_parts = parts.clone();
                stay_parts[self.variable as usize] = (min_v, self.value);
                (Some((self.exit_name, exit_parts)), Some(stay_parts))
            } else {
                (None, Some(parts.clone()))
            }
        }
    }
}

fn read_rule(input: &str) -> Rule {
    let variable = variable_to_index(input.chars().nth(0).unwrap());
    let less_operator = input.chars().nth(1).unwrap() == '<';
    let value = input.split(':').nth(0).unwrap()[2..]
        .parse::<i32>()
        .unwrap();
    let exit_name = input.split(':').nth(1).unwrap();
    Rule {
        variable,
        less_operator,
        value,
        exit_name,
    }
}

#[derive(Debug, Clone)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    default_exit: &'a str,
}

impl<'a> Workflow<'a> {
    fn run(&self, parts: &ADomain) -> Vec<(&str, ADomain)> {
        let mut res = Vec::new();
        let mut current_parts = parts.clone();
        for rule in &self.rules {
            let (exit_parts, continue_parts) = rule.run(&current_parts);
            if let Some(exit_parts) = exit_parts {
                res.push(exit_parts);
            }
            if continue_parts.is_none() {
                return res;
            }
            current_parts = continue_parts.unwrap();
        }
        res.push((self.default_exit, current_parts));
        res
    }
}

fn read_workflow(input: &str) -> (&str, Workflow) {
    let name = input.split('{').nth(0).unwrap();
    let all_rules: Vec<_> = input
        .split('{')
        .nth(1)
        .unwrap()
        .split('}')
        .nth(0)
        .unwrap()
        .split(',')
        .collect();
    let rules = all_rules[..all_rules.len() - 1]
        .iter()
        .map(|rule| read_rule(rule))
        .collect();
    let default_exit = all_rules[all_rules.len() - 1];
    (
        name,
        Workflow {
            rules,
            default_exit,
        },
    )
}

fn read_workflows(input: &str) -> Vec<(&str, Workflow)> {
    input.split('\n').map(|line| read_workflow(line)).collect()
}

fn read_parts(input: &str) -> [i32; 4] {
    input
        .split('=')
        .skip(1)
        .map(|part| {
            part.split(',')
                .nth(0)
                .unwrap()
                .split('}')
                .nth(0)
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap()
}

fn read_input(input: &str) -> (Vec<(&str, Workflow)>, Vec<[i32; 4]>) {
    let mut it = input.split("\n\n");
    let workflows = read_workflows(it.next().unwrap());
    let parts = it
        .next()
        .unwrap()
        .lines()
        .map(|parts| read_parts(parts))
        .collect();
    (workflows, parts)
}

fn main() {
    let time = std::time::Instant::now();
    let file = read_file();
    let (workflows, _) = read_input(&file);
    let mut workflows_map = HashMap::new();
    for (name, workflow) in workflows {
        workflows_map.insert(name, workflow);
    }

    let mut worklist = Vec::new();
    worklist.push(("in", [(1, 4000), (1, 4000), (1, 4000), (1, 4000)]));

    let mut res = 0;

    while let Some((node, parts)) = worklist.pop() {
        if node == "R" {
            continue;
        }
        if node == "A" {
            res += parts
                .iter()
                .map(|(min, max)| (max - min + 1) as i64)
                .product::<i64>();
            continue;
        }
        let mut new_nodes = workflows_map.get(node).unwrap().run(&parts);
        worklist.append(&mut new_nodes);
    }

    println!("{}", res);
}
