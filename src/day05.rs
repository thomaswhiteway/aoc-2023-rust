mod parse {
    use super::{Almanac, Map, MapRange};
    use crate::parsers::unsigned;
    use failure::{err_msg, Error};
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, newline, space1},
        combinator::{all_consuming, map},
        multi::{many1, separated_list1},
        sequence::{delimited, separated_pair, terminated, tuple},
    };

    pub fn parse_input(input: &str) -> Result<Almanac, Error> {
        let seeds = delimited(tag("seeds: "), separated_list1(space1, unsigned), newline);
        let map_range = map(
            tuple((unsigned, space1, unsigned, space1, unsigned)),
            |(dest_start, _, src_start, _, len)| MapRange {
                dest_start,
                src_start,
                len,
            },
        );
        let item_map = map(
            tuple((
                terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:\n")),
                many1(terminated(map_range, newline)),
            )),
            |((source, dest), ranges)| Map {
                source: source.to_string(),
                dest: dest.to_string(),
                ranges,
            },
        );
        let maps = map(separated_list1(newline, item_map), |maps| {
            maps.into_iter()
                .map(|map| (map.source.clone(), map))
                .collect()
        });
        let almanac = map(separated_pair(seeds, newline, maps), |(seeds, maps)| {
            Almanac { seeds, maps }
        });

        all_consuming(almanac)(input)
            .map(|(_, almanac)| almanac)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }
}

use failure::Error;
use parse::parse_input;
use std::collections::HashMap;

#[derive(Debug)]
struct MapRange {
    dest_start: u64,
    src_start: u64,
    len: u64,
}

impl MapRange {
    fn maybe_map_value(&self, value: u64) -> Option<u64> {
        if self.src_start <= value && value < self.src_start + self.len {
            Some(self.dest_start + (value - self.src_start))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    source: String,
    dest: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map_value(&self, value: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| range.maybe_map_value(value))
            .unwrap_or(value)
    }
}

pub struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}

impl Almanac {
    fn get_locations(&self) -> Vec<u64> {
        self.get_items(&self.seeds, "seed", "location")
    }

    fn get_items(
        &self,
        current_values: &[u64],
        current_type: &str,
        desired_type: &str,
    ) -> Vec<u64> {
        if current_type == desired_type {
            current_values.to_vec()
        } else {
            let map = self.maps.get(current_type).unwrap();
            let next_values: Vec<_> = current_values
                .iter()
                .map(|val| map.map_value(*val))
                .collect();
            self.get_items(&next_values, &map.dest, desired_type)
        }
    }
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Almanac;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve(almanac: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = almanac.get_locations().into_iter().min().unwrap();
        (Some(part1.to_string()), None)
    }
}
