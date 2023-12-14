use failure::Error;
use std::str::FromStr;

fn find_reflection<T: Eq>(entries: &[Vec<T>], num_change: usize) -> Option<usize> {
    let mut before: Vec<&Vec<T>> = vec![];
    let mut after: Vec<&Vec<T>> = entries.iter().rev().collect();

    while after.len() > 1 {
        before.push(after.pop().unwrap());

        let num_different: usize = before
            .iter()
            .rev()
            .zip(after.iter().rev())
            .map(|(before_row, after_row)| {
                before_row
                    .iter()
                    .zip(after_row.iter())
                    .filter(|(b, a)| b != a)
                    .count()
            })
            .sum();

        if num_different == num_change {
            return Some(before.len());
        }
    }

    None
}

pub struct Grid {
    rows: Vec<Vec<bool>>,
}

impl Grid {
    fn cols_before_reflection(&self, num_change: usize) -> Option<usize> {
        let cols: Vec<_> = (0..self.rows[0].len())
            .map(|y| self.rows.iter().map(|row| row[y]).collect())
            .collect();
        find_reflection(&cols, num_change)
    }

    fn rows_before_reflection(&self, num_change: usize) -> Option<usize> {
        find_reflection(&self.rows, num_change)
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Ok(Grid { rows })
    }
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Grid>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.split("\n\n").map(|input| input.parse()).collect()
    }

    fn solve(grids: Self::Problem) -> (Option<String>, Option<String>) {
        let part1: usize = grids
            .iter()
            .map(|grid| {
                grid.cols_before_reflection(0)
                    .or_else(|| grid.rows_before_reflection(0).map(|rows| 100 * rows))
                    .unwrap()
            })
            .sum();
        let part2: usize = grids
            .iter()
            .map(|grid| {
                grid.cols_before_reflection(1)
                    .or_else(|| grid.rows_before_reflection(1).map(|rows| 100 * rows))
                    .unwrap()
            })
            .sum();

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
