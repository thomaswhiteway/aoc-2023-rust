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
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    cards: Vec<u8>,
    bid: u64,
}

fn is_joker(card: u8) -> bool {
    card == 0
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let num_jokers = self.cards.iter().filter(|card| is_joker(**card)).count();

        let mut counts: Vec<_> = self
            .cards
            .iter()
            .filter(|card| !is_joker(**card))
            .counts()
            .into_values()
            .collect();
        counts.sort_by(|x, y| x.cmp(y).reverse());

        if !counts.is_empty() {
            counts[0] += num_jokers;
        } else {
            counts.push(num_jokers);
        }

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

    fn with_jokers(&self) -> Self {
        let cards = self
            .cards
            .iter()
            .map(|&card| if card == 11 { 0 } else { card })
            .collect();
        Hand {
            cards,
            bid: self.bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then(self.cards.cmp(&other.cards))
    }
}

fn find_total_winnings(hands: &[Hand], jokers: bool) -> u64 {
    let mut hands: Vec<_> = hands
        .iter()
        .map(|hand| {
            if jokers {
                hand.with_jokers()
            } else {
                hand.clone()
            }
        })
        .collect();
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
        let part1 = find_total_winnings(&hands, false);
        let part2 = find_total_winnings(&hands, true);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
