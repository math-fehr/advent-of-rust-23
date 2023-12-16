const FILENAME: &'static str = "day15/part1.in";

fn read_file() -> String {
    std::fs::read_to_string(FILENAME).expect("Something went wrong reading the file")
}

fn get_hash(input: &str) -> u8 {
    let mut val: u8 = 0;
    for c in input.chars() {
        val += c as u8;
        val *= 17;
    }
    val
}

fn main() {
    let res = read_file()
        .split(",")
        .map(|x| get_hash(x) as i64)
        .sum::<i64>();
    println!("{}", res);
}
