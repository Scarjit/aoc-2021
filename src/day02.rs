use aoc_runner_derive::aoc_generator;

pub struct NavigationCommands {
    pub directions: Direction,
    pub units: i32,
}

pub enum Direction {
    Forward,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            &_ => {
                panic!("Invalid direction {} !", s)
            }
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<NavigationCommands> {
    input
        .lines()
        .map(|line: &str| {
            let splits = line.split_once(' ').expect("No command in line");
            NavigationCommands {
                directions: Direction::from(splits.0),
                units: splits
                    .1
                    .parse::<i32>()
                    .expect("Failed to parse depth to i32"),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part_1(navcommands: &[NavigationCommands]) -> i32 {
    let (forward, depth): (i32, i32) =
        navcommands
            .iter()
            .fold(
                (0, 0),
                |(forward, depth), nc: &NavigationCommands| match nc.directions {
                    Direction::Forward => (forward + nc.units, depth),
                    Direction::Down => (forward, depth + nc.units),
                    Direction::Up => (forward, depth - nc.units),
                },
            );
    forward * depth
}

#[aoc(day2, part2)]
pub fn solve_part_2(navcommands: &[NavigationCommands]) -> i32 {
    let (forward, depth, _aim): (i32, i32, i32) = navcommands.iter().fold(
        (0, 0, 0),
        |(forward, depth, aim), nc: &NavigationCommands| match nc.directions {
            Direction::Forward => (forward + nc.units, depth + (aim * nc.units), aim),
            Direction::Down => (forward, depth, aim + nc.units),
            Direction::Up => (forward, depth, aim - nc.units),
        },
    );
    forward * depth
}
