use fancy_regex::Regex;
use std::fs;

fn count_vowels(line: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut vowel_count = 0;
    for c in line.chars() {
        if vowels.contains(&c) {
            vowel_count += 1;
        }
    }
    return vowel_count;
}

fn has_double_letter(line: &str) -> bool {
    let mut prev_char = ' ';
    for c in line.chars() {
        if c == prev_char {
            return true;
        }
        prev_char = c;
    }
    return false;
}

fn has_bad_strings(line: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    for bad_string in bad_strings.iter() {
        if line.contains(bad_string) {
            return true;
        }
    }
    return false;
}

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let result = puzzle_input
        .lines()
        .filter(|line| count_vowels(line) >= 3 && has_double_letter(line) && !has_bad_strings(line))
        .count();

    println!("Part 1: {}", result);
}

fn has_double_letter_pair(line: &str) -> bool {
    let re = Regex::new(r"([a-z]{2}).*\1").unwrap();
    return re.is_match(line).unwrap();
}

fn has_double_letter_with_gap(line: &str) -> bool {
    let re = Regex::new(r"([a-z]).\1").unwrap();
    return re.is_match(line).unwrap();
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let result = puzzle_input
        .lines()
        .filter(|line| has_double_letter_pair(line) && has_double_letter_with_gap(line))
        .count();

    println!("Part 2: {}", result);
}

fn main() {
    part1();
    part2();
}
