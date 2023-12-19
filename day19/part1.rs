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

impl<'a> Rule<'a> {
    fn run(&self, parts: &[i32; 4]) -> Option<&str> {
        let variable = parts[self.variable as usize];
        if (self.less_operator && variable < self.value)
            || (!self.less_operator && variable > self.value)
        {
            Some(self.exit_name)
        } else {
            None
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
    fn run(&self, parts: &[i32; 4]) -> &str {
        for rule in &self.rules {
            if let Some(exit_name) = rule.run(parts) {
                return exit_name;
            }
        }
        self.default_exit
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
    let file = read_file();
    let (workflows, parts) = read_input(&file);
    let mut workflows_map = HashMap::new();
    for (name, workflow) in workflows {
        workflows_map.insert(name, workflow);
    }

    let mut res = 0;

    for part in parts {
        let mut node = "in";
        while node != "A" && node != "R" {
            node = workflows_map.get(node).unwrap().run(&part);
        }
        if node == "A" {
            res += part.iter().sum::<i32>();
        }
    }

    println!("{}", res);
}
