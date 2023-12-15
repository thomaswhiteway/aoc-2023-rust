use std::{fmt::Display, str::FromStr};

use array_init::array_init;
use failure::{err_msg, Error};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{alpha1, char},
    combinator::{all_consuming, map, value},
    sequence::{preceded, tuple},
};

use crate::parsers::unsigned;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Insert(u32),
    Remove,
}

impl Operation {
    fn execute(self, lens_box: &mut Vec<Lens>, label: &str) {
        use Operation::*;
        match self {
            Insert(focal_length) => {
                if let Some(other_lens) = lens_box
                    .iter_mut()
                    .find(|other_lens| other_lens.label == label)
                {
                    other_lens.focal_length = focal_length;
                } else {
                    lens_box.push(Lens {
                        label: label.to_string(),
                        focal_length,
                    })
                }
            }
            Remove => lens_box.retain(|other_lens| other_lens.label != label),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lens {
    label: String,
    focal_length: u32,
}

impl Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    label: String,
    operation: Operation,
    hash: u8,
}

impl Instruction {
    fn apply(&self, lenses: &mut [Vec<Lens>; 256]) {
        self.operation
            .execute(&mut lenses[hash(&self.label) as usize], &self.label)
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation = alt((
            map(preceded(char('='), unsigned), Operation::Insert),
            value(Operation::Remove, char('-')),
        ));
        all_consuming(map(tuple((alpha1, operation)), |(label, operation)| {
            Instruction {
                label: label.to_string(),
                operation,
                hash: hash(s),
            }
        }))(s)
        .map(|(_, instruction)| instruction)
        .map_err(|err| err_msg(format!("Invalid Instruction: {}", err)))
    }
}

fn hash(data: &str) -> u8 {
    let mut val: u8 = 0;

    for c in data.chars() {
        val = val.wrapping_add(c as u8);
        val = val.wrapping_mul(17);
    }

    val
}

fn assemble_lenses(instructions: &[Instruction]) -> [Vec<Lens>; 256] {
    let mut lenses = array_init(|_| Vec::new());

    for instruction in instructions {
        instruction.apply(&mut lenses)
    }

    lenses
}

fn get_focussing_power(lenses: &[Vec<Lens>; 256]) -> u64 {
    (1..)
        .zip(lenses.iter())
        .flat_map(|(box_id, slots)| {
            (1..)
                .zip(slots.iter())
                .map(move |(slot_id, lens)| (box_id * slot_id * lens.focal_length) as u64)
        })
        .sum()
}

#[allow(unused)]
fn display_lenses(lenses: &[Vec<Lens>; 256]) {
    for (box_id, slots) in lenses.iter().enumerate() {
        if !slots.is_empty() {
            println!(
                "Box {}: {}",
                box_id,
                slots.iter().map(|lens| lens.to_string()).join(" ")
            )
        }
    }
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Instruction>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        data.trim().split(',').map(Instruction::from_str).collect()
    }

    fn solve(sequence: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = sequence
            .iter()
            .map(|instruction| instruction.hash as u64)
            .sum::<u64>();

        let lenses = assemble_lenses(&sequence);
        let part2 = get_focussing_power(&lenses);

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
