use std::collections::{HashMap, HashSet};

const FILENAME: &'static str = "day23/part1.in";

fn read_map() -> Vec<Vec<char>> {
    let file = std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    file.lines().map(|x| x.chars().collect()).collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn get_neighbors(&self, map: &Vec<Vec<char>>) -> Vec<Pos> {
        let mut res = vec![];
        if self.y > 0 {
            res.push(Pos {
                y: self.y - 1,
                x: self.x,
            });
        }
        if self.y < map.len() - 1 {
            res.push(Pos {
                y: self.y + 1,
                x: self.x,
            });
        }
        if self.x > 0 {
            res.push(Pos {
                y: self.y,
                x: self.x - 1,
            });
        }
        if self.x < map[self.y].len() - 1 {
            res.push(Pos {
                y: self.y,
                x: self.x + 1,
            });
        }
        res
    }

    fn get_walkable_neighbors(&self, map: &Vec<Vec<char>>) -> Vec<Pos> {
        match map[self.y][self.x] {
            '#' => vec![],
            _ => self
                .get_neighbors(map)
                .into_iter()
                .filter(|p| map[p.y][p.x] != '#')
                .collect(),
        }
    }
}

fn is_intersection(pos: Pos, map: &Vec<Vec<char>>) -> bool {
    if map[pos.y][pos.x] == '#' {
        return false;
    }
    pos.get_walkable_neighbors(map).len() > 2
}

fn get_start(map: &Vec<Vec<char>>) -> Pos {
    for x in 0..map[0].len() {
        if map[0][x] != '#' {
            return Pos { y: 0, x };
        }
    }
    panic!("No start found");
}

fn get_end(map: &Vec<Vec<char>>) -> Pos {
    for x in 0..map[map.len() - 1].len() {
        if map[map.len() - 1][x] != '#' {
            return Pos {
                y: map.len() - 1,
                x,
            };
        }
    }
    panic!("No end found");
}

fn get_intersection_nodes(map: &Vec<Vec<char>>) -> HashSet<Pos> {
    let mut res = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if is_intersection(Pos { y, x }, map) {
                res.insert(Pos { y, x });
            }
        }
    }
    res.insert(get_start(map));
    res.insert(get_end(map));
    res
}

fn walk_path(
    map: &Vec<Vec<char>>,
    intersections: &HashSet<Pos>,
    start: Pos,
    mut previous_pos: Pos,
) -> Option<(Pos, i32)> {
    let mut pos = start;
    let mut distance = 1;
    while !intersections.contains(&pos) {
        let neighbors: Vec<_> = pos
            .get_walkable_neighbors(map)
            .into_iter()
            .filter(|p| *p != previous_pos)
            .collect();
        if neighbors.len() == 0 {
            return None;
        }
        assert_eq!(neighbors.len(), 1);
        previous_pos = pos;
        distance += 1;
        pos = neighbors[0];
    }
    Some((pos, distance))
}

fn walk_paths(map: &Vec<Vec<char>>, intersections: &HashSet<Pos>, start: Pos) -> Vec<(Pos, i32)> {
    let mut res = vec![];
    for neighbor in start.get_walkable_neighbors(map) {
        if let Some((pos, distance)) = walk_path(map, &intersections, neighbor, start) {
            res.push((pos, distance));
        }
    }
    res
}

fn get_edges(map: &Vec<Vec<char>>, intersections: &HashSet<Pos>) -> HashMap<Pos, Vec<(Pos, i32)>> {
    let mut res = HashMap::new();
    for intersection in intersections {
        res.insert(*intersection, walk_paths(map, intersections, *intersection));
    }
    res
}

fn find_longest_path(
    intersections: &HashSet<Pos>,
    edges: &HashMap<Pos, Vec<(Pos, i32)>>,
    pos: Pos,
    ending_pos: Pos,
    visited: &mut HashSet<Pos>,
) -> Option<i32> {
    visited.insert(pos);
    let mut max_distance = None;
    for (neighbor, distance) in &edges[&pos] {
        if visited.contains(neighbor) {
            continue;
        }
        if neighbor == &ending_pos {
            max_distance = max_distance.max(Some(*distance));
            continue;
        }
        let end_path = find_longest_path(intersections, edges, *neighbor, ending_pos, visited);
        if let Some(end_path) = end_path {
            max_distance = max_distance.max(Some(distance + end_path));
        }
    }
    visited.remove(&pos);
    max_distance
}

fn main() {
    let map = read_map();
    let intersection_nodes = get_intersection_nodes(&map);
    let edges = get_edges(&map, &intersection_nodes);
    let longest_path = find_longest_path(
        &intersection_nodes,
        &edges,
        get_start(&map),
        get_end(&map),
        &mut HashSet::new(),
    );
    println!("{:?}", longest_path);
}
