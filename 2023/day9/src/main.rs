use std::fs;

const _TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let answer = puzzle_input.lines().fold(0, |a, line| {
        let orig_numbers: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let mut breakdown: Vec<Vec<i64>> = [orig_numbers.clone()].to_vec();

        let mut numbers = orig_numbers.clone();
        while numbers.iter().any(|n| *n != 0) {
            numbers = numbers[1..]
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    return n - numbers[i];
                })
                .collect();

            breakdown.push(numbers.clone());
        }

        let next_number = breakdown.iter().rfold(0, |acc, nums| {
            return acc + nums[nums.len() - 1];
        });

        return a + next_number;
    });

    println!("Part 1: {}", answer);
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let answer = puzzle_input.lines().fold(0, |a, line| {
        let orig_numbers: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let mut breakdown: Vec<Vec<i64>> = [orig_numbers.clone()].to_vec();

        let mut numbers = orig_numbers.clone();
        while numbers.iter().any(|n| *n != 0) {
            numbers = numbers[1..]
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    return n - numbers[i];
                })
                .collect();

            breakdown.push(numbers.clone());
        }

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
