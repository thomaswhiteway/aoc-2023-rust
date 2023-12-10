use crate::common::{Direction, Position};
use failure::{err_msg, Error};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

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

    fn is_vertical(&self) -> bool {
        use Direction::*;
        self.directions == [North, South] || self.directions == [South, North]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScanState {
    OffPipe(bool),
    OnPipe(Direction, bool),
}

fn find_loop(start: Position, pipes: &mut HashMap<Position, Pipe>) -> HashSet<Position> {
    let mut current: Vec<_> = Direction::all().map(|dir| (vec![start], dir)).collect();

    loop {
        current.retain_mut(|(route, dir)| {
            let next_pos = route.last().unwrap().step(*dir);
            if let Some(next_dir) = pipes.get(&next_pos).and_then(|pipe| pipe.new_dir(*dir)) {
                route.push(next_pos);
                *dir = next_dir;
                true
            } else {
                false
            }
        });

        assert!(current.len() > 1);

        for i in 0..current.len() {
            let this_route = &current[i].0;
            for (other_route, _) in current.iter().skip(i+1) {
                if this_route.last().unwrap() == other_route.last().unwrap() {
                    pipes.insert(
                        start,
                        Pipe::new(
                            start.direction_to(&this_route[1]).unwrap(),
                            start.direction_to(&other_route[1]).unwrap(),
                        ),
                    );

                    return this_route
                        .iter()
                        .chain(other_route.iter())
                        .cloned()
                        .collect();
                }
            }
        }
    }
}

fn find_furthest_distance(pipe_loop: &HashSet<Position>) -> usize {
    pipe_loop.len() / 2
}

fn find_spaces_inside(pipes: &HashMap<Position, Pipe>, pipe_loop: &HashSet<Position>) -> usize {
    use Direction::*;
    use ScanState::*;

    let mut total = 0;

    let (min_x, max_x) = pipes
        .keys()
        .map(|pos| pos.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = pipes
        .keys()
        .map(|pos| pos.y)
        .minmax()
        .into_option()
        .unwrap();

    for y in min_y..=max_y {
        let mut state = OffPipe(false);

        for x in min_x..=max_x {
            let pos = Position { x, y };
            if pipe_loop.contains(&pos) {
                let pipe = pipes.get(&pos).unwrap();

                state = match state {
                    OffPipe(inside) => {
                        if pipe.is_vertical() {
                            OffPipe(!inside)
                        } else {
                            OnPipe(pipe.new_dir(West).unwrap(), inside)
                        }
                    }
                    OnPipe(dir, inside) => match pipe.new_dir(East).unwrap() {
                        East => state,
                        other => {
                            assert!(other != East);
                            if other != dir {
                                OffPipe(!inside)
                            } else {
                                OffPipe(inside)
                            }
                        }
                    },
                }
            } else {
                match state {
                    OffPipe(true) => {
                        total += 1;
                    }
                    OffPipe(_) => {}
                    _ => unreachable!(),
                }
            }
        }
    }

    total
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
            .ok_or(err_msg("Failed to find start position"))?;

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

    fn solve((start, mut pipes): Self::Problem) -> (Option<String>, Option<String>) {
        let pipe_loop = find_loop(start, &mut pipes);

        let part1 = find_furthest_distance(&pipe_loop);
        let part2 = find_spaces_inside(&pipes, &pipe_loop);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
