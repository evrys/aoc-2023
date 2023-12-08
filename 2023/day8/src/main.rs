#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{collections::HashMap, fs};

const _TEST_INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;

    let mut iter = puzzle_input.lines();

    let directions = iter.next().unwrap();
    iter.next();

    let mut nodes = HashMap::<&str, Node>::new();
    for line in iter {
        let (name, rest) = line.split_once(" = ").unwrap();
        let (left, right) = rest.split_once(", ").unwrap();
        let left = left.trim_matches('(');
        let right = right.trim_matches(')');

        nodes.insert(name, Node { name, left, right });
    }

    let mut dirindex = 0;
    let mut current = &nodes["AAA"];
    let mut steps = 0;

    while current.name != "ZZZ" {
        let dir = directions.chars().nth(dirindex).unwrap();

        if dir == 'L' {
            current = &nodes[current.left];
        } else {
            current = &nodes[current.right];
        }

        steps += 1;
        dirindex += 1;
        if dirindex >= directions.len() {
            dirindex = 0;
        }
    }

    println!("Part 1: {}", steps);
}

fn lcm(first: i128, second: i128) -> i128 {
    first * second / gcd(first, second)
}

fn gcd(first: i128, second: i128) -> i128 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;

    let mut iter = puzzle_input.lines();

    let directions = iter.next().unwrap();
    iter.next();

    let mut nodes = HashMap::<&str, Node>::new();
    for line in iter {
        let (name, rest) = line.split_once(" = ").unwrap();
        let (left, right) = rest.split_once(", ").unwrap();
        let left = left.trim_matches('(');
        let right = right.trim_matches(')');

        nodes.insert(name, Node { name, left, right });
    }

    let mut dirindex = 0;
    let mut steps = 0;

    let mut current = nodes
        .values()
        .filter(|n| n.name.ends_with("A"))
        .collect::<Vec<_>>();

    let mut steps_to_z = current.iter().map(|n| 0).collect::<Vec<_>>();

    loop {
        steps += 1;

        let dir = directions.chars().nth(dirindex).unwrap();

        for i in 0..current.len() {
            if dir == 'L' {
                current[i] = &nodes[current[i].left];
            } else {
                current[i] = &nodes[current[i].right];
            }

            let n = current[i];

            if n.name.ends_with("Z") && steps_to_z[i] == 0 {
                steps_to_z[i] = steps;

                if steps_to_z.iter().all(|v| *v != 0) {
                    let result = steps_to_z.into_iter().reduce(|a, b| lcm(a, b)).unwrap();
                    println!("Part 2: {}", result);
                    return;
                }
            }
        }

        dirindex += 1;
        if dirindex >= directions.len() {
            dirindex = 0;
        }
    }
}

fn main() {
    part1();
    part2();
}
