const FILENAME: &'static str = "day2/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn get_game_id(game: &str) -> u32 {
    let prefix = game.split(':').nth(0).unwrap();
    prefix[5..].parse().unwrap()
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
    let game_id = get_game_id(game);

    let games = game.split(':').nth(1).unwrap();
    for round in games.split(';') {
        let (r, g, b) = get_rgb_values(round);
        if r > 12 || g > 13 || b > 14 {
            return 0;
        }
    }
    game_id
}

fn main() {
    let mut sum = 0;
    for line in read_file().lines() {
        sum += get_game_value(line);
    }
    println!("{}", sum);
}
