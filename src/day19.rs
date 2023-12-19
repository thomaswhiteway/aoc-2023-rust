mod parse {
    use failure::{err_msg, Error};
    use nom::{
        branch::alt,
        bytes::complete::take_while1,
        character::complete::{char, newline},
        combinator::{all_consuming, map, opt, value},
        multi::{many1, separated_list1},
        sequence::{delimited, separated_pair, terminated, tuple},
        IResult,
    };
    use std::collections::HashMap;

    use crate::parsers::unsigned;

    use super::{Category, Comparison, Condition, Outcome, Part, Rule, Workflow};

    fn workflow_name(input: &str) -> IResult<&str, String> {
        map(
            take_while1(|c: char| c.is_ascii_lowercase()),
            |name: &str| name.to_string(),
        )(input)
    }

    fn category(input: &str) -> IResult<&str, Category> {
        alt((
            value(Category::Cool, char('x')),
            value(Category::Musical, char('m')),
            value(Category::Aerodynamic, char('a')),
            value(Category::Shiny, char('s')),
        ))(input)
    }

    fn comparison(input: &str) -> IResult<&str, Comparison> {
        alt((
            value(Comparison::LessThan, char('<')),
            value(Comparison::MoreThan, char('>')),
        ))(input)
    }

    fn condition(input: &str) -> IResult<&str, Condition> {
        map(
            tuple((category, comparison, unsigned)),
            |(category, comparison, value)| Condition {
                category,
                comparison,
                value,
            },
        )(input)
    }

    fn outcome(input: &str) -> IResult<&str, Outcome> {
        alt((
            value(Outcome::Accept, char('A')),
            value(Outcome::Reject, char('R')),
            map(workflow_name, Outcome::Jump),
        ))(input)
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        map(
            tuple((opt(terminated(condition, char(':'))), outcome)),
            |(condition, outcome)| Rule { condition, outcome },
        )(input)
    }

    fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
        separated_list1(char(','), rule)(input)
    }

    fn workflow(input: &str) -> IResult<&str, Workflow> {
        map(
            tuple((workflow_name, delimited(char('{'), rules, char('}')))),
            |(name, rules)| Workflow { name, rules },
        )(input)
    }

    fn workflows(input: &str) -> IResult<&str, HashMap<String, Workflow>> {
        map(many1(terminated(workflow, newline)), |workflows| {
            workflows
                .into_iter()
                .map(|workflow| (workflow.name.clone(), workflow))
                .collect()
        })(input)
    }

    fn assignment(input: &str) -> IResult<&str, (Category, u64)> {
        separated_pair(category, char('='), unsigned)(input)
    }

    fn part(input: &str) -> IResult<&str, Part> {
        map(
            delimited(char('{'), separated_list1(char(','), assignment), char('}')),
            |assignments| {
                assignments
                    .into_iter()
                    .fold(Part::default(), |part, (category, value)| {
                        part.update(category, value)
                    })
            },
        )(input)
    }

    fn parts(input: &str) -> IResult<&str, Vec<Part>> {
        many1(terminated(part, newline))(input)
    }

    pub(super) fn parse_input(
        input: &str,
    ) -> Result<(HashMap<String, Workflow>, Vec<Part>), Error> {
        all_consuming(separated_pair(workflows, newline, parts))(input)
            .map(|(_, (workflows, parts))| (workflows, parts))
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
    }
}

use std::collections::HashMap;

