use std::collections::HashSet;

use failure::{err_msg, Error};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, hex_digit1, newline, space1},
    combinator::{all_consuming, map, value},
    multi::many1,
    sequence::{delimited, tuple},
};

use crate::{
    common::{Direction, Position},
    parsers::unsigned,
};

pub struct Instruction {
    direction: Direction,
    length: u32,
}

fn dig_trench(instructions: &[Instruction]) -> HashSet<Position> {
    [Position::origin()]
        .into_iter()
        .chain(
            instructions
                .iter()
                .scan(Position::origin(), |pos, instruction| {
                    Some(
                        (0..instruction.length)
                            .scan(pos, |pos, _steps| {
                                **pos = pos.step(instruction.direction);
                                Some(**pos)
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .flatten(),
        )
        .collect()
}

fn find_point_inside(trench: &HashSet<Position>) -> Position {
    let (min_x, max_x) = trench
        .iter()
        .map(|pos| pos.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = trench
        .iter()
        .map(|pos| pos.y)
        .minmax()
        .into_option()
        .unwrap();

    let mid_y = (min_y + max_y) / 2;

    let mut position = Position { x: min_x, y: mid_y };
    let mut on_trench = false;
    let mut row_started_north = false;
    loop {
        assert!(position.x <= max_x);

        match (on_trench, trench.contains(&position)) {
            (false, true) => {
                on_trench = true;
                row_started_north = trench.contains(&position.step(Direction::North));
            }
            (true, false) => {
                let row_turned_north =
                    trench.contains(&position.step(Direction::West).step(Direction::North));

                if (row_started_north && !row_turned_north)
                    || (!row_started_north && row_turned_north)
                {
                    return position;
                }
            }
            _ => {}
        }

        position = position.step(Direction::East);
    }
}

fn fill(hole: &mut HashSet<Position>, start: Position) {
    let mut to_visit = vec![start];

    while let Some(position) = to_visit.pop() {
        if !hole.contains(&position) {
            hole.insert(position);
            to_visit.extend(position.adjacent());
        }
    }
}

fn dig_out_middle(mut trench: HashSet<Position>) -> HashSet<Position> {
    let inside_point = find_point_inside(&trench);

    fill(&mut trench, inside_point);

    trench
}

fn display_hole(hole: &HashSet<Position>) {
    let (min_x, max_x) = hole.iter().map(|pos| pos.x).minmax().into_option().unwrap();
    let (min_y, max_y) = hole.iter().map(|pos| pos.y).minmax().into_option().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let position = Position { x, y };
            if hole.contains(&position) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Instruction>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let direction = alt((
            value(Direction::North, char('U')),
            value(Direction::East, char('R')),
            value(Direction::South, char('D')),
            value(Direction::West, char('L')),
        ));

        let colour = delimited(tag("(#"), hex_digit1, tag(")"));

        let instruction = map(
            tuple((direction, space1, unsigned, space1, colour, newline)),
            |(direction, _, length, _, _, _)| Instruction { direction, length },
        );

        let instructions = many1(instruction);

        all_consuming(instructions)(&data)
            .map(|(_, instructions)| instructions)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve(instructions: Self::Problem) -> (Option<String>, Option<String>) {
        let trench = dig_trench(&instructions);
        let hole = dig_out_middle(trench);
        display_hole(&hole);

        let part1 = hole.len();
        (Some(part1.to_string()), None)
    }
}
