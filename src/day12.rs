use crate::parsers::unsigned;
use failure::{err_msg, Error};
use itertools::intersperse;
use nom::{
    branch::alt,
    character::complete::{char, newline, space1},
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};
use std::cmp::min;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
pub struct Line {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Line {
    fn unfold(&self) -> Self {
        let springs = intersperse((0..5).map(|_| self.springs.clone()), vec![Spring::Unknown])
            .flatten()
            .collect();
        let groups = self
            .groups
            .iter()
            .cloned()
            .cycle()
            .take(self.groups.len() * 5)
            .collect();
        Line { springs, groups }
    }
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

fn group_match_len(springs: &[Spring], group_size: usize) -> Option<usize> {
    if !can_have_group(springs, group_size) {
        None
    } else {
        Some(min(springs.len(), group_size + 1))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    spring_offset: usize,
    group_offset: usize,
    combinations: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.spring_offset
            .cmp(&other.spring_offset)
            .then(self.group_offset.cmp(&other.group_offset))
    }
}

fn get_num_arragements(line: &Line) -> usize {
    let mut candidates = vec![State {
        spring_offset: 0,
        group_offset: 0,
        combinations: 1,
    }];

    let mut total = 0;

    while !candidates.is_empty() {
        let spring_offset = candidates[0].spring_offset;

        let group_offset = candidates[0].group_offset;
        let num_to_process = candidates
            .iter()
            .take_while(|candidate| {
                candidate.spring_offset == spring_offset && candidate.group_offset == group_offset
            })
            .count();

        let combinations = candidates
            .drain(0..num_to_process)
            .map(|candidate| candidate.combinations)
            .sum();

        let springs = &line.springs[spring_offset..];
        let groups = &line.groups[group_offset..];

        if groups.is_empty() {
            if springs.iter().all(|spring| *spring != Spring::Damaged) {
                total += combinations;
            }

            continue;
        }

        if springs.len() < groups.iter().sum::<usize>() + groups.len() - 1 {
            continue;
        }

        if springs[0] != Spring::Damaged {
            candidates.insert(
                0,
                State {
                    spring_offset: spring_offset + 1,
                    group_offset,
                    combinations,
                },
            );
        }

        if springs[0] != Spring::Operational {
            if let Some(match_len) = group_match_len(springs, groups[0]) {
                candidates.insert(
                    0,
                    State {
                        spring_offset: spring_offset + match_len,
                        group_offset: group_offset + 1,
                        combinations,
                    },
                );
            }
        }

        candidates.sort();
    }

    total
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
        let unfolded_lines: Vec<_> = lines.iter().map(Line::unfold).collect();
        let part2: usize = unfolded_lines.iter().map(get_num_arragements).sum();
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
