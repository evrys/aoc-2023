use std::collections::HashSet;
use std::fs;

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let mut visited = HashSet::<(i32, i32)>::new();
    let mut x = 0;
    let mut y = 0;

    for ch in puzzle_input.chars() {
        visited.insert((x, y));

        match ch {
            '^' => y -= 1,
            'v' => y += 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => panic!("Invalid character: {}", ch),
        }
    }

    println!("Part 1: {}", visited.len());
}

struct Mover {
    x: i32,
    y: i32,
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let mut santa = Mover { x: 0, y: 0 };
    let mut robot = Mover { x: 0, y: 0 };

    let mut visited = HashSet::<(i32, i32)>::new();
    let mut robot_turn = false;

    for ch in puzzle_input.chars() {
        let mover = if robot_turn { &mut robot } else { &mut santa };
        visited.insert((mover.x, mover.y));

        match ch {
            '^' => mover.y -= 1,
            'v' => mover.y += 1,
            '>' => mover.x += 1,
            '<' => mover.x -= 1,
            _ => panic!("Invalid character: {}", ch),
        }

        robot_turn = !robot_turn;
    }

    println!("Part 2: {}", visited.len());
}

fn main() {
    part1();
    part2();
}
