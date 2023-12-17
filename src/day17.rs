use crate::a_star;
use failure::{err_msg, Error};
use std::fmt::Debug;
use std::{collections::HashMap, hash::Hash, str::FromStr};

use crate::common::{Direction, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Crucible {
    min_row: u8,
    max_row: u8,
}

impl Crucible {
    fn can_turn(self, steps_in_direction: u8) -> bool {
        steps_in_direction >= self.min_row
    }

    fn must_turn(self, steps_in_direction: u8) -> bool {
        steps_in_direction >= self.max_row
    }

    fn can_stop(self, steps_in_direction: u8) -> bool {
        steps_in_direction >= self.min_row
    }
}

pub struct Grid {
    width: i64,
    height: i64,
    heat_loss: HashMap<Position, u64>,
}

impl Grid {
    fn new(heat_loss: HashMap<Position, u64>) -> Self {
        let width = heat_loss.keys().map(|pos| pos.x).max().unwrap_or(0);
        let height = heat_loss.keys().map(|pos| pos.x).max().unwrap_or(0);
        Grid {
            width,
            height,
            heat_loss,
        }
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    c.to_digit(10)
                        .map(|d| ((x, y).into(), d as u64))
                        .ok_or_else(|| err_msg(format!("Invalid digit {}", c)))
                })
            })
            .collect::<Result<_, _>>()
            .map(Grid::new)
    }
}

#[derive(Clone)]
struct State<'a> {
    grid: &'a Grid,
    crucible: Crucible,
    position: Position,
    target: Position,
    direction: Direction,
    steps_in_direction: u8,
}

impl State<'_> {
    fn step(&self, direction: Direction) -> Self {
        let position = self.position.step(direction);
        let steps_in_direction = if direction == self.direction {
            self.steps_in_direction + 1
        } else {
            1
        };
        State {
            grid: self.grid,
            crucible: self.crucible,
            position,
            target: self.target,
            direction,
            steps_in_direction,
        }
    }
}

impl Debug for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State{{{:?}, {:?}, {:?}, {:?}}}",
            self.position, self.target, self.direction, self.steps_in_direction
        )
    }
}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.direction == other.direction
            && self.steps_in_direction == other.steps_in_direction
    }
}

impl Eq for State<'_> {}

impl Hash for State<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.hash(state);
        self.steps_in_direction.hash(state);
    }
}

impl a_star::State for State<'_> {
    fn heuristic(&self) -> u64 {
        self.position.manhattan_distance_to(&self.target)
    }

    fn successors(&self) -> Vec<(u64, Self)> {
        let mut new_states = vec![];
        if !self.crucible.must_turn(self.steps_in_direction) {
            new_states.push(self.step(self.direction));
        }

        if self.crucible.can_turn(self.steps_in_direction) {
            new_states.push(self.step(self.direction.turn_left()));
            new_states.push(self.step(self.direction.turn_right()));
        }

        new_states
            .into_iter()
            .filter_map(|state| {
                self.grid
                    .heat_loss
                    .get(&state.position)
                    .map(|heat_loss| (*heat_loss, state))
            })
            .collect()
    }

    fn is_end(&self) -> bool {
        self.position == self.target && self.crucible.can_stop(self.steps_in_direction)
    }
}

fn find_min_heat_loss(grid: &Grid, crucible: Crucible) -> u64 {
    a_star::solve(
        [Direction::East, Direction::South]
            .into_iter()
            .map(|direction| State {
                grid,
                crucible,
                position: Position::origin(),
                target: Position {
                    x: grid.width,
                    y: grid.height,
                },
                direction,
                steps_in_direction: 0,
            }),
    )
    .unwrap()
    .cost
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Grid;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.parse()
    }

    fn solve(grid: Self::Problem) -> (Option<String>, Option<String>) {
        let crucible = Crucible {
            min_row: 0,
            max_row: 3,
        };
        let part1 = find_min_heat_loss(&grid, crucible);

        let ultra_crucible = Crucible {
            min_row: 4,
            max_row: 10,
        };
        let part2 = find_min_heat_loss(&grid, ultra_crucible);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
