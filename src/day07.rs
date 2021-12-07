use aoc_runner_derive::aoc_generator;

#[inline]
#[must_use]
#[aoc_generator(day07)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|v|{
        v.parse().unwrap()
    }).collect()
}

#[inline]
#[aoc(day07, part1)]
pub fn solve_part_1(input: &[i32]) -> i32 {
    let mut input = input.to_vec();
    input.sort_unstable();
    let mid = input[input.len()/2];
    input.iter().map(|sub| (sub-mid).abs()).sum()
}

fn calc_total_fuel_needed(input: &[i32], alignment: i32) -> i32{
    input.iter().map(|sub|{
        let steps = (sub - alignment).abs();
        (steps + 1) * steps / 2
    }).sum()
}

#[inline]
#[aoc(day07, part2)]
pub fn solve_part_2(input: &[i32]) -> i32 {
    let avg = input.iter().sum::<i32>() as f32 / input.len() as f32;
    let floor_fuel = calc_total_fuel_needed(&input, avg.floor() as i32);
    let ceil_fuel = calc_total_fuel_needed(&input, avg.ceil() as i32);
    floor_fuel.min(ceil_fuel)
}

#[cfg(test)]
mod tests {
    use crate::day07::{input_generator, solve_part_1, solve_part_2};
    const SAMPLE_DATA: &str = "16,1,2,0,4,2,7,1,2,14";
    #[test]
    fn test_part_1() {
        let generated = input_generator(SAMPLE_DATA);
        assert_eq!(solve_part_1(&generated), 37)
    }
    #[test]
    fn test_part_2() {
        let generated = input_generator(SAMPLE_DATA);
        assert_eq!(solve_part_2(&generated), 168)
    }
}
