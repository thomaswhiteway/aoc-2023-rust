use failure::{err_msg, Error};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, newline};
use nom::combinator::{all_consuming, map};
use nom::multi::many1;
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;
use nom::{branch::alt, combinator::value};
use num::integer::lcm;
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

fn path<'a>(
    locations: &'a HashMap<String, Location>,
    directions: &'a [Direction],
    from: &'a str,
) -> impl Iterator<Item = &'a str> + 'a {
    directions.iter().cycle().scan(from, |current, direction| {
        let location = locations.get(*current).unwrap();
        *current = location.get_next(*direction);
        Some(*current)
    })
}

fn path_length(
    locations: &HashMap<String, Location>,
    directions: &[Direction],
    from: &str,
    to: &str,
) -> usize {
    path(locations, directions, from)
        .take_while(|loc| *loc != to)
        .count()
        + 1
}

fn find_cycle<'a>(dir_cycle: usize, path: impl Iterator<Item = &'a str>) -> (usize, usize) {
    let mut visited: HashMap<&str, Vec<usize>> = HashMap::new();

    for (distance, current) in (1..).zip(path) {
        let prev = visited.entry(current).or_default();
        if let Some(prev_dist) = prev.iter().find(|d| (distance - **d) % dir_cycle == 0) {
            return (*prev_dist, distance - prev_dist);
        }

        prev.push(distance)
    }

    unreachable!()
}

fn find_end_offset<'a, E>(
    cycle_start: usize,
    cycle_len: usize,
    path: impl Iterator<Item = &'a str>,
    end_filter: E,
) -> usize
where
    E: Fn(&str) -> bool,
{
    let mut offsets = (1..)
        .zip(path.skip(cycle_start))
        .take(cycle_len)
        .filter_map(|(offset, current)| {
            if end_filter(current) {
                Some(offset)
            } else {
                None
            }
        });

    let offset = offsets.next().unwrap();
    assert!(offsets.next().is_none());

    offset
}

fn ghost_path_length<S, E>(
    locations: &HashMap<String, Location>,
    directions: &[Direction],
    start_filter: &S,
    end_filter: &E,
) -> usize
where
    S: Fn(&str) -> bool,
    E: Fn(&str) -> bool,
{
    let starts: Vec<_> = locations.keys().filter(|name| start_filter(name)).collect();

    let cycle_lengths: Vec<_> = starts
        .iter()
        .map(|start| find_cycle(directions.len(), path(locations, directions, start)))
        .collect();
    let offsets: Vec<_> = starts
        .iter()
        .zip(cycle_lengths.iter())
        .map(|(start, (cycle_start, cycle_len))| {
            find_end_offset(
                *cycle_start,
                *cycle_len,
                path(locations, directions, start),
                end_filter,
            )
        })
        .collect();

    cycle_lengths
        .iter()
        .zip(offsets.iter())
        .map(|((init, cycle_len), end_offset)| (init + end_offset, *cycle_len))
        .reduce(|(offset1, cycle_len1), (offset2, cycle_len2)| {
            let cycle_len = lcm(cycle_len1, cycle_len2);
            let offset = ((0..).find(|n| {
                n * cycle_len1 + offset1 > offset2
                    && (n * cycle_len1 + offset1 - offset2) % cycle_len2 == 0
            }))
            .map(|n| n * cycle_len1 + offset1)
            .unwrap();
            (offset, cycle_len)
        })
        .map(|(offset, _)| offset)
        .unwrap()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Vec<Direction>, HashMap<String, Location>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let directions = terminated(many1(direction), newline);

        let location = map(
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
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
        let part2 = ghost_path_length(
            &locations,
            &directions,
            &|name| name.ends_with('A'),
            &|name| name.ends_with('Z'),
        );
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
