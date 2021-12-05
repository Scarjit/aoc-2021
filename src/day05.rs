use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;
use std::str::FromStr;

type Point = (i32, i32);

#[derive(Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }
    fn points(&self) -> impl Iterator<Item = Point> {
        let delta = (
            (self.end.0 - self.start.0).signum(),
            (self.end.1 - self.start.1).signum(),
        );
        let (mut point, end) = (self.start, self.end);
        std::iter::repeat_with(move || {
            let r = point;
            point = (point.0 + delta.0, point.1 + delta.1);
            r
        })
        .take_while(move |p| p != &end)
        .chain(std::iter::once(self.end))
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s
            .split(" -> ")
            .map(|s| {
                let splitx = s
                    .split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (splitx[0], splitx[1])
            })
            .collect::<Vec<(i32, i32)>>();
        Ok(Self {
            start: (splits[0].0, splits[0].1),
            end: (splits[1].0, splits[1].1),
        })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part_1(input: &[Line]) -> usize {
    let mut points = HashMap::new();
    input
        .iter()
        .filter(|f| f.is_horizontal_or_vertical())
        .flat_map(|l| l.points())
        .for_each(|p| *points.entry(p).or_insert(0) += 1);
    points.values().filter(|v| v > &&1).count()
}

#[aoc(day5, part2)]
pub fn solve_part_2(input: &[Line]) -> usize {
    let mut points = HashMap::new();
    input
        .iter()
        .flat_map(|l| l.points())
        .for_each(|p| *points.entry(p).or_insert(0) += 1);
    points.values().filter(|v| v > &&1).count()
}
