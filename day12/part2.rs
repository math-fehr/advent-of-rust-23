use std::collections::HashMap;

const FILENAME: &'static str = "day12/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn read_list_springs(springs: &str) -> Vec<Spring> {
    let springs: Vec<_> = springs
        .chars()
        .map(|c| match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => unreachable!(),
        })
        .collect();
    let mut res = Vec::new();
    res.append(&mut springs.clone());
    res.push(Spring::Unknown);
    res.append(&mut springs.clone());
    res.push(Spring::Unknown);
    res.append(&mut springs.clone());
    res.push(Spring::Unknown);
    res.append(&mut springs.clone());
    res.push(Spring::Unknown);
    res.append(&mut springs.clone());
    res
}

fn read_problem(problem: &str) -> (Vec<Spring>, Vec<i32>) {
    let springs = read_list_springs(problem.split_whitespace().nth(0).unwrap());
    let dimensions: Vec<_> = problem
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut res = Vec::new();
    res.append(&mut dimensions.clone());
    res.append(&mut dimensions.clone());
    res.append(&mut dimensions.clone());
    res.append(&mut dimensions.clone());
    res.append(&mut dimensions.clone());

    (springs, res)
}

fn read_problems(problems: &str) -> Vec<(Vec<Spring>, Vec<i32>)> {
    problems.lines().map(|line| read_problem(line)).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn solve(
    springs: &mut Vec<Spring>,
    mut spring_index: usize,
    indices: &mut Vec<i32>,
    mut index_index: usize,
    memoization: &mut std::collections::HashMap<(usize, usize), i64>,
) -> i64 {
    loop {
        // Out of bounds in springs
        if spring_index == springs.len() {
            return (index_index == indices.len()) as i64;
        }

        // No indices left -> optimization
        if index_index == indices.len() {
            return springs[spring_index..]
                .iter()
                .all(|s| *s != Spring::Damaged) as i64;
        }

        // Operational spring
        if springs[spring_index] == Spring::Operational {
            spring_index += 1;
            continue;
        }

        // Damaged spring
        if springs[spring_index] == Spring::Damaged {
            let num_damaged = indices[index_index];
            if spring_index + num_damaged as usize > springs.len() {
                return 0;
            }
            if springs[spring_index..spring_index + num_damaged as usize]
                .iter()
                .any(|s| *s == Spring::Operational)
            {
                return 0;
            }
            spring_index += num_damaged as usize;
            index_index += 1;
            if spring_index == springs.len() {
                return (index_index == indices.len()) as i64;
            }
            if springs[spring_index] == Spring::Damaged {
                return 0;
            }
            spring_index += 1;
            continue;
        }

        // Unknown spring
        if springs[spring_index] == Spring::Unknown {
            if memoization.contains_key(&(spring_index, index_index)) {
                return memoization[&(spring_index, index_index)];
            }
            let mut res = 0;
            springs[spring_index] = Spring::Operational;
            res += solve(springs, spring_index, indices, index_index, memoization);
            springs[spring_index] = Spring::Damaged;
            res += solve(springs, spring_index, indices, index_index, memoization);
            springs[spring_index] = Spring::Unknown;
            memoization.insert((spring_index, index_index), res);
            return res;
        }
    }
}

fn main() {
    // Time the file read:
    let now = std::time::Instant::now();
    let problems = read_problems(&read_file());
    let elapsed_parse = now.elapsed();

    let mut res = 0;
    let now = std::time::Instant::now();
    for (springs, indices) in problems {
        let mut springs = springs;
        let mut indices = indices;
        res += solve(&mut springs, 0, &mut indices, 0, &mut HashMap::new());
    }

    let elapsed = now.elapsed();
    println!("{} {:?} {:?}", res, elapsed_parse, elapsed);
}
