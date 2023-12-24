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

type Matrix = Vec<Vec<f64>>;

fn inverse(matrix: &mut Matrix) {
    let n = matrix.len();
    let mut identity = vec![vec![0.0; n]; n];
    for i in 0..n {
        identity[i][i] = 1.0;
    }
    for i in 0..n {
        let mut max = matrix[i][i];
        let mut max_index = i;
        for j in i + 1..n {
            if matrix[j][i] > max {
                max = matrix[j][i];
                max_index = j;
            }
        }
        if max_index != i {
            matrix.swap(i, max_index);
            identity.swap(i, max_index);
        }
        let pivot = matrix[i][i];
        for j in i..n {
            matrix[i][j] /= pivot;
        }
        for j in 0..n {
            identity[i][j] /= pivot;
        }
        for j in 0..n {
            if j != i {
                let factor = matrix[j][i];
                for k in i..n {
                    matrix[j][k] -= factor * matrix[i][k];
                }
                for k in 0..n {
                    identity[j][k] -= factor * identity[i][k];
                }
            }
        }
    }
    *matrix = identity;
}

fn matvec(matrix: &Matrix, vec: &Vec<f64>) -> Vec<f64> {
    let n = matrix.len();
    let mut res = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            res[i] += matrix[i][j] * vec[j];
        }
    }
    res
}

// x_init + vx * t0 = x1_init + vx1 * t0
// y_init + vy * t0 = y1_init + vy1 * t0

// x_init + vx * t1 = x2_init + vx2 * t1
// y_init + vy * t1 = y2_init + vy2 * t1

// x * (vy0 - vy1) + vy * (x0 - x1) - y * (vx0 - vx1) - vx * (y0 - x0) - x0 * vy0 + x1 * vy1 + y0 * vx0 - y1 * vx1 = 0

fn main() {
    let map = read_map();
    let mut matrix = vec![];
    let mut b = vec![];

    for i in 0..4 {
        let mut line = vec![];
        let (Coord { x: x0, y: y0, .. }, Coord { x: vx0, y: vy0, .. }) = map[i];
        let (Coord { x: x1, y: y1, .. }, Coord { x: vx1, y: vy1, .. }) = map[i + 1];
        // Values for x, vy, y, vx
        line.push(vy0 - vy1);
        line.push(x0 - x1);
        line.push(-(vx0 - vx1));
        line.push(-(y0 - y1));

        matrix.push(line);
        b.push(x0 * vy0 - x1 * vy1 - y0 * vx0 + y1 * vx1);
    }

    inverse(&mut matrix);
    let res = matvec(&matrix, &b);
    let x = (res[0] + 0.5) as i64;
    let y = (res[2] + 0.5) as i64;

    matrix = vec![];
    b = vec![];
    for i in 0..4 {
        let mut line = vec![];
        let (Coord { x: x0, z: y0, .. }, Coord { x: vx0, z: vy0, .. }) = map[i];
        let (Coord { x: x1, z: y1, .. }, Coord { x: vx1, z: vy1, .. }) = map[i + 1];
        // Values for x, vy, y, vx
        line.push(vy0 - vy1);
        line.push(x0 - x1);
        line.push(-(vx0 - vx1));
        line.push(-(y0 - y1));

        matrix.push(line);
        b.push(x0 * vy0 - x1 * vy1 - y0 * vx0 + y1 * vx1);
    }

    inverse(&mut matrix);
    let res = matvec(&matrix, &b);
    let z = (res[2] + 0.5) as i64;
    println!("{}", x + y + z);
}
