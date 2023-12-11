use crate::common::Position;
use failure::Error;
use itertools::Itertools;

fn get_total_lengths(galaxies: &Vec<Position>) -> usize {
    let mut total = 0;

    let (min_x, max_x) = galaxies
        .iter()
        .map(|pos| pos.x)
        .minmax()
        .into_option()
        .unwrap();

    let mut right = galaxies.len();
    let mut left = 0;

    for x in min_x..=max_x {
        let num_in_col = galaxies.iter().filter(|pos| pos.x == x).count();

        if num_in_col == 0 {
            total += 2 * right * left;
        } else {
            total += right * left;
            right -= num_in_col;
            left += num_in_col;
        }
    }
    let (min_y, max_y) = galaxies
        .iter()
        .map(|pos| pos.y)
        .minmax()
        .into_option()
        .unwrap();

    let mut below = galaxies.len();
    let mut above = 0;

    for y in min_y..=max_y {
        let num_in_row = galaxies.iter().filter(|pos| pos.y == y).count();

        if num_in_row == 0 {
            total += 2 * below * above;
        } else {
            total += below * above;
            below -= num_in_row;
            above += num_in_row;
        }
    }

    total
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Position>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Position {
                            x: x as i64,
                            y: y as i64,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect())
    }

    fn solve(galaxies: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = get_total_lengths(&galaxies);
        (Some(part1.to_string()), None)
    }
}
