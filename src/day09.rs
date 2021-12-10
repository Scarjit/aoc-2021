use aoc_runner_derive::aoc_generator;

#[inline]
#[must_use]
#[aoc_generator(day09)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>()
}


#[inline]
#[aoc(day09, part1)]
pub fn solve_part_1(input: &[Vec<u8>]) -> u8 {

    let mut lsum = 0;

    for (x,line) in input.iter().enumerate() {
        for (y, val) in line.iter().enumerate() {
            if val == &9 {
                continue
            }

            let mut top_val = u8::MAX;
            let mut bot_val = u8::MAX;
            let mut right_val = u8::MAX;
            let mut left_val = u8::MAX;
            if x > 0 {
                top_val = input[x - 1][y];
            }
            if x + 1 < input.len() {
                bot_val = input[x + 1][y];
            }
            if y > 0 {
                left_val = input[x][y - 1];
            }
            if y + 1 < line.len() {
                right_val = input[x][y + 1];
            }
            if val < &top_val && val < &bot_val && val < &right_val && val < &left_val {
                lsum += *val+1
            }
        }
    }
    lsum
}

#[inline]
#[aoc(day09, part2)]
pub fn solve_part_2(input: &[Vec<u8>]) -> i32 {
    let lines = input.len();
    let line_len = input[0].len();

    unimplemented!()
}


#[cfg(test)]
mod tests {
    use crate::day09::{input_generator, solve_part_1, solve_part_2};
    const SAMPLEDATA: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part_1() {
        let generated = input_generator(SAMPLEDATA);
        assert_eq!(solve_part_1(&generated), 15)
    }
    #[test]
    fn test_part_2() {
        let generated = input_generator(SAMPLEDATA);
        assert_eq!(solve_part_2(&generated), 1134)
    }
}
