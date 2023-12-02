use failure::Error;

pub struct Solver {}

fn get_digits_part1(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn get_digits_part2(line: &str) -> Vec<u32> {
    let mut digits = vec![];
    for index in 0..line.len() {
        let substr = &line[index..];
        if let Some(digit) = substr.chars().next().unwrap().to_digit(10) {
            digits.push(digit);
        } else if substr.starts_with("one") {
            digits.push(1);
        } else if substr.starts_with("two") {
            digits.push(2);
        } else if substr.starts_with("three") {
            digits.push(3);
        } else if substr.starts_with("four") {
            digits.push(4);
        } else if substr.starts_with("five") {
            digits.push(5);
        } else if substr.starts_with("six") {
            digits.push(6);
        } else if substr.starts_with("seven") {
            digits.push(7);
        } else if substr.starts_with("eight") {
            digits.push(8);
        } else if substr.starts_with("nine") {
            digits.push(9);
        }
    }
    digits
}

fn solve<F>(lines: &[String], get_digits: F) -> u32
where
    F: Fn(&str) -> Vec<u32>,
{
    lines
        .iter()
        .map(|line| get_digits(line))
        .map(|digits: Vec<u32>| (*digits.first().unwrap(), *digits.last().unwrap()))
        .map(|(x, y)| x * 10 + y)
        .sum()
}

impl super::Solver for Solver {
    type Problem = Vec<String>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data.lines().map(|line| line.to_string()).collect())
    }

    fn solve(lines: Self::Problem) -> (Option<String>, Option<String>) {
        let part1: u32 = solve(&lines, get_digits_part1);
        let part2: u32 = solve(&lines, get_digits_part2);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
