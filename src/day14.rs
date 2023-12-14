use failure::Error;

pub struct Size {
    _width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct Rock {
    offset: usize,
    movable: bool,
}

fn find_total_load(size: &Size, rocks: &[Vec<Rock>]) -> usize {
    let mut load = 0;

    for column in rocks {
        let mut next_available = 0;

        for rock in column {
            if rock.movable {
                assert!(rock.offset >= next_available);
                load += size.height - next_available;
                next_available += 1;
            } else {
                next_available = rock.offset + 1;
            }
        }
    }

    load
}


pub struct Solver {}

impl super::Solver for Solver {
    type Problem = (Size, Vec<Vec<Rock>>);

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        let grid: Vec<Vec<_>> = data
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'O' => Some(true),
                        '#' => Some(false),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        let width = grid[0].len();
        let height = grid.len();

        let rocks = (0..width)
            .map(|x| {
                (0..height)
                    .filter_map(|y| grid[y][x].map(|movable| Rock { offset: y, movable }))
                    .collect()
            })
            .collect();

        Ok((Size { _width: width, height }, rocks))
    }

    fn solve((size, rocks): Self::Problem) -> (Option<String>, Option<String>) {
        let part1 = find_total_load(&size, &rocks);
        (Some(part1.to_string()), None)
    }
}
