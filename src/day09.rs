use failure::{err_msg, Error};
use itertools::Itertools;

fn find_next_value(values: &[i64]) -> i64 {
    let mut stack: Vec<Vec<i64>> = vec![values.iter().cloned().collect()];
    while !stack.last().unwrap().iter().all(|val| *val == 0) {
        stack.push(
            stack
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(x, y)| y - x)
                .collect(),
        );
    }

    stack.last_mut().unwrap().push(0);

    for index in (0..stack.len() - 1).rev() {
        let new_val = stack[index].last().unwrap() + stack[index + 1].last().unwrap();
        stack[index].push(new_val);
    }

    *stack[0].last().unwrap()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Vec<i64>>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|val| {
                        val.parse()
                            .map_err(|err| err_msg(format!("Invalid number {}: {}", val, err)))
                    })
                    .collect()
            })
            .collect()
    }

    fn solve(series: Self::Problem) -> (Option<String>, Option<String>) {
        let part1: i64 = series.iter().map(|values| find_next_value(values)).sum();
        (Some(part1.to_string()), None)
    }
}
