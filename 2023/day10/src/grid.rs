use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn from_adjacent_points(p1: Point, p2: Point) -> Option<Direction> {
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        if x1 == x2 {
            if y1 + 1 == y2 {
                return Some(Direction::South);
            } else if y1 == y2 + 1 {
                return Some(Direction::North);
            }
        } else if y1 == y2 {
            if x1 + 1 == x2 {
                return Some(Direction::East);
            } else if x1 == x2 + 1 {
                return Some(Direction::West);
            }
        }

        return None;
    }

    pub fn flip(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

pub type Point = (usize, usize);

pub struct PathResult {
    pub previous: Option<Point>,
    pub distance: usize,
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Grid<T> {
        Grid {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y * self.width + x] = value;
    }

    pub fn find(&self, thing: T) -> Option<(usize, usize)> {
        self.data
            .iter()
            .position(|x| *x == thing)
            .map(|i| (i % self.width, i / self.width))
    }

    pub fn try_go_direction(&self, from: Point, dir: Direction) -> Option<Point> {
        let (x, y) = from;

        match dir {
            Direction::North => {
                if y > 0 {
                    return Some((x, y - 1));
                }
            }
            Direction::South => {
                if y < self.height - 1 {
                    return Some((x, y + 1));
                }
            }
            Direction::East => {
                if x < self.width - 1 {
                    return Some((x + 1, y));
                }
            }
            Direction::West => {
                if x > 0 {
                    return Some((x - 1, y));
                }
            }
        }

        return None;
    }

    pub fn adjacencies(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        if y > 0 {
            result.push((x, y - 1));
        }

        if y < self.height - 1 {
            result.push((x, y + 1));
        }

        if x > 0 {
            result.push((x - 1, y));
        }

        if x < self.width - 1 {
            result.push((x + 1, y));
        }

        return result;
    }

    pub fn pathflood(
        &self,
        x: usize,
        y: usize,
        can_move: impl Fn(Point, Point) -> bool,
    ) -> HashMap<Point, PathResult> {
        let mut frontier = VecDeque::new();
        frontier.push_back((x, y));
        let mut results = HashMap::new();
        results.insert(
            (x, y),
            PathResult {
                previous: None,
                distance: 0,
            },
        );

        while !frontier.is_empty() {
            let current = frontier.pop_front().unwrap();

            for next in self.adjacencies(current.0, current.1) {
                if !results.contains_key(&next) && can_move(current, next) {
                    frontier.push_back(next);
                    results.insert(
                        next,
                        PathResult {
                            previous: Some(current),
                            distance: results.get(&current).unwrap().distance + 1,
                        },
                    );
                }
            }
        }

        return results;
    }
}

impl From<&str> for Grid<char> {
    fn from(str: &str) -> Grid<char> {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();

        for line in str.lines() {
            width = line.len();
            height += 1;
            data.extend(line.chars());
        }

        Grid {
            data,
            width,
            height,
        }
    }
}

impl std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
