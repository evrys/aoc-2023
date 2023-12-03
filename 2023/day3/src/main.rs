use std::{
    collections::{HashMap, HashSet},
    fs,
};

use regex::Regex;

const _TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct EngineNumber {
    number: u32,
    x1: usize,
    x2: usize,
    y: usize,
}

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;
    let mut engine_numbers = Vec::<EngineNumber>::new();

    for (y, line) in puzzle_input.lines().enumerate() {
        let re = Regex::new(r"[0-9]+").unwrap();
        println!("{} {}", y, line);
        for m in re.find_iter(line) {
            engine_numbers.push(EngineNumber {
                number: m.as_str().parse::<u32>().unwrap(),
                x1: m.start(),
                x2: m.end(),
                y,
            });
        }
    }

    let grid = puzzle_input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    for engine_number in engine_numbers {
        let y = engine_number.y;
        for x in engine_number.x1..engine_number.x2 {
            let adjacencies = [
                (y as isize - 1, x as isize - 1),
                (y as isize - 1, x as isize),
                (y as isize - 1, x as isize + 1),
                (y as isize, x as isize - 1),
                (y as isize, x as isize + 1),
                (y as isize + 1, x as isize - 1),
                (y as isize + 1, x as isize),
                (y as isize + 1, x as isize + 1),
            ];

            let adjacent_engine_part = adjacencies.iter().find(|(y2, x2)| {
                if *y2 >= 0
                    && *y2 < grid.len() as isize
                    && *x2 >= 0
                    && *x2 < grid[*y2 as usize].len() as isize
                {
                    let adj_char = grid[*y2 as usize][*x2 as usize];
                    if adj_char != '.' && !adj_char.is_numeric() {
                        return true;
                    }
                }

                return false;
            });

            if adjacent_engine_part.is_some() {
                sum += engine_number.number;
                break;
            }
        }
    }

    println!("Part 1: {}", sum);
}

struct Gear {
    adjacent_numbers: HashSet<EngineNumber>,
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;
    let mut engine_numbers = Vec::<EngineNumber>::new();

    for (y, line) in puzzle_input.lines().enumerate() {
        let re = Regex::new(r"[0-9]+").unwrap();
        println!("{} {}", y, line);
        for m in re.find_iter(line) {
            engine_numbers.push(EngineNumber {
                number: m.as_str().parse::<u32>().unwrap(),
                x1: m.start(),
                x2: m.end(),
                y,
            });
        }
    }

    let grid = puzzle_input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut gears = HashMap::<(usize, usize), Gear>::new();

    for engine_number in engine_numbers {
        let y = engine_number.y;
        for x in engine_number.x1..engine_number.x2 {
            let adjacencies = [
                (y as isize - 1, x as isize - 1),
                (y as isize - 1, x as isize),
                (y as isize - 1, x as isize + 1),
                (y as isize, x as isize - 1),
                (y as isize, x as isize + 1),
                (y as isize + 1, x as isize - 1),
                (y as isize + 1, x as isize),
                (y as isize + 1, x as isize + 1),
            ];

            for (y2, x2) in adjacencies.iter() {
                if *y2 >= 0
                    && *y2 < grid.len() as isize
                    && *x2 >= 0
                    && *x2 < grid[*y2 as usize].len() as isize
                {
                    let adj_char = grid[*y2 as usize][*x2 as usize];
                    if adj_char == '*' {
                        let gear = (*y2 as usize, *x2 as usize);
                        if !gears.contains_key(&gear) {
                            gears.insert(
                                gear,
                                Gear {
                                    adjacent_numbers: HashSet::new(),
                                },
                            );
                        }

                        let gear = gears.get_mut(&gear).unwrap();
                        gear.adjacent_numbers.insert(engine_number);
                    }
                }
            }
        }
    }

    let sum = gears
        .values()
        .filter_map(|gear| {
            if gear.adjacent_numbers.len() == 2 {
                let mut iter = gear.adjacent_numbers.iter();
                return Some(iter.next().unwrap().number * iter.next().unwrap().number);
            } else {
                return None;
            }
        })
        .sum::<u32>();

    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}
