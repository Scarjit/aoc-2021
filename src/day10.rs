use std::collections::HashMap;
use aoc_runner_derive::aoc_generator;

#[inline]
#[must_use]
#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l|l.to_string()).collect()
}

fn remove_valid(s: &str) -> String {
    let mut sx = s.to_string();
    loop {
        let sx_t = sx.replace("()", "").replace("[]","").replace("{}","").replace("<>","");

        if sx_t == sx {
            break
        }
        sx = sx_t;
    }
    sx
}

fn get_illegal_score(sx: String) -> u32 {
    for char in sx.chars() {
        match char {
            ')' => {
                return 3
            }
            ']' => {
                return 57;
            }
            '}' => {
                return 1197
            }
            '>' => {
                return 25137;
            }
            _ => {
            }
        }
    }
    0
}


#[inline]
#[aoc(day10, part1)]
pub fn solve_part_1(input: &[String]) -> u32 {
    input.iter().map(|l| {
        let sx = remove_valid(l);
        get_illegal_score(sx)
    }).sum()
}


fn fill_incomplete(s: &str, scores: &HashMap<char, u64>) -> u64 {
    let mut sx = remove_valid(s);
    if get_illegal_score(sx.clone()) != 0 {
        return 0;
    }
    sx = sx.chars().rev().collect::<String>();
    sx.chars().fold(0u64, |acc, item|{
        acc * 5 + scores.get(&item).unwrap()
    })
}

#[inline]
#[aoc(day10, part2)]
pub fn solve_part_2(input: &[String]) -> u64 {
    let mut scores: HashMap<char, u64> = HashMap::with_capacity(4);
    scores.insert('(',1);
    scores.insert('[',2);
    scores.insert('{',3);
    scores.insert('<',4);
    let mut values = input.iter().map(|l| {
        fill_incomplete(l, &scores)
    }).filter(|s| s != &0).collect::<Vec<u64>>();

    values.sort_unstable();

    *values.get(values.len()/2).unwrap()
}


#[cfg(test)]
mod tests {
    use crate::day10::{input_generator, solve_part_1, solve_part_2};
    const SAMPLEDATA: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1() {
        let generated = input_generator(SAMPLEDATA);
        assert_eq!(solve_part_1(&generated), 26397)
    }
    #[test]
    fn test_part_2() {
        let generated = input_generator(SAMPLEDATA);
        assert_eq!(solve_part_2(&generated), 288957)
    }
}
