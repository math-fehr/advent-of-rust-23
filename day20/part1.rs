use std::collections::{HashMap, VecDeque};

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
}

fn run_once(program: &Program, state: &mut ProgramState) -> (i32, i32) {
    let mut n_low_pulse = 1; // The button press
    let mut n_high_pulse = 0;

    let mut pulses: VecDeque<_> = program
        .inputs
        .iter()
        .map(|x| ("broadcaster", false, *x))
        .collect();
    while let Some((input, pulse, output)) = pulses.pop_front() {
        if pulse {
            n_high_pulse += 1;
        } else {
            n_low_pulse += 1;
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
    (n_low_pulse, n_high_pulse)
}

fn main() {
    let input = read_file();
    let program = read_input(&input);
    let mut state = ProgramState::new(&program);
    let mut n_low_pulses = 0;
    let mut n_high_pulses = 0;
    for _ in 0..1000 {
        let (n_low, n_high) = run_once(&program, &mut state);
        n_low_pulses += n_low as i64;
        n_high_pulses += n_high as i64;
    }
    println!("{}", n_low_pulses * n_high_pulses);
}
