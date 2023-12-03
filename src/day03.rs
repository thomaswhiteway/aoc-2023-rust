use crate::common::Position;
use failure::Error;
use std::collections::{HashMap, HashSet};

pub struct Solver {}

fn find_numbers(grid: HashMap<Position, char>) -> Vec<(u64, Vec<Position>)> {
    let max_x = grid.keys().map(|pos| pos.x).max().unwrap();
    let max_y = grid.keys().map(|pos| pos.y).max().unwrap();

    let mut numbers = vec![];

    let mut current_number = 0;
    let mut current_positions = vec![];

    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = Position { x, y };
            let c = grid.get(&pos).unwrap();
            if let Some(digit) = c.to_digit(10) {
                current_number = digit as u64 + current_number * 10;
                current_positions.push(pos);
            } else if !current_positions.is_empty() {
                numbers.push((current_number, current_positions));
                current_number = 0;
                current_positions = vec![];
            }
        }

        if !current_positions.is_empty() {
            numbers.push((current_number, current_positions));
            current_number = 0;
            current_positions = vec![];
        }
    }

    numbers
}

fn find_positions_near_symbols(grid: &HashMap<Position, char>) -> HashSet<Position> {
    grid.iter()
        .filter_map(|(pos, c)| {
            if !c.is_digit(10) && *c != '.' {
                Some(pos)
            } else {
                None
            }
        })
        .flat_map(|pos| pos.surrounding())
        .collect()
}

fn is_part_number(pos: &Position, near_symbols: &HashSet<Position>) -> bool {
    near_symbols.contains(pos)
}

fn find_part_numbers(grid: HashMap<Position, char>) -> Vec<u64> {
    let near_symbols = find_positions_near_symbols(&grid);
    find_numbers(grid)
        .into_iter()
        .filter_map(|(num, positions)| {
            if positions
                .iter()
                .any(|pos| is_part_number(pos, &near_symbols))
            {
                Some(num)
            } else {
                None
            }
        })
        .collect()
}

impl super::Solver for Solver {
    type Problem = HashMap<Position, char>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        Position {
                            x: x as i64,
                            y: y as i64,
                        },
                        c,
                    )
                })
            })
            .collect())
    }

    fn solve(grid: Self::Problem) -> (Option<String>, Option<String>) {
        let part_one: u64 = find_part_numbers(grid).iter().sum();
        (Some(part_one.to_string()), None)
    }
}
