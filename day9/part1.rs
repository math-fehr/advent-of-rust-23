const FILENAME: &'static str = "day9/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn read_problems(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn derivative(v: &Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 1..v.len() {
        res.push(v[i] - v[i - 1]);
    }
    res
}

fn compute_problem(v: &Vec<i32>) -> i32 {
    let mut derivatives = Vec::new();
    derivatives.push(v.clone());
    while !derivatives.last().unwrap().iter().all(|&x| x == 0) {
        derivatives.push(derivative(&derivatives.last().unwrap()));
    }

    derivatives.last_mut().unwrap().push(0);
    for i in (0..(derivatives.len() - 1)).rev() {
        let val1 = *derivatives[i + 1].last().unwrap();
        let val2 = *derivatives[i].last().unwrap();
        derivatives[i].push(val1 + val2);
    }

    *derivatives[0].last().unwrap()
}

fn main() {
    let problem = read_problems(&read_file());
    println!(
        "{:?}",
        problem.iter().map(|v| compute_problem(&v)).sum::<i32>()
    );
}
