use std::collections::{HashMap, HashSet, VecDeque};

const FILENAME: &'static str = "day20/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module<'a> {
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
    module_type: ModuleType,
}

#[derive(Debug)]
struct Program<'a> {
    inputs: Vec<&'a str>,
    modules: HashMap<&'a str, Module<'a>>,
}

fn read_module(input: &str) -> (&str, Module) {
    let name = input.split("->").nth(0).unwrap().trim();
    let type_ = if name.chars().nth(0).unwrap() == '%' {
        ModuleType::FlipFlop
    } else {
        ModuleType::Conjunction
    };
    let name = &name[1..];
    let outputs = input
        .split("->")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.trim())
        .collect();
    let module = Module {
        inputs: Vec::new(),
        outputs,
        module_type: type_,
    };
    (name, module)
}

fn read_input(input: &str) -> Program {
    let first_line = input.lines().nth(0).unwrap();
    let program_inputs = first_line
        .split("->")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.trim())
        .collect();

    let mut modules = input
        .lines()
        .skip(1)
        .map(|x| read_module(x))
        .collect::<HashMap<&str, Module>>();

    let mut inputs = HashMap::new();
    for (module_name, module) in modules.iter() {
        for output in module.outputs.iter() {
            inputs.entry(*output).or_insert(vec![]).push(*module_name);
        }
    }

    for (module_name, input) in inputs {
        if let Some(module) = modules.get_mut(module_name) {
            module.inputs = input;
        }
    }

    Program {
        inputs: program_inputs,
        modules,
    }
}

#[derive(Debug)]
struct FlipFlopState {
    is_on: bool,
}

impl FlipFlopState {
    fn new() -> FlipFlopState {
        FlipFlopState { is_on: false }
    }

    fn get_pulse(&mut self, pulse: bool) -> Option<bool> {
        if pulse {
            return None;
        }
        self.is_on = !self.is_on;
        Some(self.is_on)
    }
}

#[derive(Debug)]
struct ConjunctionState<'a> {
    inputs: HashMap<&'a str, bool>,
}

impl<'a> ConjunctionState<'a> {
    fn new(module: &'a Module) -> ConjunctionState<'a> {
        let inputs = module.inputs.iter().map(|x| (*x, false)).collect();
        ConjunctionState { inputs }
    }

    fn get_pulse(&mut self, input: &str, pulse: bool) -> bool {
        *self.inputs.get_mut(input).unwrap() = pulse;
        self.inputs.values().any(|x| !*x)
    }
}

#[derive(Debug)]
enum ModuleState<'a> {
    FlipFlop(FlipFlopState),
    Conjunction(ConjunctionState<'a>),
}

impl<'a> ModuleState<'a> {
    fn new(module: &'a Module) -> ModuleState<'a> {
        match module.module_type {
            ModuleType::FlipFlop => ModuleState::FlipFlop(FlipFlopState::new()),
            ModuleType::Conjunction => ModuleState::Conjunction(ConjunctionState::new(module)),
        }
    }

    fn get_pulse(&mut self, input: &str, pulse: bool) -> Option<bool> {
        match self {
            ModuleState::FlipFlop(state) => state.get_pulse(pulse),
            ModuleState::Conjunction(state) => Some(state.get_pulse(input, pulse)),
        }
    }
}

#[derive(Debug)]
struct ProgramState<'a> {
    modules: HashMap<&'a str, ModuleState<'a>>,
}

impl<'a> ProgramState<'a> {
    fn new(program: &'a Program) -> ProgramState<'a> {
        let modules = program
            .modules
            .iter()
            .map(|(name, module)| (*name, ModuleState::new(module)))
            .collect();
        ProgramState { modules }
    }

    fn get_signature(&self, nodes: &Vec<&str>) -> i64 {
        let mut num_values = 0;
        let mut signature = 0;
        for node in nodes {
            let module = &self.modules[node];
            match module {
                ModuleState::FlipFlop(state) => {
                    signature = (signature << 1) + state.is_on as i64;
                    num_values += 1;
                }
                ModuleState::Conjunction(state) => {
                    let mut sorted_inputs = state.inputs.iter().collect::<Vec<_>>();
                    sorted_inputs.sort_by(|a, b| a.0.cmp(b.0));
                    for (_, value) in sorted_inputs {
                        signature = (signature << 1) + *value as i64;
                        num_values += 1;
                    }
                }
            }
        }
        assert!(num_values <= 64);
        signature
    }
}

fn tarjan_scc<'a>(
    program: &'a Program,
    node: &'a str,
    components: &mut Vec<Vec<&'a str>>,
    indices: &mut HashMap<&'a str, usize>,
    lowlink: &mut HashMap<&'a str, usize>,
    on_stack: &mut HashSet<&'a str>,
    stack: &mut Vec<&'a str>,
    index: &mut usize,
) {
    indices.insert(node, *index);
    lowlink.insert(node, *index);
    *index += 1;
    stack.push(node);
    on_stack.insert(node);

    for output in program.modules[node].outputs.iter() {
        if !program.modules.contains_key(output) {
            continue;
        }
        if indices.get(output).is_none() {
            tarjan_scc(
                program, output, components, indices, lowlink, on_stack, stack, index,
            );
            let lowlink_node = lowlink.get(node).unwrap();
            let lowlink_output = lowlink.get(output).unwrap();
            if lowlink_output < lowlink_node {
                lowlink.insert(node, *lowlink_output);
            }
        } else if on_stack.contains(output) {
            let lowlink_node = lowlink.get(node).unwrap();
            let index_output = indices.get(output).unwrap();
            if index_output < lowlink_node {
                lowlink.insert(node, *index_output);
            }
        }
    }

    if lowlink.get(node) == indices.get(node) {
        let mut component = Vec::new();
        loop {
            let output = stack.pop().unwrap();
            on_stack.remove(output);
            component.push(output);
            if output == node {
                break;
            }
        }
        components.push(component);
    }
}

fn tarjan<'a>(program: &'a Program) -> Vec<Vec<&'a str>> {
    let mut index = 0;
    let mut components = Vec::new();
    let mut indices = HashMap::new();
    let mut on_stack = HashSet::new();
    let mut lowlink = HashMap::new();
    let mut stack = Vec::<&str>::new();

    for module in program.modules.keys().cloned() {
        if !indices.contains_key(module) {
            tarjan_scc(
                program,
                module,
                &mut components,
                &mut indices,
                &mut lowlink,
                &mut on_stack,
                &mut stack,
                &mut index,
            );
        }
    }
    components
}

