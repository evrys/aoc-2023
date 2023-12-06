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

    // Original, brute force solution

    while held_ms <= time {
        held_ms += 1;
        let distance = held_ms * (time - held_ms);
        if distance > record_to_beat {
            ways_to_win += 1;
        }
    }

    println!("Part 2: {}", ways_to_win);

    // Later I figured out the proper mathsy way to do it

    // f(x) = x * (time - x)
    // f(x) = -x^2 + time * x
    // -x^2 + time * x = record_to_beat
    // -x^2 + time * x - record_to_beat = 0

    // Now we can apply quadratic formula

    // ax^2 + bx + c = 0
    // x = (-b±√(b²-4ac))/(2a)

    let a = -1.0;
    let b = time as f64;
    let c = -record_to_beat as f64;
    let root1 = (-b + (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a);
    let root2 = (-b - (b.powf(2.0) - (4.0 * a * c)).sqrt()) / (2.0 * a);

    // Every value of x between the first and last root on the parabola
    // Gives a higher distance than the record distance

    let first_root = if root1 < root2 { root1 } else { root2 };
    let last_root = if root1 > root2 { root1 } else { root2 };

    let num_better_x = (last_root - first_root) as i64;

    println!("Part 2 fancy: {}", num_better_x);
}

fn main() {
    part1();
    part2();
}
