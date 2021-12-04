use std::fmt::Debug;
use std::str::FromStr;
use aoc_runner_derive::aoc_generator;
use array2d::Array2D;

pub struct Board{
    board: Array2D<(u32, bool)>
}

impl Board {
    pub fn new() -> Self{
        Default::default()
    }
}

fn parse_csv<A: std::str::FromStr>(line: &str) -> Vec<A> where <A as FromStr>::Err: Debug{
    line.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines();

    // Read winning numbers
    let mut number_line = String::new();
    let first_line = parse_csv::<u32>(lines.next().unwrap());

    // Skip the first new line
    lines.next();

    // Read boards
    let  mut boards: Vec<Board> = vec![];
    loop {
        let mut board = Board::new();
        let line = lines.next();
        match line {
            None => {
                println!("Skip");
                // EOF
                break
            }
            Some(v) => {
                if v.trim().len() == 0 {
                    println!("Skip");
                    //Empty line
                    continue
                }

                println!("{}",v);
            }
        }
    }

    (first_line, boards)
}

#[aoc(day4, part1)]
pub fn solve_part_1(input: &(Vec<u32>, Vec<Board>)) -> i32 {
    unimplemented!()
}

#[aoc(day4, part2)]
pub fn solve_part_2(input: &(Vec<u32>, Vec<Board>)) -> i32 {
    unimplemented!()
}
