use aoc_runner_derive::aoc_generator;
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part_1(numbers: &[u32]) -> u32 {
    // Iterate over numbers and numbers where first element is skipped
    // Then compare if the current number from the first iter is smaller then from the 2nd iter
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count() as u32
}

#[aoc(day1, part2)]
pub fn solve_part_2(numbers: &[u32]) -> u32 {
    // Skip 3 here to only check in same sliding window
    numbers
        .iter()
        .zip(numbers.iter().skip(3))
        .filter(|(a, b)| a < b)
        .count() as u32
}
