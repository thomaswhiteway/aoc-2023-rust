use failure::Error;

fn hash(data: &str) -> u8 {
    let mut val: u8 = 0;

    for c in data.chars() {
        val = val.wrapping_add(c as u8);
        val = val.wrapping_mul(17);
    }

    val
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Vec<String>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        Ok(data.trim().split(',').map(str::to_string).collect())
    }

    fn solve(sequence: Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = sequence.iter().map(|instruction| hash(instruction) as u64).sum::<u64>();

        (Some(part1.to_string()), None)
    }
}
