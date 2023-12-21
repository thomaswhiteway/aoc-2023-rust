use failure::{err_msg, Error};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    combinator::{all_consuming, map, peek, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::High => write!(f, "1"),
            Pulse::Low => write!(f, "0"),
        }
    }
}

trait PulseHandler {
    fn handle_pulse(&mut self, pulse: Pulse, source: &str) -> Option<Pulse>;
    fn add_source(&mut self, _source: &str) {}
}

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    handler: ModuleHandler,
    output: Vec<String>,
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.handler)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct FlipFlop {
    on: bool,
}

impl PulseHandler for FlipFlop {
    fn handle_pulse(&mut self, pulse: Pulse, _source: &str) -> Option<Pulse> {
        if pulse == Pulse::Low {
            self.on = !self.on;
            Some(if self.on { Pulse::High } else { Pulse::Low })
        } else {
            None
        }
    }
}

impl Display for FlipFlop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.on {
            write!(f, "1")
        } else {
            write!(f, "0")
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Conjunction {
    last_pulse: HashMap<String, Pulse>,
}

impl PulseHandler for Conjunction {
    fn handle_pulse(&mut self, pulse: Pulse, source: &str) -> Option<Pulse> {
        *self
            .last_pulse
            .get_mut(source)
            .unwrap_or_else(|| panic!("Received pulse from unknown source {}", source)) = pulse;

        if self.last_pulse.values().all(|pulse| *pulse == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn add_source(&mut self, source: &str) {
        self.last_pulse.insert(source.to_string(), Pulse::Low);
    }
}

impl Display for Conjunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for pulse in self.last_pulse.values() {
            if !first {
                write!(f, ",")?
            }
            first = false;

            write!(f, "{}", pulse)?
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Broadcast {}

impl PulseHandler for Broadcast {
    fn handle_pulse(&mut self, pulse: Pulse, _source: &str) -> Option<Pulse> {
        Some(pulse)
    }
}

#[derive(Debug, Clone)]
enum ModuleHandler {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}

impl Display for ModuleHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleHandler::FlipFlop(flipflop) => write!(f, "FlipFlop({})", flipflop),
            ModuleHandler::Conjunction(conjunction) => write!(f, "Conjunction({})", conjunction),
            ModuleHandler::Broadcast(_) => write!(f, "Broadcast()"),
        }
    }
}

impl PulseHandler for ModuleHandler {
    fn handle_pulse(&mut self, pulse: Pulse, source: &str) -> Option<Pulse> {
        match self {
            ModuleHandler::FlipFlop(flipflop) => flipflop.handle_pulse(pulse, source),
            ModuleHandler::Conjunction(conjunction) => conjunction.handle_pulse(pulse, source),
            ModuleHandler::Broadcast(broadcast) => broadcast.handle_pulse(pulse, source),
        }
    }

    fn add_source(&mut self, source: &str) {
        match self {
            ModuleHandler::FlipFlop(flipflop) => flipflop.add_source(source),
            ModuleHandler::Conjunction(conjunction) => conjunction.add_source(source),
            ModuleHandler::Broadcast(broadcast) => broadcast.add_source(source),
        }
    }
}

fn module_name(input: &str) -> IResult<&str, String> {
    map(alpha1, str::to_string)(input)
}

fn destinations(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tag(", "), module_name)(input)
}

fn module(input: &str) -> IResult<&str, Module> {
    let handler = alt((
        value(
            ModuleHandler::Broadcast(Broadcast::default()),
            peek(tag("broadcast")),
        ),
        value(
            ModuleHandler::Conjunction(Conjunction::default()),
            char('&'),
        ),
        value(ModuleHandler::FlipFlop(FlipFlop::default()), char('%')),
    ));

    map(
        tuple((
            handler,
            separated_pair(module_name, tag(" -> "), destinations),
        )),
        |(handler, (name, output))| Module {
            name,
            handler,
            output,
        },
    )(input)
}

fn modules(input: &str) -> IResult<&str, Vec<Module>> {
    many1(terminated(module, newline))(input)
}

fn press_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut num_low = 0;
    let mut num_high = 0;
    let mut pulses = VecDeque::new();
    pulses.push_back((Pulse::Low, "broadcaster".to_string(), "button".to_string()));

    while let Some((pulse, destination, source)) = pulses.pop_front() {
        match pulse {
            Pulse::Low => num_low += 1,
            Pulse::High => num_high += 1,
        }

        if let Some(module) = modules.get_mut(&destination) {
            if let Some(new_pulse) = module.handler.handle_pulse(pulse, &source) {
                for new_dest in module.output.iter() {
                    pulses.push_back((new_pulse, new_dest.clone(), destination.clone()));
                }
            }
        }
    }

    (num_low, num_high)
}

fn count_pulses(mut modules: HashMap<String, Module>, num_presses: usize) -> (usize, usize) {
    (0..num_presses)
        .map(|_| press_button(&mut modules))
        .fold((0, 0), |(tot_low, tot_high), (new_low, new_high)| {
            (tot_low + new_low, tot_high + new_high)
        })
}

fn display_header(modules: &HashMap<String, Module>) {
    let names = modules.keys().sorted().collect::<Vec<_>>();
    print!("presses");

    for name in names.iter() {
        let module = modules.get(*name).unwrap();

        if let ModuleHandler::Conjunction(conjunction) = &module.handler {
            for _ in conjunction.last_pulse.keys() {
                print!(",{}", name)
            }
        } else {
            print!(",{}", name);
        }
    }
    println!();

    for name in names {
        let module = modules.get(name).unwrap();

        if let ModuleHandler::Conjunction(conjunction) = &module.handler {
            let sources = conjunction.last_pulse.keys().sorted();
            for source in sources {
                print!(",{}", source)
            }
        } else {
            print!(",");
        }
    }

    println!();
}

fn display_modules(presses: usize, modules: &HashMap<String, Module>) {
    print!("{}", presses);

    let names = modules.keys().sorted();

    for name in names {
        let module = modules.get(name).unwrap();

        match &module.handler {
            ModuleHandler::Conjunction(conjunction) => {
                let sources = conjunction.last_pulse.keys().sorted();

                for source in sources {
                    print!(",{}", conjunction.last_pulse.get(source).unwrap());
                }
            }
            ModuleHandler::FlipFlop(flipflop) => {
                print!(",{}", flipflop);
            }
            ModuleHandler::Broadcast(_) => {
                print!(",1");
            }
        }
    }
    println!();
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = HashMap<String, Module>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let mut modules: HashMap<_, _> = all_consuming(modules)(&data)
            .map(|(_, modules)| modules)
            .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))?
            .into_iter()
            .map(|module| (module.name.clone(), module))
            .collect();

        let links: Vec<_> = modules
            .values()
            .flat_map(|module| {
                module
                    .output
                    .iter()
                    .map(|dest| (module.name.clone(), dest.clone()))
            })
            .collect();

        for (source, dest) in links {
            if let Some(module) = modules.get_mut(&dest) {
                module.handler.add_source(&source);
            }
        }

        Ok(modules)
    }

    fn solve(mut modules: Self::Problem) -> (Option<String>, Option<String>) {
        let (low, high) = count_pulses(modules.clone(), 1000);
        let part1 = low * high;

        display_header(&modules);
        display_modules(0, &modules);
        for index in 1..=64000 {
            press_button(&mut modules);
            display_modules(index, &modules);
        }

        (Some(part1.to_string()), None)
    }
}
