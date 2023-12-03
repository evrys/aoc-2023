use std::fs;

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let result: i32 = puzzle_input
        .lines()
        .map(|line| {
            let mut iter = line.splitn(3, 'x');
            let l = iter.next().unwrap().parse::<i32>().unwrap();
            let w = iter.next().unwrap().parse::<i32>().unwrap();
            let h = iter.next().unwrap().parse::<i32>().unwrap();

            return 2 * l * w + 2 * w * h + 2 * h * l + [l * w, w * h, h * l].iter().min().unwrap();
        })
        .sum();

    println!("Part 1: {}", result);
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let result: i32 = puzzle_input
        .lines()
        .map(|line| {
            let mut iter = line.splitn(3, 'x');
            let l = iter.next().unwrap().parse::<i32>().unwrap();
            let w = iter.next().unwrap().parse::<i32>().unwrap();
            let h = iter.next().unwrap().parse::<i32>().unwrap();

            let mut sides = [l, w, h];
            sides.sort();

            return sides[0] * 2 + sides[1] * 2 + l * w * h;
        })
        .sum();

    println!("Part 2: {}", result);
}

fn main() {
    part1();
    part2();
}
