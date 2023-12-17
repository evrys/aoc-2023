mod grid;
use grid::{Direction, Grid, Point};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn can_connect_pipe(from: char, dir: Direction, to: char) -> bool {
    return list_connecting_dirs_of_pipe(from).contains(&dir)
        && list_connecting_dirs_of_pipe(to).contains(&dir.flip());
}

struct Pipemap {
    grid: Grid<char>,
    start: Point,
}

impl Pipemap {
    fn is_pipe_connection(&self, from: Point, to: Point) -> bool {
        let dir = Direction::from_adjacent_points(from, to);

        match dir {
            Some(dir) => can_connect_pipe(
                *self.grid.get(from.0, from.1),
                dir,
                *self.grid.get(to.0, to.1),
            ),
            None => false,
        }
    }

    fn adjacent_connections(&self, p: Point) -> Vec<Point> {
        return self
            .grid
            .adjacencies(p.0, p.1)
            .iter()
            .filter(|adj| self.is_pipe_connection(p, **adj))
            .map(|p| *p)
            .collect();
    }

    fn find_pipe_loop(&self) -> Vec<Point> {
        let mut current = self.start;
        let mut path = vec![self.start];
        let mut visited = HashSet::from([self.start]);

        loop {
            let connections = self.adjacent_connections(current);
            let next = connections
                .iter()
                .filter(|adj| !visited.contains(*adj))
                .next();

            match next {
                Some(next) => {
                    path.push(*next);
                    visited.insert(*next);
                    current = *next;
                }
                None => {
                    break;
                }
            }
        }

        return path;
    }
}

impl From<&str> for Pipemap {
    fn from(str: &str) -> Pipemap {
        let mut grid = Grid::from(str);
        let start = grid.find('S').unwrap();

        // Figure out what kind of pipe the start is
        for maybe_start_pipe in ['|', '-', 'L', 'J', '7', 'F'] {
            let dirs = list_connecting_dirs_of_pipe(maybe_start_pipe);
            let valid_dirs = dirs
                .iter()
                .filter(|dir| {
                    let target = grid.try_go_direction(start, **dir);

                    match target {
                        Some(target) => {
                            can_connect_pipe(maybe_start_pipe, **dir, *grid.get(target.0, target.1))
                        }
                        None => false,
                    }
                })
                .count();

            if valid_dirs == 2 {
                grid.set(start.0, start.1, maybe_start_pipe);
            }
        }

        Pipemap { grid, start }
    }
}

const _TEST_INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
fn list_connecting_dirs_of_pipe(pipe: char) -> Vec<Direction> {
    match pipe {
        '|' => vec![Direction::North, Direction::South],
        '-' => vec![Direction::East, Direction::West],
        'L' => vec![Direction::North, Direction::East],
        'J' => vec![Direction::North, Direction::West],
        '7' => vec![Direction::South, Direction::West],
        'F' => vec![Direction::South, Direction::East],
        'S' => vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ],
        _ => vec![],
    }
}

fn part1() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();

    let map = Pipemap::from(puzzle_input.as_str());

    let paths = map.grid.pathflood(map.start.0, map.start.1, |a, b| {
        map.is_pipe_connection(a, b)
    });

    let mut dests = paths.keys().collect::<Vec<_>>();
    dests.sort_by_key(|p| paths.get(p).unwrap().distance);
    let furthest = dests.last().unwrap();
    let distance = paths.get(furthest).unwrap().distance;

    println!("Part 1: {}", distance);
}

// const _TEST_INPUT_2: &str = "..........
// .S------7.
// .|F----7|.
// .||....||.
// .||....||.
// .|L-7F-J|.
// .|..||..|.
// .L--JL--J.
// ..........";

const _TEST_INPUT_2: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

// F--7
// |..|
// S--J

fn part2() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    // let puzzle_input = _TEST_INPUT_2;

    let mut orig_map = Pipemap::from(puzzle_input.as_str());

    let pipe_loop = orig_map.find_pipe_loop();
    let pipe_tiles = pipe_loop.iter().collect::<HashSet<_>>();

    // First, remove all the unconnected pipe tiles
    // from the grid (problem treats them the same as empty space)
    for i in 0..orig_map.grid.width {
        for j in 0..orig_map.grid.height {
            if !pipe_tiles.contains(&(i, j)) {
                orig_map.grid.set(i, j, '.')
            }
        }
    }

    let orig_grid = orig_map.grid;

    // Make big grid

    let mut grid = Grid::new(orig_grid.width * 3, orig_grid.height * 3, '.');

    for i in 0..orig_grid.width {
        for j in 0..orig_grid.height {
            let ch = *orig_grid.get(i, j);
            let center = (i * 3 + 1, j * 3 + 1);
            grid.set(center.0, center.1, ch);

            for dir in list_connecting_dirs_of_pipe(ch) {
                let target = grid.try_go_direction(center, dir);

                if !target.is_some() {
                    continue;
                }

                let target = target.unwrap();

                match dir {
                    Direction::North | Direction::South => {
                        grid.set(target.0, target.1, '|');
                    }
                    Direction::West | Direction::East => {
                        grid.set(target.0, target.1, '-');
                    }
                }
            }
        }
    }

    let mut bigmap = Pipemap {
        grid,
        start: (orig_map.start.0 * 3 + 1, orig_map.start.1 * 3 + 1),
    };

    println!("{}", bigmap.grid);

    let bigpipe = bigmap.find_pipe_loop();
    let bigpipe_tiles = bigpipe.iter().collect::<HashSet<_>>();

    let mut in_or_out = HashMap::<Point, bool>::new();

    for i in 0..bigmap.grid.width {
        for j in 0..bigmap.grid.height {
            if bigpipe_tiles.contains(&(i, j)) || in_or_out.contains_key(&(i, j)) {
                continue;
            }

            let paths = bigmap.grid.pathflood(i, j, |a, b| {
                bigmap.grid.get(a.0, a.1) == &'.' && bigmap.grid.get(b.0, b.1) == &'.'
            });

            let reaches_outside = paths.keys().any(|p| {
                p.0 == 0
                    || p.0 == bigmap.grid.width - 1
                    || p.1 == 0
                    || p.1 == bigmap.grid.height - 1
            });

            for p in paths.keys() {
                in_or_out.insert(*p, !reaches_outside);
            }
        }
    }

    for (p, inside) in in_or_out.iter() {
        if *inside {
            bigmap.grid.set(p.0, p.1, 'I');
        } else {
            bigmap.grid.set(p.0, p.1, 'O');
        }
    }

    let mut count = 0;
    for i in 0..orig_grid.width {
        for j in 0..orig_grid.height {
            let mut inside = true;
            for i2 in 0..3 {
                for j2 in 0..3 {
                    if bigmap.grid.get(i * 3 + i2, j * 3 + j2) != &'I' {
                        inside = false;
                    }
                }
            }

            if inside {
                count += 1;
            }
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    part1();
    part2();
}