fn run_once<'a>(
    program: &'a Program,
    mut pulses: VecDeque<(&'a str, bool, &'a str)>,
    state: &mut ProgramState,
    component: &Vec<&'a str>,
) -> Vec<(&'a str, bool, &'a str)> {
    let mut res = Vec::new();
    while let Some((input, pulse, output)) = pulses.pop_front() {
        if !component.contains(&output) {
            res.push((input, pulse, output));
            continue;
        }
        if let Some(module) = program.modules.get(output) {
            let new_pulse = state
                .modules
                .get_mut(output)
                .unwrap()
                .get_pulse(input, pulse);
            if let Some(new_pulse) = new_pulse {
                for new_output in module.outputs.iter() {
                    pulses.push_back((output, new_pulse, *new_output));
                }
            }
        }
    }
    res
}

fn get_component_cycle<'a>(
    program: &'a Program,
    components: &Vec<Vec<&'a str>>,
    input: &'a str,
) -> (i64, Vec<(i64, Vec<(&'a str, bool, &'a str)>)>) {
    let component = components.iter().find(|x| x.contains(&input)).unwrap();
    let mut n_button_pressed = 0;
    let mut res = Vec::new();
    let mut state = ProgramState::new(&program);
    let mut signatures = HashSet::new();
    signatures.insert(state.get_signature(&component));

    loop {
        let mut pulses = VecDeque::new();
        pulses.push_back(("broadcaster", false, input));
        let outside_pulses = run_once(&program, pulses, &mut state, component);
        let signature = state.get_signature(&component);
        if outside_pulses.len() != 0 {
            res.push((n_button_pressed, outside_pulses));
        }
        n_button_pressed += 1;
        if signatures.contains(&signature) {
            assert!(signature == 0);
            return (n_button_pressed, res);
        }
        signatures.insert(signature);
    }
}

fn main() {
    let input = read_file();
    let program = read_input(&input);
    let components = tarjan(&program);

    let component = components.iter().find(|x| x.contains(&"lf")).unwrap();
    println!("{:?}", component);

    // I'm not gonna write a generic solution for a problem that is not expecting one.
    let cycle1 = get_component_cycle(&program, &components, "lf").0;
    let cycle2 = get_component_cycle(&program, &components, "mc").0;
    let cycle3 = get_component_cycle(&program, &components, "fx").0;
    let cycle4 = get_component_cycle(&program, &components, "nd").0;

    println!("{}", cycle1 * cycle2 * cycle3 * cycle4);
}
