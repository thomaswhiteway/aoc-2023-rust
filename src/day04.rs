use std::collections::HashSet;

use crate::parsers::unsigned;
use failure::{err_msg, Error};
use nom::bytes::complete::tag;
use nom::character::complete::{newline, space0, space1};
use nom::combinator::{all_consuming, map};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;

fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(space0, separated_list1(space1, unsigned))(input)
}

pub struct Card {
    winning_numbers: Vec<u64>,
    card_numbers: Vec<u64>,
}

impl Card {
    fn score(&self) -> u64 {
        let num_common = self.num_winning_numbers();
        if num_common > 0 {
            2_u64.pow(num_common as u32 - 1)
        } else {
            0
        }
    }

    fn num_winning_numbers(&self) -> usize {
        let winning_numbers: HashSet<_> = self.winning_numbers.iter().cloned().collect();
        let card_numbers: HashSet<_> = self.card_numbers.iter().cloned().collect();
        winning_numbers.intersection(&card_numbers).count()
    }
}

fn copies_of_scratchcards(cards: &[Card]) -> Vec<usize> {
    let mut num_copies: Vec<usize> = cards.iter().map(|_| 1).collect();

    for (index, card) in cards.iter().enumerate() {
        for offset in 1..=card.num_winning_numbers() {
            num_copies[index + offset] += num_copies[index];
        }
    }

    num_copies
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Card>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let card = map(
            preceded(
                tuple((tag("Card"), space1, unsigned::<u64>, tag(": "))),
                separated_pair(numbers, tag(" | "), numbers),
            ),
            |(winning_numbers, card_numbers)| Card {
                winning_numbers,
                card_numbers,
            },
        );

        all_consuming(many1(terminated(card, newline)))(&data)
            .map(|(_, cards)| cards)
            .map_err(|err| err_msg(format!("Failed to parse cards: {}", err)))
    }

    fn solve(cards: Self::Problem) -> (Option<String>, Option<String>) {
        let part1: u64 = cards.iter().map(|card| card.score()).sum();
        let part2: usize = copies_of_scratchcards(&cards).iter().sum();
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
