const FILENAME: &'static str = "day24/part1.in";

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: f64,
    y: f64,
    z: f64,
}

impl FromIterator<f64> for Coord {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Coord {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
        }
    }
}

impl Coord {
    fn is_intersection_in_bounds(
        p: Coord,
        v: Coord,
        pp: Coord,
        vp: Coord,
        min_val: f64,
        max_val: f64,
    ) -> bool {
        // y = ax + c
        // y' = bx' + d
        let a = v.y / v.x;
        let c = p.y - p.x * v.y / v.x;
        let b = vp.y / vp.x;
        let d = pp.y - pp.x * vp.y / vp.x;

        let x_intersect = (d - c) / (a - b);
        let y_intersect = a * x_intersect + c;

        // Intersect in the past
        if (p.x - x_intersect) * v.x > 0.0 {
            return false;
        }
        if (pp.x - x_intersect) * vp.x > 0.0 {
            return false;
        }

        x_intersect >= min_val
            && x_intersect <= max_val
            && y_intersect >= min_val
            && y_intersect <= max_val
    }
}

fn read_coord(input: &str) -> Coord {
    input
        .split(',')
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect()
}

fn read_coords(input: &str) -> (Coord, Coord) {
    let mut iter = input.split('@');
    let id = iter.next().unwrap();
    let coords = iter.next().unwrap();
    (read_coord(id), read_coord(coords))
}

fn read_map() -> Vec<(Coord, Coord)> {
    let file = std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    file.lines().map(|x| read_coords(x)).collect()
}

fn main() {
    let map = read_map();
    let mut res = 0;
    for i in 0..map.len() {
        for j in i..map.len() {
            let (pos1, v1) = map[i];
            let (pos2, v2) = map[j];
            if Coord::is_intersection_in_bounds(
                pos1,
                v1,
                pos2,
                v2,
                200000000000000.0,
                400000000000000.0,
            ) {
                res += 1;
                //println!("{:?} and {:?} intersect", pos1, pos2);
            }
        }
    }
    println!("{}", res);
}
