const FILENAME: &'static str = "day22/part1.in";

#[derive(Debug, Clone, Copy)]
struct Brick {
    x_min: i32,
    y_min: i32,
    z_min: i32,
    x_max: i32,
    y_max: i32,
    z_max: i32,
}

impl Brick {
    fn move_to_z_min(&mut self, new_z_min: i32) {
        self.z_max = new_z_min + (self.z_max - self.z_min);
        self.z_min = new_z_min;
    }

    fn is_intersecting(&self, block: &Brick) -> bool {
        let intersect_x = self.x_min <= block.x_max && self.x_max >= block.x_min;
        let intersect_y = self.y_min <= block.y_max && self.y_max >= block.y_min;
        intersect_x && intersect_y
    }

    fn fall_on(&self, block: &Brick) -> i32 {
        if self.is_intersecting(block) {
            block.z_max + 1
        } else {
            1
        }
    }

    fn is_supporting(&self, block: &Brick) -> bool {
        self.is_intersecting(block) && self.z_max + 1 == block.z_min
    }
}

fn read_file() -> Vec<Brick> {
    let file = std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    let mut res = vec![];
    for line in file.lines() {
        let values: Vec<_> = line
            .split(|x| x == ',' || x == '~')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let brick = Brick {
            x_min: values[0].min(values[3]),
            y_min: values[1].min(values[4]),
            z_min: values[2].min(values[5]),
            x_max: values[0].max(values[3]),
            y_max: values[1].max(values[4]),
            z_max: values[2].max(values[5]),
        };
        res.push(brick);
    }
    res
}

fn main() {
    let mut bricks = read_file();
    bricks.sort_by(|a, b| a.z_min.cmp(&b.z_min));

    // Make the bricks fall one by one
    for i in 0..bricks.len() {
        let mut max_min_z = 1;
        for j in 0..i {
            let new_min_z = bricks[i].fall_on(&bricks[j]);
            max_min_z = max_min_z.max(new_min_z);
        }
        bricks[i].move_to_z_min(max_min_z);
    }

    let mut supporting = vec![false; bricks.len()];
    for i in 0..bricks.len() {
        let mut supporting_block = None;
        for j in 0..bricks.len() {
            if bricks[j].is_supporting(&bricks[i]) {
                if supporting_block.is_some() {
                    supporting_block = None;
                    break;
                }
                supporting_block = Some(j);
            }
        }
        if let Some(j) = supporting_block {
            supporting[j] = true;
        }
    }
    println!("{:?}", supporting.iter().filter(|x| !**x).count());
}
