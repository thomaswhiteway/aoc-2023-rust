use failure::{err_msg, Error};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, newline};
use nom::combinator::{all_consuming, map};
use nom::multi::many1;
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;
use nom::{branch::alt, combinator::value};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub struct Location {
    name: String,
    left: String,
    right: String,
}

impl Location {
    fn get_next(&self, direction: Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Right, char('R')),
        value(Direction::Left, char('L')),
    ))(input)
}

fn path_length(
    locations: &HashMap<String, Location>,
    directions: &[Direction],
    from: &str,
    to: &str,
) -> usize {
    directions
        .iter()
        .cycle()
        .scan(from, |current, direction| {
            if *current == to {
                None
            } else {
                let location = locations.get(*current).unwrap();
                *current = location.get_next(*direction);
                Some(1)
            }
        })
        .count()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Vec<Direction>, HashMap<String, Location>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let directions = terminated(many1(direction), newline);

        let location = map(
            separated_pair(
                alpha1,
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(alpha1, tag(", "), alpha1),
                    tag(")"),
                ),
            ),
            |(name, (left, right)): (&str, (&str, &str))| Location {
                name: name.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            },
        );
        let locations = map(many1(terminated(location, newline)), |locs| {
            locs.into_iter()
                .map(|loc| (loc.name.clone(), loc))
                .collect()
        });

        all_consuming(separated_pair(directions, newline, locations))(&data)
            .map(|(_, res)| res)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve((directions, locations): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = path_length(&locations, &directions, "AAA", "ZZZ");
        (Some(part1.to_string()), None)
    }
}
