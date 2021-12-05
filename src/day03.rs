use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;

#[inline]
#[must_use]
#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> HashSet<Vec<bool>> {
    let x = input
        .lines()
        .map(|line| line.chars().map(|c| c == '0').collect())
        .collect();
    x
}

#[inline]
#[aoc(day3, part1)]
pub fn solve_part_1(input: &HashSet<Vec<bool>>) -> i64 {
    let rows = input.iter().next().unwrap().len();
    let (gamma, epsilon) = (0..rows).fold((0, 0), |(gamma, epsilon), position| {
        let zeroes = input.iter().filter(|p| p[position]).count();
        let ones = input.len() - zeroes;
        let one_major = ones >= zeroes;
        (
            (gamma << 1) | one_major as i64,
            (epsilon << 1) | !one_major as i64,
        )
    });
    gamma * epsilon
}

fn reduce(mut input: HashSet<Vec<bool>>, retain_one: bool) -> i64 {
    let rows = input.iter().next().unwrap().len();

    for row in 0..rows {
        let zeroes = input.iter().filter(|p| p[row]).count();
        let ones = input.len() - zeroes;
        let one_major = ones >= zeroes;
        let retain = one_major == retain_one;

        input.retain(|r| r[row] == retain);
        if input.len() == 1 {
            let remaining_row = input.iter().next().unwrap();
            return remaining_row
                .iter()
                .fold(0, |x, bit| (x << 1) | !bit as i64);
        }
    }
    unreachable!()
}

#[inline]
#[aoc(day3, part2)]
pub fn solve_part_2(input: &HashSet<Vec<bool>>) -> i64 {
    let o2 = reduce(input.clone(), true);
    let co2 = reduce(input.clone(), false);

    o2 * co2
}
