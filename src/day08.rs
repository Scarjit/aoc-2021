use aoc_runner_derive::aoc_generator;

#[inline]
#[must_use]
#[aoc_generator(day08)]
pub fn input_generator(input: &str) -> Vec<(Vec<String>,Vec<String>)> {
    let lines: Vec<String> = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<String>>();
    lines.iter().map(|line | {
        let s = line.split("|").collect::<Vec<&str>>();
        let encoded = s[0].trim().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        let digits = s[1].trim().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        (encoded,digits)
    }).collect::<Vec<(Vec<String>,Vec<String>)>>()
}

#[inline]
#[aoc(day08, part1)]
pub fn solve_part_1(input: &[(Vec<String>,Vec<String>)]) -> usize {
    let mut count = 0;
    for (_encoded, digits) in input {
        count += digits.iter().filter(|f|{
            f.len() != 5 && f.len() != 6
        }).collect::<Vec<&String>>().len();
    }
    count
}

#[inline]
#[aoc(day08, part2)]
pub fn solve_part_2(input: &[(Vec<String>,Vec<String>)]) -> usize {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use crate::day08::{input_generator, solve_part_1, solve_part_2};
    const SAMPLEDATA: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part_1() {
        let generated = input_generator(SAMPLEDATA);
        assert_eq!(solve_part_1(&generated), 26)
    }
    #[test]
    fn test_part_2() {
        let generated = input_generator(SAMPLEDATA);
        assert_eq!(solve_part_2(&generated), 0)
    }
}
