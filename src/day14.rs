use crate::common::{Direction, Position};
use failure::Error;
use itertools::iproduct;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GridEntry {
    Empty,
    Movable,
    Static,
}

impl Display for GridEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridEntry::Empty => write!(f, "."),
            GridEntry::Movable => write!(f, "O"),
            GridEntry::Static => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    entries: Vec<GridEntry>,
    size: Size,
}

impl Grid {
    fn get_index(&self, position: Position) -> Option<usize> {
        if position.x >= 0
            && position.x < self.size.width as i64
            && position.y >= 0
            && position.y < self.size.height as i64
        {
            Some((position.y * self.size.width as i64 + position.x) as usize)
        } else {
            None
        }
    }

    fn get_entry(&self, position: Position) -> Option<GridEntry> {
        self.get_index(position).map(|index| self.entries[index])
    }

    fn move_rock(&mut self, old_pos: Position, new_pos: Position) {
        let old_index = self.get_index(old_pos).unwrap();
        let new_index = self.get_index(new_pos).unwrap();

        self.entries[old_index] = GridEntry::Empty;
        self.entries[new_index] = GridEntry::Movable;
    }

    fn roll(&mut self, direction: Direction) {
        let positions: Box<dyn Iterator<Item = Position>> = match direction {
            Direction::North => {
                Box::new(iproduct!(0..self.size.width, 0..self.size.height).map(Position::from))
            }
            Direction::East => Box::new(
                iproduct!((0..self.size.width).rev(), 0..self.size.height).map(Position::from),
            ),
            Direction::South => Box::new(
                iproduct!(0..self.size.width, (0..self.size.height).rev()).map(Position::from),
            ),
            Direction::West => {
                Box::new(iproduct!(0..self.size.width, 0..self.size.height).map(Position::from))
            }
        };

        for position in positions {
            if self.get_entry(position) != Some(GridEntry::Movable) {
                continue;
            }

            let mut next_pos = position;

            while self
                .get_entry(next_pos.step(direction))
                .map(|entry| entry == GridEntry::Empty)
                .unwrap_or_default()
            {
                next_pos = next_pos.step(direction);
            }

            if next_pos != position {
                self.move_rock(position, next_pos)
            }
        }
    }

    fn cycle(&mut self) {
        self.roll(Direction::North);
        self.roll(Direction::West);
        self.roll(Direction::South);
        self.roll(Direction::East);
    }

    fn total_load(&self) -> usize {
        self.entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                if *entry == GridEntry::Movable {
                    let row = index / self.size.width;
                    self.size.height - row
                } else {
                    0
                }
            })
            .sum()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let pos = (x, y).into();
                write!(f, "{}", self.get_entry(pos).unwrap())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Grid;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let grid: Vec<Vec<_>> = data
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'O' => GridEntry::Movable,
                        '#' => GridEntry::Static,
                        _ => GridEntry::Empty,
                    })
                    .collect()
            })
            .collect();

        let width = grid[0].len();
        let height = grid.len();

        let entries = grid.into_iter().flatten().collect();

        Ok(Grid {
            entries,
            size: Size { width, height },
        })
    }

    fn solve(grid: Self::Problem) -> (Option<String>, Option<String>) {
        let mut grid1 = grid.clone();
        grid1.roll(Direction::North);
        let part1 = grid1.total_load();

        let mut visited = HashMap::new();
        let mut grid2 = grid.clone();

        let mut rem_spins = 1000000000;
        while rem_spins > 0 {
            if let Some(prev_spins) = visited.get(&grid2) {
                let cycle_len = prev_spins - rem_spins;
                rem_spins %= cycle_len;

                while rem_spins > 0 {
                    grid2.cycle();
                    rem_spins -= 1;
                }

                break;
            } else {
                visited.insert(grid2.clone(), rem_spins);
            }

            grid2.cycle();
            rem_spins -= 1;
        }

        let part2 = grid2.total_load();

        (Some(part1.to_string()), Some(part2.to_string()))
    }
}
