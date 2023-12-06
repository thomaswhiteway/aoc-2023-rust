use crate::parsers::unsigned;
use failure::{err_msg, Error};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::delimited,
    sequence::tuple,
    IResult,
};

pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        // If t is the time for the race, and x is the time the button is
        // pressed then the boat moves for (t - x) ms at a speed of x mm/ms
        // covering a distance of (t - x) * x.
        //
        // If d is the distance to beat then we need
        // > (t - x) * x > d
        // which is equivalent to
        // > -x^2 + tx - d > 0
        //
        // Use the quadratic formula (https://en.wikipedia.org/wiki/Quadratic_formula)
        // to find the roots with:
        // > a = -1
        // > b = t
        // > c = -d
        let discriminant = (self.time as f64).powi(2) - 4.0 * (-1.0) * (-(self.distance as f64));
        if discriminant < 0.0 {
            return 0;
        }

        // The roots are given by
        // > (-t ± sqrt(discriminant)) / (2.0 * (-1.0))
        // or
        // > (t ± sqrt(discriminant)) / 2.0
        let lower = ((self.time as f64) - discriminant.sqrt()) / 2.0;
        let upper = ((self.time as f64) + discriminant.sqrt()) / 2.0;

        // Need to find the number of integers > lower and < upper.
        let min_solution = (lower + 1.0).floor() as u64;
        let max_solution = (upper - 1.0).ceil() as u64;

        if max_solution >= min_solution {
            max_solution - min_solution + 1
        } else {
            0
        }
    }
}

fn named_value<'a, 'b, F, A>(
    name: &'a str,
    value_parser: F,
) -> impl FnMut(&'b str) -> IResult<&'b str, A> + 'a
where
    F: FnMut(&str) -> IResult<&str, A> + 'static,
    A: 'static,
    'b: 'a,
{
    delimited(tuple((tag(name), tag(":"), space1)), value_parser, newline)
}

fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, unsigned)(input)
}

fn parse_small_races(data: &str) -> Result<Vec<Race>, Error> {
    let times = named_value("Time", numbers);
    let distances = named_value("Distance", numbers);
    let races = map(tuple((times, distances)), |(ts, ds)| {
        ts.into_iter()
            .zip(ds)
            .map(|(time, distance)| Race { time, distance })
            .collect()
    });
    all_consuming(races)(data)
        .map(|(_, races)| races)
        .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
}

fn distributed_number(input: &str) -> IResult<&str, u64> {
    map(separated_list1(space1, digit1), |segments| {
        segments.join("").parse().unwrap()
    })(input)
}

fn parse_big_race(data: &str) -> Result<Race, Error> {
    let time = named_value("Time", distributed_number);
    let distance = named_value("Distance", distributed_number);
    let race = map(tuple((time, distance)), |(time, distance)| Race {
        time,
        distance,
    });
    all_consuming(race)(data)
        .map(|(_, race)| race)
        .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Vec<Race>, Race);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let small_races = parse_small_races(&data)?;
        let big_race = parse_big_race(&data)?;
        Ok((small_races, big_race))
    }

    fn solve((small_races, big_race): Self::Problem) -> (Option<String>, Option<String>) {
        let part1: u64 = small_races.iter().map(|race| race.ways_to_win()).product();
        let part2: u64 = big_race.ways_to_win();
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
