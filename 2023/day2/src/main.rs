use std::fs;

const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = TEST_INPUT;

    let total_red = 12;
    let total_green = 13;
    let total_blue = 14;

    let result = puzzle_input
        .lines()
        .filter_map(|line| {
            let (game, pull_str) = line.split_once(": ").unwrap();
            let id = game.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            let pulls = pull_str.split("; ");

            for pull in pulls {
                let cubes = pull.split(", ");
                for cube in cubes {
                    let (count, color) = cube.split_once(" ").unwrap();
                    let count = count.parse::<i32>().unwrap();

                    if (color == "red" && count > total_red)
                        || (color == "green" && count > total_green)
                        || (color == "blue" && count > total_blue)
                    {
                        return None;
                    }
                }
            }

            return Some(id);
        })
        .sum::<i32>();

    println!("{}", result);
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = TEST_INPUT;

    let result = puzzle_input
        .lines()
        .map(|line| {
            let (_, pull_str) = line.split_once(": ").unwrap();
            let pulls = pull_str.split("; ");

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for pull in pulls {
                let cubes = pull.split(", ");
                for cube in cubes {
                    let (count, color) = cube.split_once(" ").unwrap();
                    let count = count.parse::<i32>().unwrap();

                    if color == "red" && count > max_red {
                        max_red = count;
                    } else if color == "green" && count > max_green {
                        max_green = count;
                    } else if color == "blue" && count > max_blue {
                        max_blue = count;
                    }
                }
            }

            return max_red * max_green * max_blue;
        })
        .sum::<i32>();

    println!("{}", result);
}

fn main() {
    part1();
    part2();
}
