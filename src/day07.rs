use failure::{err_msg, Error};
use itertools::Itertools;
use nom::{
    character::complete::{anychar, newline, space1},
    combinator::{all_consuming, map, map_res},
    multi::{many1, many_m_n},
    sequence::{separated_pair, terminated},
};

use crate::parsers::unsigned;

#[derive(PartialEq, Eq, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    cards: Vec<u8>,
    bid: u64,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts: Vec<_> = self.cards.iter().counts().into_values().collect();
        counts.sort_by(|x, y| x.cmp(y).reverse());

        use HandType::*;
        if counts[0] == 5 {
            FiveOfAKind
        } else if counts[0] == 4 {
            FourOfAKind
        } else if counts[0] == 3 && counts[1] == 2 {
            FullHouse
        } else if counts[0] == 3 {
            ThreeOfAKind
        } else if counts[0] == 2 && counts[1] == 2 {
            TwoPair
        } else if counts[0] == 2 {
            OnePair
        } else {
            HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.hand_type()
                .cmp(&other.hand_type())
                .then(self.cards.cmp(&other.cards)),
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn find_total_winnings(hands: &[Hand]) -> u64 {
    let mut hands = hands.to_vec();
    hands.sort();

    (1..).zip(hands).map(|(rank, hand)| hand.bid * rank).sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Hand>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let card = map_res(anychar, |c| match c {
            'A' => Ok(14),
            'K' => Ok(13),
            'Q' => Ok(12),
            'J' => Ok(11),
            'T' => Ok(10),
            c => match c.to_digit(10) {
                Some(d) if d != 0 => Ok(d as u8),
                _ => Err(format!("Invalid character for card: {}", c)),
            },
        });
        let cards = many_m_n(5, 5, card);
        let hand = map(
            terminated(separated_pair(cards, space1, unsigned), newline),
            |(cards, bid)| Hand { cards, bid },
        );

        all_consuming(many1(hand))(&data)
            .map(|(_, hand_bids)| hand_bids)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve(hands: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_total_winnings(&hands);
        (Some(part1.to_string()), None)
    }
}
