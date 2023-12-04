use std::{collections::HashSet, fs};

const _TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;

    let mut sum = 0;
    for line in puzzle_input.lines() {
        let (_, rest) = line.split_once(": ").unwrap();
        let (winner_str, having_str) = rest.split_once(" | ").unwrap();
        let winners = winner_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let havenums = having_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        let mut score = 0;
        for _ in winners.intersection(&havenums) {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
        sum += score;
    }

    println!("Part 1: {}", sum);
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;

    let lines = puzzle_input.lines().collect::<Vec<_>>();
    let mut card_counts = vec![1; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let (_, rest) = line.split_once(": ").unwrap();
        let (winner_str, having_str) = rest.split_once(" | ").unwrap();
        let winners = winner_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let havenums = having_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        let num_matches = winners.intersection(&havenums).count();

        println!("{}: {}", i + 1, card_counts[i]);
        for _ in 0..card_counts[i] {
            for j in 0..num_matches {
                card_counts[i + j + 1] += 1;
            }
        }
    }

    let sum = card_counts.iter().sum::<usize>();

    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}
