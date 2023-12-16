use crate::common::{Direction, Position};
use failure::Error;
use itertools::Either;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirror {
    Left,
    Right,
}

impl Mirror {
    fn map_direction(self, dir: Direction) -> impl Iterator<Item = Direction> {
        use Direction::*;
        use Mirror::*;
        [match (self, dir) {
            (Right, North) => East,
            (Right, East) => North,
            (Right, South) => West,
            (Right, West) => South,
            (Left, North) => West,
            (Left, East) => South,
            (Left, South) => East,
            (Left, West) => North,
        }]
        .into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Splitter {
    Across,
    Down,
}

impl Splitter {
    fn map_direction(self, dir: Direction) -> impl Iterator<Item = Direction> {
        use Direction::*;
        use Splitter::*;
        match (self, dir) {
            (Across, North) | (Across, South) => Either::Left([East, West].into_iter()),
            (Down, East) | (Down, West) => Either::Left([North, South].into_iter()),
            _ => Either::Right([dir].into_iter()),
        }
    }
}

pub struct Objects {
    objects: HashMap<Position, Object>,
    max_x: i64,
    max_y: i64,
}

impl Objects {
    fn new(objects: HashMap<Position, Object>) -> Self {
        let max_x = objects.keys().map(|pos| pos.x).max().unwrap();
        let max_y = objects.keys().map(|pos| pos.y).max().unwrap();

        Objects {
            objects,
            max_x,
            max_y,
        }
    }

    fn pos_valid(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.x <= self.max_x && pos.y >= 0 && pos.y <= self.max_y
    }

    fn get(&self, pos: &Position) -> Option<&Object> {
        self.objects.get(pos)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Mirror(Mirror),
    Splitter(Splitter),
}

impl Object {
    fn map_direction(self, dir: Direction) -> impl Iterator<Item = Direction> {
        match self {
            Object::Mirror(mirror) => Either::Left(mirror.map_direction(dir)),
            Object::Splitter(splitter) => Either::Right(splitter.map_direction(dir)),
        }
    }
}

fn num_energised(objects: &Objects, start_pos: Position, start_dir: Direction) -> usize {
    let mut energised = HashSet::new();
    let mut visited = HashSet::new();

    let mut positions = vec![(start_pos, start_dir)];

    while !positions.is_empty() {
        positions.retain(|loc| !visited.contains(loc));
        visited.extend(positions.clone());

        energised.extend(positions.iter().map(|(pos, _)| *pos));

        positions = positions
            .into_iter()
            .flat_map(|(pos, dir)| {
                if let Some(obj) = objects.get(&pos) {
                    Either::Left(obj.map_direction(dir))
                } else {
                    Either::Right([dir].into_iter())
                }
                .filter_map(move |new_dir| {
                    let new_pos = pos.step(new_dir);
                    if !objects.pos_valid(new_pos) {
                        None
                    } else {
                        Some((new_pos, new_dir))
                    }
                })
            })
            .collect();
    }

    energised.len()
}

fn find_most_energised(objects: &Objects) -> usize {
    use Direction::*;
    Direction::all()
        .flat_map(|dir| {
            match dir {
                North => Either::Left(Either::Left((0..=objects.max_x).map(|x| Position {
                    x,
                    y: objects.max_y,
                }))),
                East => Either::Left(Either::Right(
                    (0..=objects.max_y).map(|y| Position { x: 0, y }),
                )),
                South => Either::Right(Either::Left(
                    (0..=objects.max_x).map(|x| Position { x, y: 0 }),
                )),
                West => Either::Right(Either::Right((0..=objects.max_y).map(|y| Position {
                    x: objects.max_x,
                    y,
                }))),
            }
            .map(move |pos| (pos, dir))
        })
        .map(|(start_pos, start_dir)| num_energised(objects, start_pos, start_dir))
        .max()
        .unwrap()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Objects;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(Objects::new(
            data.lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        match c {
                            '|' => Some(Object::Splitter(Splitter::Down)),
                            '-' => Some(Object::Splitter(Splitter::Across)),
                            '/' => Some(Object::Mirror(Mirror::Right)),
                            '\\' => Some(Object::Mirror(Mirror::Left)),
                            _ => None,
                        }
                        .map(|obj| ((x, y).into(), obj))
                    })
                })
                .collect(),
        ))
    }

    fn solve(objects: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = num_energised(&objects, Position::origin(), Direction::East);
        let part2 = find_most_energised(&objects);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
