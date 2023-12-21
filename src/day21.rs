use crate::common::Position;
use failure::Error;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Grid {
    rocks: HashSet<Position>,
    max_x: i64,
    max_y: i64,
    start: Position,
}

impl Grid {
    fn is_valid(&self, position: Position) -> bool {
        position.x >= 0 && position.x <= self.max_x && position.y >= 0 && position.y <= self.max_y
    }

    fn can_move_to(&self, position: Position) -> bool {
        self.is_valid(position) && !self.rocks.contains(&position)
    }
}

fn find_max_plots(grid: &Grid, num_steps: u64) -> usize {
    let mut current = vec![grid.start];

    for _ in 0..num_steps {
        current = current
            .into_iter()
            .flat_map(|pos| pos.adjacent())
            .unique()
            .filter(|pos| grid.can_move_to(*pos))
            .collect();
    }

    current.len()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Grid;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let rocks = data
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x, y).into())
                    } else {
                        None
                    }
                })
            })
            .collect();

        let max_x = (data.lines().next().unwrap().len() - 1) as i64;
        let max_y = (data.lines().count() - 1) as i64;

        let start = data
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find_map(|(x, c)| if c == 'S' { Some((x, y).into()) } else { None })
            })
            .unwrap();

        Ok(Grid {
            rocks,
            max_x,
            max_y,
            start,
        })
    }

    fn solve(grid: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_max_plots(&grid, 64);

        (Some(part1.to_string()), None)
    }
}