use failure::Error;
use parse::parse_input;
use std::{
    cmp::{max, min},
    ops::Range,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparison {
    LessThan,
    MoreThan,
}

impl Comparison {
    fn apply(self, left: u64, right: u64) -> bool {
        use Comparison::*;
        match self {
            LessThan => left < right,
            MoreThan => left > right,
        }
    }

    fn split(self, range: &Range<u64>, value: u64) -> (Option<Range<u64>>, Option<Range<u64>>) {
        use Comparison::*;
        match self {
            LessThan => {
                let matched: Option<Range<u64>> = if range.start < value {
                    Some(range.start..min(value, range.end))
                } else {
                    None
                };
                let unmatched = if range.end > value {
                    Some(max(value, range.start)..range.end)
                } else {
                    None
                };
                (matched, unmatched)
            }
            MoreThan => {
                let matched: Option<Range<u64>> = if range.end > value + 1 {
                    Some(max(range.start, value + 1)..range.end)
                } else {
                    None
                };
                let unmatched = if range.start <= value {
                    Some(range.start..min(value + 1, range.end))
                } else {
                    None
                };
                (matched, unmatched)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Outcome {
    Accept,
    Reject,
    Jump(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Condition {
    category: Category,
    comparison: Comparison,
    value: u64,
}

impl Condition {
    fn matches(&self, part: &Part) -> bool {
        self.comparison.apply(part.value(self.category), self.value)
    }

    fn split(&self, range: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        let category_range = range.category_range(self.category);
        let (matched, unmatched) = self.comparison.split(category_range, self.value);
        (
            matched.map(|matched_range| range.update(self.category, matched_range)),
            unmatched.map(|unmatched_range| range.update(self.category, unmatched_range)),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    condition: Option<Condition>,
    outcome: Outcome,
}

impl Rule {
    fn get_outcome(&self, part: &Part) -> Option<&Outcome> {
        if self
            .condition
            .map(|condition| condition.matches(part))
            .unwrap_or(true)
        {
            Some(&self.outcome)
        } else {
            None
        }
    }

    fn split(&self, range: &PartRange) -> (Option<(PartRange, &Outcome)>, Option<PartRange>) {
        if let Some(condition) = self.condition {
            let (matches, doesnt_match) = condition.split(range);
            (
                matches.map(|matches| (matches, &self.outcome)),
                doesnt_match,
            )
        } else {
            (Some((range.clone(), &self.outcome)), None)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn outcome(&self, part: &Part) -> Option<&Outcome> {
        self.rules.iter().find_map(|rule| rule.get_outcome(part))
    }

    fn split(&self, range: PartRange) -> impl Iterator<Item = (PartRange, &Outcome)> + '_ {
        self.rules
            .iter()
            .scan(Some(range), |current_range, rule| {
                if let Some(range) = current_range.clone() {
                    let (match_range, remaining_range) = rule.split(&range);
                    *current_range = remaining_range;

                    Some(match_range)
                } else {
                    None
                }
            })
            .flatten()
    }
}

#[derive(Debug, Default)]
pub struct Part {
    cool: u64,
    musical: u64,
    aerodynamic: u64,
    shiny: u64,
}

impl Part {
    fn value(&self, category: Category) -> u64 {
        use Category::*;
        match category {
            Cool => self.cool,
            Musical => self.musical,
            Aerodynamic => self.aerodynamic,
            Shiny => self.shiny,
        }
    }

    fn field_mut(&mut self, category: Category) -> &mut u64 {
        use Category::*;
        match category {
            Cool => &mut self.cool,
            Musical => &mut self.musical,
            Aerodynamic => &mut self.aerodynamic,
            Shiny => &mut self.shiny,
        }
    }

    fn update(mut self, category: Category, value: u64) -> Self {
        *self.field_mut(category) = value;
        self
    }

    fn total(&self) -> u64 {
        self.cool + self.musical + self.aerodynamic + self.shiny
    }

    fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut workflow_name = "in".to_string();

        loop {
            let workflow = workflows
                .get(&workflow_name)
                .unwrap_or_else(|| panic!("Failed to find workflow: {}", workflow_name));

            match workflow.outcome(self) {
                Some(Outcome::Accept) => break true,
                Some(Outcome::Reject) => break false,
                Some(Outcome::Jump(name)) => workflow_name = name.clone(),
                None => {
                    panic!(
                        "No rule applied in workflow {:?} for part {:?}",
                        workflow, self
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartRange {
    cool: Range<u64>,
    musical: Range<u64>,
    aerodynamic: Range<u64>,
    shiny: Range<u64>,
}

impl PartRange {
    fn full() -> Self {
        PartRange {
            cool: 1..4001,
            musical: 1..4001,
            aerodynamic: 1..4001,
            shiny: 1..4001,
        }
    }

    fn split(self, workflows: &HashMap<String, Workflow>) -> Vec<(PartRange, bool)> {
        let mut results = vec![];
        let mut to_split = vec![("in".to_string(), self)];

        while let Some((workflow_name, part_range)) = to_split.pop() {
            let workflow = workflows
                .get(&workflow_name)
                .unwrap_or_else(|| panic!("Failed to find workflow: {}", workflow_name));

            for (range, outcome) in workflow.split(part_range) {
                match outcome {
                    Outcome::Accept => results.push((range, true)),
                    Outcome::Reject => results.push((range, false)),
                    Outcome::Jump(name) => to_split.push((name.clone(), range)),
                }
            }
        }

        results
    }

    fn category_range(&self, category: Category) -> &Range<u64> {
        use Category::*;
        match category {
            Cool => &self.cool,
            Musical => &self.musical,
            Aerodynamic => &self.aerodynamic,
            Shiny => &self.shiny,
        }
    }

    fn category_range_mut(&mut self, category: Category) -> &mut Range<u64> {
        use Category::*;
        match category {
            Cool => &mut self.cool,
            Musical => &mut self.musical,
            Aerodynamic => &mut self.aerodynamic,
            Shiny => &mut self.shiny,
        }
    }

    fn update(&self, category: Category, range: Range<u64>) -> Self {
        let mut updated = self.clone();
        *updated.category_range_mut(category) = range;
        updated
    }

    fn size(&self) -> u64 {
        [&self.cool, &self.musical, &self.aerodynamic, &self.shiny]
            .into_iter()
            .map(|range| range.end - range.start)
            .product()
    }
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (HashMap<String, Workflow>, Vec<Part>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve((workflows, parts): Self::Problem) -> (Option<String>, Option<String>) {
        let part1: u64 = parts
            .iter()
            .filter(|part| part.is_accepted(&workflows))
            .map(|part| part.total())
            .sum();

        let part2: u64 = PartRange::full()
            .split(&workflows)
            .into_iter()
            .filter_map(|(range, accepted)| if accepted { Some(range.size()) } else { None })
            .sum();
        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
