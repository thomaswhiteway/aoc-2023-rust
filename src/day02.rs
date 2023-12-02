mod parse {
    use failure::{err_msg, Error};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::newline,
        combinator::{all_consuming, map, value},
        multi::{many1, separated_list1},
        sequence::{preceded, separated_pair, terminated, tuple},
        IResult,
    };

    use crate::parsers::unsigned;

    use super::Colour;

    fn colour(input: &str) -> IResult<&str, Colour> {
        alt((
            value(Colour::Blue, tag("blue")),
            value(Colour::Red, tag("red")),
            value(Colour::Green, tag("green")),
        ))(input)
    }

    fn amount(input: &str) -> IResult<&str, (usize, Colour)> {
        separated_pair(unsigned, tag(" "), colour)(input)
    }

    fn round(input: &str) -> IResult<&str, [usize; 3]> {
        map(separated_list1(tag(", "), amount), |amounts| {
            let mut result = [0; 3];
            for (num, colour) in amounts {
                result[colour as usize] += num;
            }
            result
        })(input)
    }

    fn game(input: &str) -> IResult<&str, Vec<[usize; 3]>> {
        preceded(
            tuple((tag("Game "), unsigned::<usize>, tag(": "))),
            separated_list1(tag("; "), round),
        )(input)
    }

    fn games(input: &str) -> IResult<&str, Vec<Vec<[usize; 3]>>> {
        many1(terminated(game, newline))(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Vec<[usize; 3]>>, Error> {
        all_consuming(games)(input)
            .map(|(_, games)| games)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }
}

use failure::Error;
use parse::parse_input;
use std::cmp::max;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Colour {
    Red,
    Green,
    Blue,
}

fn is_round_possible(round: &[usize; 3], candidate: &[usize; 3]) -> bool {
    round.iter().zip(candidate.iter()).all(|(x, y)| x <= y)
}

fn is_game_possible(rounds: &[[usize; 3]], candidate: &[usize; 3]) -> bool {
    rounds
        .iter()
        .all(|round| is_round_possible(round, candidate))
}

fn game_min_cubes(rounds: &Vec<[usize; 3]>) -> [usize; 3] {
    rounds
        .iter()
        .fold(vec![0, 0, 0], |current, round| {
            current
                .iter()
                .zip(round.iter())
                .map(|(&c, &r)| max(c, r))
                .collect::<Vec<_>>()
        })
        .try_into()
        .unwrap()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Vec<[usize; 3]>>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve(games: Self::Problem) -> (Option<String>, Option<String>) {
        let candidate = [12, 13, 14];

        let part1: usize = (1..)
            .zip(games.iter())
            .filter_map(|(game_id, game)| {
                if is_game_possible(&game, &candidate) {
                    Some(game_id)
                } else {
                    None
                }
            })
            .sum();

        let part2: usize = games
            .iter()
            .map(game_min_cubes)
            .map(|min_cubes| min_cubes.iter().product::<usize>())
            .sum();

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
