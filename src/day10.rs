use crate::common::{Direction, Position};
use failure::{err_msg, Error};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct Pipe {
    directions: [Direction; 2],
}

impl Pipe {
    fn new(dir1: Direction, dir2: Direction) -> Self {
        Pipe {
            directions: [dir1, dir2],
        }
    }

    fn new_dir(self, direction: Direction) -> Option<Direction> {
        if direction == self.directions[0].reverse() {
            Some(self.directions[1])
        } else if direction == self.directions[1].reverse() {
            Some(self.directions[0])
        } else {
            None
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            '-' => Ok(Pipe::new(East, West)),
            '|' => Ok(Pipe::new(North, South)),
            'L' => Ok(Pipe::new(North, East)),
            'J' => Ok(Pipe::new(North, West)),
            '7' => Ok(Pipe::new(South, West)),
            'F' => Ok(Pipe::new(South, East)),
            _ => Err(()),
        }
    }
}

fn find_furthest_distance(start: Position, pipes: &HashMap<Position, Pipe>) -> usize {
    let mut current: Vec<_> = Direction::all().map(|dir| (start, dir)).collect();

    for distance in 1.. {
        let next: Vec<_> = current
            .iter()
            .filter_map(|(pos, dir)| {
                let next_pos = pos.step(*dir);
                pipes
                    .get(&next_pos)
                    .and_then(|pipe| pipe.new_dir(*dir))
                    .map(|new_dir| (next_pos, new_dir))
            })
            .collect();

        assert!(next.len() > 1);

        if !next.iter().map(|(pos, _)| pos).all_unique() {
            return distance;
        }

        if next
            .iter()
            .any(|(pos, _)| current.iter().any(|(prev_pos, _)| pos == prev_pos))
        {
            return distance - 1;
        }

        current = next;
    }

    unreachable!()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Position, HashMap<Position, Pipe>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let start = data
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars().enumerate().find_map(|(x, c)| {
                    if c == 'S' {
                        Some(Position {
                            x: x as i64,
                            y: y as i64,
                        })
                    } else {
                        None
                    }
                })
            })
            .ok_or(err_msg(format!("Failed to find start position")))?;

        let pipes = data
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    c.try_into().ok().map(|pipe| {
                        (
                            Position {
                                x: x as i64,
                                y: y as i64,
                            },
                            pipe,
                        )
                    })
                })
            })
            .collect();

        Ok((start, pipes))
    }

    fn solve((start, pipes): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_furthest_distance(start, &pipes);
        (Some(part1.to_string()), None)
    }
}
