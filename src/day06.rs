use aoc_runner_derive::aoc_generator;

#[inline]
#[must_use]
#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> [usize; 9] {
    let mut swarm: [usize; 9] = [0usize; 9];
    for fish in input.trim().split(',').map(|l| l.parse::<usize>().unwrap()) {
        swarm[fish] += 1;
    }
    swarm
}

#[inline]
fn r(swarm: &mut [usize], days: usize) -> usize {
    for d in 0..days {
        swarm[((d + 7) % 9)] += swarm[(d % 9)]
    }
    swarm.iter().sum()
}

#[inline]
#[aoc(day6, part1)]
pub fn solve_part_1(input: &[usize; 9]) -> usize {
    let mut input = *input;
    r(&mut input, 80)
}

#[inline]
#[aoc(day6, part2)]
pub fn solve_part_2(input: &[usize; 9]) -> usize {
    let mut input = *input;
    r(&mut input, 256)
}

#[cfg(test)]
mod tests {
    use crate::day06::{input_generator, solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        let input = "3,4,3,1,2";
        let generated = input_generator(input);
        assert_eq!(solve_part_1(&generated), 5934)
    }
    #[test]
    fn test_part_2() {
        let input = "3,4,3,1,2";
        let generated = input_generator(input);
        assert_eq!(solve_part_2(&generated), 26984457539)
    }
}
