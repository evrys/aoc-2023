use std::{collections::HashMap, fs};

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let sum = puzzle_input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
            let last_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
            return format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .unwrap();
        })
        .sum::<i32>();

    println!("{}", sum);
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    let digit_map = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let sum = puzzle_input
        .lines()
        .map(|line| {
            let mut best_first: i32 = 0;
            let mut best_last: i32 = 0;
            let mut best_start: usize = usize::MAX;
            let mut best_end: usize = 0;

            for (word, digit) in digit_map.iter() {
                for candidate in [word, digit.to_string().as_str()] {
                    let word_start = line.find(candidate);
                    let word_end = line
                        .match_indices(candidate)
                        .last()
                        .and_then(|w| Some(w.0 + candidate.len()));

                    match word_start {
                        Some(start) => {
                            if start < best_start {
                                best_start = start;
                                best_first = *digit;
                            }
                        }
                        None => {}
                    }

                    match word_end {
                        Some(end) => {
                            if end > best_end {
                                best_end = end;
                                best_last = *digit;
                            }
                        }
                        None => {}
                    }
                }
            }

            return format!("{}{}", best_first, best_last)
                .parse::<i32>()
                .unwrap();
        })
        .sum::<i32>();

    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
