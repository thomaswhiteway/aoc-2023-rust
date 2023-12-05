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
            |(dest_start, _, src_start, _, len)| MapRange::new(dest_start, src_start, len),
        );
        let item_map = map(
            tuple((
                terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:\n")),
                many1(terminated(map_range, newline)),
            )),
            |((source, dest), ranges)| Map::new(source, dest, ranges),
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
use std::cmp::{max, min};
use std::{collections::HashMap, ops::Range};

#[derive(Debug, PartialEq, Eq)]
struct RangeMapping {
    before: Option<Range<u64>>,
    mapped: Option<Range<u64>>,
    after: Option<Range<u64>>,
}

#[derive(Debug)]
struct MapRange {
    dest: Range<u64>,
    src: Range<u64>,
}

impl MapRange {
    fn new(dest_start: u64, src_start: u64, len: u64) -> Self {
        MapRange {
            dest: dest_start..dest_start + len,
            src: src_start..src_start + len,
        }
    }

    fn map_value(&self, value: u64) -> u64 {
        self.dest.start + (value - self.src.start)
    }

    fn map_range(&self, range: Range<u64>) -> RangeMapping {
        let before = if range.start < self.src.start {
            Some(range.start..min(range.end, self.src.start))
        } else {
            None
        };

        let map_start = max(range.start, self.src.start);
        let map_end = min(range.end, self.src.end);

        let mapped = if map_start < map_end {
            let mapped_start = self.map_value(map_start);
            let mapped_end = self.map_value(map_end);
            Some(mapped_start..mapped_end)
        } else {
            None
        };

        let after = if range.end > self.src.end {
            Some(max(range.start, self.src.end)..range.end)
        } else {
            None
        };

        assert!(before.is_some() || mapped.is_some() || after.is_some());

        RangeMapping {
            before,
            mapped,
            after,
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
    fn new(source: &str, dest: &str, mut ranges: Vec<MapRange>) -> Self {
        ranges.sort_by_key(|range| range.src.start);
        Map {
            source: source.to_string(),
            dest: dest.to_string(),
            ranges,
        }
    }

    fn map_range(&self, mut range: Range<u64>) -> Vec<Range<u64>> {
        let mut mapped_ranges = vec![];

        for map_range in &self.ranges {
            let mapping = map_range.map_range(range.clone());
            if let Some(before) = mapping.before {
                mapped_ranges.push(before);
            }
            if let Some(mapped) = mapping.mapped {
                mapped_ranges.push(mapped)
            }
            if let Some(after) = mapping.after {
                range = after;
            } else {
                break;
            }
        }

        mapped_ranges
    }
}

pub struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}

impl Almanac {
    fn get_seeds(&self, seed_ranges: bool) -> Vec<Range<u64>> {
        if seed_ranges {
            self.seeds
                .chunks(2)
                .map(|range| range[0]..range[0] + range[1])
                .collect()
        } else {
            self.seeds.iter().map(|&seed| seed..seed + 1).collect()
        }
    }

    fn get_closest_location(&self, seed_ranges: bool) -> u64 {
        let seeds = self.get_seeds(seed_ranges);
        self.get_locations(&seeds)
            .into_iter()
            .map(|range| range.start)
            .min()
            .unwrap()
    }

    fn get_locations(&self, seeds: &[Range<u64>]) -> Vec<Range<u64>> {
        self.get_items(seeds, "seed", "location")
    }

    fn get_items(
        &self,
        current_ranges: &[Range<u64>],
        current_type: &str,
        desired_type: &str,
    ) -> Vec<Range<u64>> {
        if current_type == desired_type {
            current_ranges.to_vec()
        } else {
            let map = self.maps.get(current_type).unwrap();
            let next_ranges: Vec<_> = current_ranges
                .iter()
                .flat_map(|range| map.map_range(range.clone()))
                .collect();
            self.get_items(&next_ranges, &map.dest, desired_type)
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
        let part1 = almanac.get_closest_location(false);
        let part2 = almanac.get_closest_location(true);
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
