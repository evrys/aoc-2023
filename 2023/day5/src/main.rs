use std::fs;

const _TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[derive(Debug)]
struct MapPart {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl MapPart {
    fn flip(&self) -> MapPart {
        return MapPart {
            dest_range_start: self.source_range_start,
            source_range_start: self.dest_range_start,
            range_length: self.range_length,
        };
    }
}

#[derive(Debug)]
struct Mapper {
    start_type: String,
    dest_type: String,
    parts: Vec<MapPart>,
}

impl Mapper {
    fn convert(&self, value: u64) -> u64 {
        for part in self.parts.iter() {
            if value >= part.source_range_start
                && value < part.source_range_start + part.range_length
            {
                return part.dest_range_start + (value - part.source_range_start);
            }
        }

        return value;
    }
}

struct Garden {
    mappers: Vec<Mapper>,
}

impl Garden {
    fn find_mapper_path(&self, start_type: &str, dest_type: &str) -> Option<Vec<&Mapper>> {
        let candidates = self
            .mappers
            .iter()
            .filter(|mapper| mapper.start_type == start_type);

        for candidate in candidates {
            if candidate.dest_type == dest_type {
                return Some(vec![candidate]);
            } else {
                let path = self.find_mapper_path(&candidate.dest_type, dest_type);
                if path.is_some() {
                    let mut path = path.unwrap();
                    path.insert(0, candidate);
                    return Some(path);
                }
            }
        }

        return None;
    }

    fn convert(&self, value: u64, start_type: &str, dest_type: &str) -> u64 {
        let mut current_value = value;

        let path = self.find_mapper_path(start_type, dest_type);

        if path.is_none() {
            panic!("No path found from {} to {}", start_type, dest_type);
        }

        let path = path.unwrap();
        for mapper in path {
            current_value = mapper.convert(current_value);
        }
        return current_value;
    }
}

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;

    let mut iter = puzzle_input.lines();
    let seedline = iter.next().unwrap();
    let seeds: Vec<u64> = seedline
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();
    iter.next();

    let mut garden = Garden {
        mappers: Vec::new(),
    };

    loop {
        let block = iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();
        if block.is_empty() {
            break;
        }

        // let mappertype = block[0].split_whitespace().nth(0).unwrap();

        let (from_type, to_type) = block[0]
            .split_whitespace()
            .nth(0)
            .unwrap()
            .split_once("-to-")
            .unwrap();

        let mut mapper = Mapper {
            start_type: from_type.to_string(),
            dest_type: to_type.to_string(),
            parts: Vec::new(),
        };

        for line in block[1..].iter() {
            let mut numiter = line
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());

            let dest_range_start = numiter.next().unwrap();
            let source_range_start = numiter.next().unwrap();
            let range_length = numiter.next().unwrap();

            mapper.parts.push(MapPart {
                dest_range_start: dest_range_start,
                source_range_start: source_range_start,
                range_length: range_length,
            });
        }

        garden.mappers.push(mapper);
    }

    let mut locations: Vec<u64> = seeds
        .iter()
        .map(|seed| garden.convert(*seed, "seed", "location"))
        .collect();

    locations.sort();

    println!("Part 1: {}", locations[0]);
}

fn convert(seed: u64, path: &Vec<&Mapper>) -> u64 {
    let mut current = seed;
    for mapper in path.iter() {
        current = mapper.convert(current);
    }
    return current;
}

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT;

    let mut iter = puzzle_input.lines();
    let mut seednums = iter
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap());

    let mut ranges = Vec::new();

    loop {
        let pair = seednums.by_ref().take(2).collect::<Vec<_>>();
        if pair.is_empty() {
            break;
        }

        let (range_start, range_len) = (pair[0], pair[1]);

        ranges.push((range_start, range_len));
        // println!("Range: {} - {}", range_start, range_start + range_len - 1);
    }

    iter.next();

    let mut garden = Garden {
        mappers: Vec::new(),
    };

    loop {
        let blocks = iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();
        if blocks.is_empty() {
            break;
        }

        // let mappertype = blocks[0].split_whitespace().nth(0).unwrap();

        let (from_type, to_type) = blocks[0]
            .split_whitespace()
            .nth(0)
            .unwrap()
            .split_once("-to-")
            .unwrap();

        let mut mapper = Mapper {
            start_type: from_type.to_string(),
            dest_type: to_type.to_string(),
            parts: Vec::new(),
        };

        for line in blocks[1..].iter() {
            let mut numiter = line
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap());

            let dest_range_start = numiter.next().unwrap();
            let source_range_start = numiter.next().unwrap();
            let range_length = numiter.next().unwrap();

            mapper.parts.push(MapPart {
                dest_range_start: dest_range_start,
                source_range_start: source_range_start,
                range_length: range_length,
            });
        }

        garden.mappers.push(mapper);
    }

    let combinations = garden
        .mappers
        .iter()
        .map(|mapper| mapper.parts.len())
        .reduce(|a, b| a * b)
        .unwrap();

    println!("Combinations: {}", combinations);
    // println!("Part 2: {}", min_location);
}

fn main() {
    part1();
    part2();
}
