use std::fs;

const _TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;

    let mut iter = puzzle_input.lines();
    let times = iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let distances = iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut result = 1;
    for i in 0..times.len() {
        let record_to_beat = distances[i];
        let mut ways_to_win = 0;
        let mut held_ms = 0;

        while held_ms <= times[i] {
            held_ms += 1;
            let distance = held_ms * (times[i] - held_ms);
            if distance > record_to_beat {
                ways_to_win += 1;
            }
        }

        result *= ways_to_win;
    }

    println!("Part 1: {}", result);
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;

    let mut iter = puzzle_input.lines();
    let time = iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    let distance = iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>()
        .unwrap();

    let record_to_beat = distance;
    let mut ways_to_win = 0;
    let mut held_ms = 0;

    while held_ms <= time {
        held_ms += 1;
        let distance = held_ms * (time - held_ms);
        if distance > record_to_beat {
            ways_to_win += 1;
        }
    }

    println!("Part 2: {}", ways_to_win);
}

fn main() {
    part1();
    part2();
}
