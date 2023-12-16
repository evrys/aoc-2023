use std::fs;

const _TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

struct NumberDiffBreakdown {
    numbers: Vec<i64>,
}

impl Iterator for NumberDiffBreakdown {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        let diff_numbers: Vec<i64> = self.numbers[1..]
            .iter()
            .enumerate()
            .map(|(i, n)| {
                return n - self.numbers[i];
            })
            .collect();

        if diff_numbers.iter().all(|n| *n == 0) {
            None
        } else {
            Some(diff_numbers)
        }
    }
}

fn parse_and_breakdown_numbers(line: &str) -> Vec<Vec<i64>> {
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    return NumberDiffBreakdown { numbers }.collect();
}

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let answer: i64 = puzzle_input
        .lines()
        .map(|line| {
            let breakdown = parse_and_breakdown_numbers(line);

            let next_number = breakdown.iter().rfold(0, |acc, nums| {
                return acc + nums[nums.len() - 1];
            });

            return next_number;
        })
        .sum();

    println!("Part 1: {}", answer);
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let answer = puzzle_input.lines().fold(0, |a, line| {
        let breakdown = parse_and_breakdown_numbers(line);

        let next_number = breakdown.iter().rfold(0, |acc, nums| {
            return nums[0] - acc;
        });

        return a + next_number;
    });

    println!("Part 2: {}", answer);
}

fn main() {
    part1();
    part2();
}
