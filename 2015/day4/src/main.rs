use md5;
use std::collections::HashSet;
use std::fs;

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let mut i = 1;

    loop {
        let key = format!("{}{}", puzzle_input, i);
        let hash = md5::compute(key);
        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with("00000") {
            println!("Part 1: {}", i);
            break;
        }
        i += 1;
    }
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let mut i = 1;

    loop {
        let key = format!("{}{}", puzzle_input, i);
        let hash = md5::compute(key);
        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with("000000") {
            println!("Part 2: {}", i);
            break;
        }
        i += 1;
    }
}

fn main() {
    part1();
    part2();
}
