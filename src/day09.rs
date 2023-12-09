use failure::{err_msg, Error};
use itertools::Itertools;

fn find_prev_next_value(values: &[i64]) -> (i64, i64) {
    let mut stack: Vec<Vec<i64>> = vec![values.to_vec()];
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
    stack.last_mut().unwrap().insert(0, 0);

    for index in (0..stack.len() - 1).rev() {
        let start_val = stack[index].first().unwrap() - stack[index + 1].first().unwrap();
        stack[index].insert(0, start_val);
        let end_val = stack[index].last().unwrap() + stack[index + 1].last().unwrap();
        stack[index].push(end_val);
    }

    (*stack[0].first().unwrap(), *stack[0].last().unwrap())
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
        let (part2, part1) = series
            .iter()
            .map(|values| find_prev_next_value(values))
            .fold((0, 0), |(tot_x, tot_y), (x, y)| (tot_x + x, tot_y + y));
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
