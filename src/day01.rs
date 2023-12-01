use failure::Error;

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<String>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data.lines().map(|line| line.to_string()).collect())
    }

    fn solve(lines: Self::Problem) -> (Option<String>, Option<String>) {
        let part1: u32 = lines
            .iter()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .map(|digits: Vec<u32>| (*digits.first().unwrap(), *digits.last().unwrap()))
            .map(|(x, y)| x * 10 + y)
            .sum();

        (Some(part1.to_string()), None)
    }
}
