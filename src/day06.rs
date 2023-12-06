use crate::parsers::unsigned;
use failure::{err_msg, Error};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1},
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

fn number_list<'a, 'b>(name: &'a str) -> impl FnMut(&'b str) -> IResult<&'b str, Vec<u64>> + 'a
where
    'b: 'a,
{
    delimited(tuple((tag(name), tag(":"), space1)), numbers, newline)
}

fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, unsigned)(input)
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<Race>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let times = number_list("Time");
        let distances = number_list("Distance");
        let races = map(tuple((times, distances)), |(ts, ds)| {
            ts.into_iter()
                .zip(ds)
                .map(|(time, distance)| Race { time, distance })
                .collect()
        });
        all_consuming(races)(&data)
            .map(|(_, races)| races)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }

    fn solve(races: Self::Problem) -> (Option<String>, Option<String>) {
        let part1: u64 = races.iter().map(|race| race.ways_to_win()).product();
        (Some(part1.to_string()), None)
    }
}
