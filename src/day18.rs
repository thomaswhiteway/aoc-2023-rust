use failure::{err_msg, Error};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, newline, space1},
    combinator::{all_consuming, map, map_res, value},
    multi::many1,
    sequence::{delimited, separated_pair, terminated, tuple},
    AsChar,
};

use crate::{
    common::{Direction, Position},
    parsers::unsigned,
};
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    direction: Direction,
    length: u32,
}

fn find_route(instructions: &[Instruction]) -> Vec<Position> {
    [Position::origin()]
        .into_iter()
        .chain(
            instructions
                .iter()
                .scan(Position::origin(), |pos, instruction| {
                    *pos = pos.step_by(instruction.direction, instruction.length);
                    Some(*pos)
                }),
        )
        .collect()
}

fn find_area(route: &[Position]) -> i64 {
    let ys: Vec<_> = route.iter().map(|pos| pos.y).unique().sorted().collect();

    ys.iter()
        .tuple_windows()
        .flat_map(|(y1, y2)| [(*y1, 1), (*y1 + 1, y2 - y1 - 1)])
        .chain([(*ys.last().unwrap(), 1)])
        .flat_map(|(y, height)| {
            route
                .iter()
                .tuple_windows()
                .filter(|(start, end)| {
                    start.x == end.x && (start.y <= y && end.y >= y || start.y >= y && end.y <= y)
                })
                .map(|(start, end)| {
                    (
                        start.x,
                        start.y == y || end.y == y,
                        start.direction_to(end).unwrap(),
                    )
                })
                .sorted_by_key(|(x, _, _)| *x)
                .scan(
                    (false, None),
                    move |(inside, on_edge), (x, edge_corner, direction)| {
                        if !edge_corner {
                            *inside = !*inside;
                            Some(Some(x))
                        } else if let Some(prev_dir) = *on_edge {
                            if direction == prev_dir {
                                *inside = !*inside;
                            }

                            *on_edge = None;

                            if !*inside {
                                Some(Some(x))
                            } else {
                                Some(None)
                            }
                        } else {
                            *on_edge = Some(direction);
                            if !*inside {
                                Some(Some(x))
                            } else {
                                Some(None)
                            }
                        }
                    },
                )
                .flatten()
                .tuples()
                .map(|(x1, x2)| x2 - x1 + 1)
                .map(move |width| width * height)
        })
        .sum()
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<(Instruction, Instruction)>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let letter_direction = alt((
            value(Direction::North, char('U')),
            value(Direction::East, char('R')),
            value(Direction::South, char('D')),
            value(Direction::West, char('L')),
        ));

        let part1_instruction = map(
            tuple((letter_direction, space1, unsigned)),
            |(direction, _, length)| Instruction { direction, length },
        );

        let number_direction = alt((
            value(Direction::North, char('3')),
            value(Direction::East, char('0')),
            value(Direction::South, char('1')),
            value(Direction::West, char('2')),
        ));

        let part2_instruction = delimited(
            tag("(#"),
            map(
                tuple((
                    map_res(take_while_m_n(5, 5, |c: char| c.is_hex_digit()), |len| {
                        u32::from_str_radix(len, 16)
                    }),
                    number_direction,
                )),
                |(length, direction)| Instruction { length, direction },
            ),
            tag(")"),
        );

        let instruction_pair = terminated(
            separated_pair(part1_instruction, space1, part2_instruction),
            newline,
        );

        let instructions = many1(instruction_pair);

        all_consuming(instructions)(&data)
            .map(|(_, instructions)| instructions)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve(instructions: Self::Problem) -> (Option<String>, Option<String>) {
        let (part1_instructions, part2_instructions): (Vec<_>, Vec<_>) =
            instructions.iter().cloned().unzip();

        let part1 = find_area(&find_route(&part1_instructions));
        let part2 = find_area(&find_route(&part2_instructions));

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
