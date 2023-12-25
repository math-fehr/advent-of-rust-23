use std::collections::{HashMap, HashSet};

const FILENAME: &'static str = "day25/part1.in";

fn read_input() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn read_file(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut res = HashMap::new();
    for line in input.lines() {
        let val_left = line.split(":").next().unwrap().trim();
        for val_right in line
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.trim())
        {
            res.entry(val_left).or_insert(vec![]).push(val_right);
            res.entry(val_right).or_insert(vec![]).push(val_left);
        }
    }
    res
}

fn str_graph_to_int_graph(graph: &HashMap<&str, Vec<&str>>) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    let mut keys = graph.keys().collect::<Vec<_>>();
    keys.sort();
    let key_to_int = keys
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect::<HashMap<_, _>>();

    for key in keys.into_iter() {
        let val_int = graph[key].iter().map(|x| key_to_int[x]).collect();
        res.push(val_int);
    }

    res
}

fn bfs(graph: &Vec<Vec<(usize, i32)>>, start: usize, end: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut queue = Vec::new();
    let mut parent = vec![usize::MAX; graph.len()];
    queue.push(start);
    visited[start] = true;
    parent[start] = start;

    while let Some(u) = queue.pop() {
        for (v, val) in graph[u].iter() {
            if !visited[*v] && *val > 0 {
                visited[*v] = true;
                parent[*v] = u;
                queue.push(*v);
            }
        }
    }

    parent
}

fn fold_fulkerson(
    graph: &Vec<Vec<usize>>,
    start: usize,
    end: usize,
) -> (i32, Vec<Vec<(usize, i32)>>) {
    let mut residual: Vec<Vec<(usize, i32)>> = graph
        .iter()
        .map(|v| v.iter().map(|x| (*x, 1)).collect())
        .collect();
    let mut max_flow = 0;
    loop {
        let parents = bfs(&residual, start, end);
        if parents[end] == usize::MAX {
            break;
        }
        max_flow += 1;
        let mut v = end;
        while v != start {
            let u = parents[v];
            for (i, (x, _)) in residual[u].iter().enumerate() {
                if *x == v {
                    residual[u][i].1 -= 1;
                    break;
                }
            }
            for (i, (x, _)) in residual[v].iter().enumerate() {
                if *x == u {
                    residual[v][i].1 += 1;
                    break;
                }
            }
            v = u;
        }
    }
    (max_flow, residual)
}

fn main() {
    let input = read_input();
    let graph = read_file(&input);
    let graph = str_graph_to_int_graph(&graph);
    for i in 1..graph.len() {
        let (flow, residual) = fold_fulkerson(&graph, 0, i);
        if flow != 3 {
            continue;
        }
        let parents = bfs(&residual, 0, i);
        let component_size = parents.iter().filter(|x| x != &&usize::MAX).count();
        println!("{}", component_size * (graph.len() - component_size));
        break;
    }
}
