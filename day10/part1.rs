const FILENAME: &'static str = "day10/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

type Map = Vec<Vec<char>>;

fn read_map(map: &str) -> Map {
    map.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
    fn get_pipe(&self, map: &Map) -> char {
        map[self.y][self.x]
    }

    fn neighbors(&self, map: &Map) -> [Pos; 2] {
        match self.get_pipe(map) {
            '|' => [Pos::new(self.y - 1, self.x), Pos::new(self.y + 1, self.x)],
            '-' => [Pos::new(self.y, self.x - 1), Pos::new(self.y, self.x + 1)],
            'L' => [Pos::new(self.y - 1, self.x), Pos::new(self.y, self.x + 1)],
            'J' => [Pos::new(self.y - 1, self.x), Pos::new(self.y, self.x - 1)],
            '7' => [Pos::new(self.y + 1, self.x), Pos::new(self.y, self.x - 1)],
            'F' => [Pos::new(self.y + 1, self.x), Pos::new(self.y, self.x + 1)],
            _ => panic!("Invalid pipe '{}'", self.get_pipe(map)),
        }
    }

    fn starting_pos_neighbors(&self, map: &Map) -> [Pos; 2] {
        let mut neighbors = Vec::new();
        if self.y != 0 {
            neighbors.push(Pos::new(self.y - 1, self.x));
        }
        if self.y != map.len() - 1 {
            neighbors.push(Pos::new(self.y + 1, self.x));
        }
        if self.x != 0 {
            neighbors.push(Pos::new(self.y, self.x - 1));
        }
        if self.x != map[0].len() - 1 {
            neighbors.push(Pos::new(self.y, self.x + 1));
        }
        let mut res = Vec::new();
        for neighbor in neighbors {
            if neighbor.get_pipe(map) == '.' {
                continue;
            }
            if neighbor.neighbors(map).contains(self) {
                res.push(neighbor);
            }
        }
        assert!(res.len() == 2);
        [res[0], res[1]]
    }
}

fn starting_position(map: &Map) -> Pos {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                return Pos::new(y, x);
            }
        }
    }
    panic!("No starting position found");
}

fn main() {
    let map = read_map(&read_file());
    let starting_pos = starting_position(&map);
    let [neigh1, neigh2] = starting_pos.starting_pos_neighbors(&map);

    let mut cycle = Vec::new();
    let mut last = starting_pos;
    let mut current = neigh1;
    while current != neigh2 {
        cycle.push(current);
        let [current_neigh1, current_neigh2] = current.neighbors(&map);
        if current_neigh1 == last {
            last = current;
            current = current_neigh2;
        } else {
            last = current;
            current = current_neigh1;
        }
    }
    cycle.push(neigh2);
    println!("{:?}", ((cycle.len() + 1) / 2));
}
