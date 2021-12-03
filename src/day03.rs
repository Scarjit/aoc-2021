use aoc_runner_derive::aoc_generator;
use array2d::Array2D;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) ->  Array2D<u32>{
    let rows = input.lines().map(|line| {
        line.chars().map(|c| {
            c.to_digit(10).unwrap()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();
    array2d::Array2D::from_rows(&rows)
}

#[aoc(day3, part1)]
pub fn solve_part_1(input: &Array2D<u32>) -> i64 {
    let collen = input.column_len();
    let mut res_gamma: String = String::with_capacity(collen);
    let mut res_epsilon: String = String::with_capacity(collen);
    for col in input.as_columns() {
        let ones = col.iter().filter(|p| p == &&1).count();
        let zeroes = collen - ones;
        if ones > zeroes {
            res_gamma += "0";
            res_epsilon += "1";
        }else{
            res_gamma += "1";
            res_epsilon += "0";
        }
    }
    let gamma = i64::from_str_radix(&res_gamma, 2).unwrap();
    let epsilon = i64::from_str_radix(&res_epsilon, 2).unwrap();
    gamma*epsilon
}

fn get_bitness_of_col(){

}

#[aoc(day3, part2)]
pub fn solve_part_2(input: &Array2D<u32>) -> i32 {
    println!("{:?}", input);
    0
}

