use crate::parsers::unsigned;
use failure::{err_msg, Error};
use nom::{
    branch::alt,
    character::complete::{char, newline, space1},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

pub struct Line {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

fn can_have_group(springs: &[Spring], group_size: usize) -> bool {
    springs.len() >= group_size
        && springs[0..group_size]
            .iter()
            .all(|spring| *spring != Spring::Operational)
        && springs
            .get(group_size)
            .cloned()
            .unwrap_or(Spring::Operational)
            != Spring::Damaged
}

fn without_leading_group(springs: &[Spring], group_size: usize) -> Option<&[Spring]> {
    if !can_have_group(springs, group_size) {
        None
    } else if springs.len() == group_size {
        Some(&springs[group_size..])
    } else {
        Some(&springs[group_size + 1..])
    }
}

fn arrangements_for_seq(springs: &[Spring], groups: &[usize]) -> usize {
    if groups.is_empty() {
        if springs.iter().all(|spring| *spring != Spring::Damaged) {
            return 1;
        } else {
            return 0;
        }
    }

    if springs.len() < groups.iter().sum::<usize>() + groups.len() - 1 {
        return 0;
    }

    match springs[0] {
        Spring::Operational => arrangements_for_seq(&springs[1..], groups),
        Spring::Damaged => {
            if let Some(rem_springs) = without_leading_group(springs, groups[0]) {
                arrangements_for_seq(rem_springs, &groups[1..])
            } else {
                0
            }
        }
        Spring::Unknown => {
            arrangements_for_seq(&springs[1..], groups)
                + if let Some(rem_springs) = without_leading_group(springs, groups[0]) {
                    arrangements_for_seq(rem_springs, &groups[1..])
                } else {
                    0
                }
        }
    }
}

fn get_num_arragements(line: &Line) -> usize {
    arrangements_for_seq(&line.springs, &line.groups)
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Line>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let spring = alt((
            value(Spring::Operational, char('.')),
            value(Spring::Damaged, char('#')),
            value(Spring::Unknown, char('?')),
        ));

        let springs = many1(spring);

        let groups = separated_list1(char(','), unsigned);
        let line = map(
            terminated(separated_pair(springs, space1, groups), newline),
            |(springs, groups)| Line { springs, groups },
        );

        let lines = many1(line);

        all_consuming(lines)(&data)
            .map(|(_, lines)| lines)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve(lines: Self::Problem) -> (Option<String>, Option<String>) {
        let part1: usize = lines.iter().map(get_num_arragements).sum();
        (Some(part1.to_string()), None)
    }
}
