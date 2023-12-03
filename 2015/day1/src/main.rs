use std::fs;

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let mut floor = 0;

    for ch in puzzle_input.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }

    println!("Part 1: {}", floor);
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let mut floor = 0;

    for (i, ch) in puzzle_input.char_indices() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor < 0 {
            println!("Part 2: {}", i + 1);
            break;
        }
    }
}

fn main() {
    part1();
    part2();
}
