use aoc_runner_derive::aoc_generator;

#[inline]
#[must_use]
#[aoc_generator(dayX)]
pub fn input_generator(input: &str) -> Vec<i32> {
    unimplemented!()
}

#[inline]
#[aoc(dayX, part1)]
pub fn solve_part_1(input: &[i32]) -> i32 {
    unimplemented!()
}

#[inline]
#[aoc(dayX, part2)]
pub fn solve_part_2(input: &[i32]) -> i32 {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use crate::dayX::{input_generator, solve_part_1, solve_part_2};
    const SAMPLEDATA: &str = "";

    #[test]
    fn test_part_1() {
        let generated = input_generator(SAMPLEDATA);
        assert_eq!(solve_part_1(&generated), 0)
    }
    #[test]
    fn test_part_2() {
        let generated = input_generator(SAMPLEDATA);
        assert_eq!(solve_part_2(&generated), 0)
    }
}
