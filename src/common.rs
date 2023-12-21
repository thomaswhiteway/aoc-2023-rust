#![allow(unused)]

use itertools::iproduct;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn manhattan_distance_to(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn adjacent(self) -> impl Iterator<Item = Position> {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| Position {
                x: self.x + dx,
                y: self.y + dy,
            })
    }

    pub fn surrounding(&self) -> impl Iterator<Item = Position> + '_ {
        iproduct!([-1, 0, 1], [-1, 0, 1]).filter_map(|(dx, dy)| {
            if dx != 0 || dy != 0 {
                Some(Position {
                    x: self.x + dx,
                    y: self.y + dy,
                })
            } else {
                None
            }
        })
    }

    pub fn direction_to(&self, other: &Self) -> Option<Direction> {
        match (other.x - self.x, other.y - self.y) {
            (0, dy) if dy < 0 => Some(Direction::North),
            (dx, 0) if dx > 0 => Some(Direction::East),
            (0, dy) if dy > 0 => Some(Direction::South),
            (dx, 0) if dx < 0 => Some(Direction::West),
            _ => None,
        }
    }

    pub fn length(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub fn points_to(self, other: Position) -> impl Iterator<Item = Position> {
        let diff = other - self;
        assert!(diff.x == 0 || diff.y == 0);
        let distance = diff.length();
        let delta = diff / distance;
        (0..distance).map(move |index| self + delta * index)
    }

    pub fn step(self, direction: Direction) -> Self {
        self + direction.offset()
    }

    pub fn step_by(self, direction: Direction, len: u32) -> Self {
        self + direction.offset() * len as i64
    }

    pub fn origin() -> Self {
        Position { x: 0, y: 0 }
    }
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Position { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<i64> for Position {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Position {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<i64> for Position {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Self> {
        use Direction::*;
        [North, East, South, West].into_iter()
    }

    pub fn as_char(&self) -> char {
        use Direction::*;
        match self {
            North => '^',
            East => '>',
            South => 'V',
            West => '<',
        }
    }

    pub fn offset(self) -> Position {
        use Direction::*;
        match self {
            North => Position { x: 0, y: -1 },
            East => Position { x: 1, y: 0 },
            South => Position { x: 0, y: 1 },
            West => Position { x: -1, y: 0 },
        }
    }

    pub fn reverse(self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub fn turn_left(self) -> Direction {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn turn_right(self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}
