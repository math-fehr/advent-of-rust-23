#![feature(extract_if)]

use std::collections::LinkedList;

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

fn do_instruction<'a>(instruction: &'a str, state: &mut Vec<LinkedList<(&'a str, u8)>>) {
    if instruction.as_bytes()[instruction.len() - 2] == '=' as u8 {
        let hash = get_hash(&instruction[0..instruction.len() - 2]);
        let val = instruction.as_bytes()[instruction.len() - 1] - '0' as u8;
        let linked_list = &mut state[hash as usize];
        let elem = linked_list
            .iter_mut()
            .find(|x| *x.0 == instruction[0..instruction.len() - 2]);
        if let Some(elem) = elem {
            elem.1 = val;
        } else {
            linked_list.push_back((&instruction[0..instruction.len() - 2], val));
        }
    } else {
        let hash = get_hash(&instruction[0..instruction.len() - 1]);
        let linked_list = &mut state[hash as usize];
        linked_list
            .extract_if(|x| *x.0 == instruction[0..instruction.len() - 1])
            .next();
    }
}

fn main() {
    let mut state: Vec<LinkedList<(&str, u8)>> = vec![LinkedList::new(); 256];
    let file = read_file();
    file.split(",").for_each(|x| do_instruction(x, &mut state));
    let mut res = 0;
    for (box_idx, box_) in state.iter().enumerate() {
        for (lens_index, (_, lens_val)) in box_.iter().enumerate() {
            res += (box_idx as i64 + 1) * (*lens_val as i64) * (lens_index + 1) as i64;
        }
    }
    println!("{}", res);
}
