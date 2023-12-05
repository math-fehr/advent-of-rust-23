const FILENAME: &'static str = "day2/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn get_rgb_values(round: &str) -> (u32, u32, u32) {
    let rgb = round.split(',');
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for value in rgb {
        let mut number_name = value.trim().split(" ");
        let number: u32 = number_name.nth(0).unwrap().trim().parse().unwrap();
        let name = number_name.nth(0).unwrap().trim();
        if name == "red" {
            r += number;
        } else if name == "green" {
            g += number;
        } else if name == "blue" {
            b += number;
        } else {
            unreachable!("Invalid color name");
        }
    }
    (r, g, b)
}

fn get_game_value(game: &str) -> u32 {
    let games = game.split(':').nth(1).unwrap();
    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;

    for round in games.split(';') {
        let (r, g, b) = get_rgb_values(round);
        max_r = max_r.max(r);
        max_g = max_g.max(g);
        max_b = max_b.max(b);
    }
    max_r * max_g * max_b
}

fn main() {
    let mut sum = 0;
    for line in read_file().lines() {
        sum += get_game_value(line);
    }
    println!("{}", sum);
}
