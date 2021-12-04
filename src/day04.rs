use std::fmt::{Debug, Display, format, Formatter};
use std::str::FromStr;
use aoc_runner_derive::aoc_generator;
use array2d::Array2D;

#[derive(Debug, Clone)]
pub struct Board{
    pub lines: Vec<Vec<(u32, bool)>>,
    pub is_winning: bool
}

impl Board {
    pub fn new() -> Self{
        Self{
            lines: vec![],
            is_winning: false
        }
    }

    pub fn add_line(&mut self, line: Vec<u32>){
        let linex = line.iter().map(|n|{
            (*n,false)
        }).collect::<Vec<(u32,bool)>>();
        self.lines.push(linex);
    }

    pub fn set_number(&mut self, number: u32) -> bool {
        for (i_line, line) in self.lines.iter().enumerate() {
            for (i_row, (n, status)) in line.iter().enumerate() {
                if n == &number {
                    self.lines[i_line][i_row] = (*n, true);
                    return self.check_board(i_line, i_row)
                }
            }
        }
        false
    }

    pub fn get_row(&self, rowx: usize) -> Vec<(u32, bool)> {
        let mut row = vec![];
        for line in &self.lines {
            row.push(line[rowx])
        }
        row
    }
    pub fn check_board(&mut self, line: usize, row: usize) -> bool {
        for line in &self.lines {
            let mut x = true;
            for row in line {
                if row.1 == false {
                    x = false;
                }
            }
            if x == true {
                return true
            }
        }

        for r in 0..5 {
            let mut x = true;
            for row in self.get_row(r) {
                if row.1 == false {
                    x = false;
                }
            }
            if x == true {
                return true
            }
        }

        false
    }

    pub fn get_unmarked(&self) -> u32 {
        let flat: u32 = self.lines.iter().flatten().filter(|f| {
            f.1 == false
        }).map(|f|f.0).sum();
        return flat;
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
    let mut board = Board::new();
    loop {
        let line = lines.next();
        match line {
            None => {
                boards.push(board);
                board = Board::new();
                // EOF
                break
            }
            Some(v) => {
                if v.trim().len() == 0 {
                    boards.push(board);
                    board = Board::new();
                    //Empty line
                    continue
                }
                let vx = v.trim_start().replace("  ", " ").split(' ').map(|n| n.parse().unwrap()).collect::<Vec<u32>>();
                board.add_line(vx);
            }
        }
    }

    (first_line, boards)
}

#[aoc(day4, part1)]
pub fn solve_part_1(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let numbers = input.0.clone();
    let mut boards = input.1.clone();

    for number in numbers {
        for board in &mut boards {
            if board.set_number(number){
                let sum = board.get_unmarked();
                return sum*number;
            }
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn solve_part_2(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let numbers = input.0.clone();
    let mut boards = input.1.clone();

    let mut current_winning: Board = Board::new();
    let mut winning_number = 0;
    for number in numbers {
        for board in &mut boards {
            if !board.is_winning && board.set_number(number){
                board.is_winning = true;
                current_winning = board.clone();
                winning_number = number;
            }
        }
    }

    let sum = current_winning.get_unmarked();
    println!("Winning number: {}", winning_number);
    println!("Sum: {}", sum);

    return sum*winning_number;
}
