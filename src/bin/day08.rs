#[derive(Copy, Clone)]
enum Instruction {
    Rect { a: usize, b: usize },
    RotateRow { row: usize, by: usize },
    RotateCol { col: usize, by: usize }
}

use itertools::Itertools;
use adventofcode2016::build_main;
use Instruction::*;

mod parse {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};
    use crate::Instruction;
    use crate::Instruction::*;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn rect(input: &str) -> IResult<&str, Instruction> {
        map(
            preceded(
                tag("rect "),
                separated_pair(number, char('x'), number)
            ),
            |(a, b)| Rect { a, b }
        )(input)
    }

    fn rotate_row(input: &str) -> IResult<&str, Instruction> {
        map(
            preceded(
                tag("rotate row y="),
                separated_pair(number, tag(" by "), number)
            ),
            |(row, by)| RotateRow { row, by }
        )(input)
    }

    fn rotate_col(input: &str) -> IResult<&str, Instruction> {
        map(
            preceded(
                tag("rotate column x="),
                separated_pair(number, tag(" by "), number)
            ),
            |(col, by)| RotateCol { col, by }
        )(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((rect, rotate_row, rotate_col))(input)
    }

    pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(newline, instruction)(input)
    }
}

struct Screen {
    data: [[bool; 50]; 6]
}

impl Screen {
    fn new() -> Screen {
        let data = [[false; 50]; 6];
        Screen { data }
    }

    fn print(&self) {
        self.data.iter().for_each(|row| {
            let s = row.iter()
                .map(|&x| if x { '#' } else { ' ' })
                .collect::<String>();

            println!("{s}");
        });
    }

    fn apply(&mut self, instr: Instruction) {
        match instr {
            Rect { a, b } => {
                (0..b).cartesian_product(0..a)
                    .for_each(|(i, j)| self.data[i][j] = true)
            },
            RotateRow { row, mut by } => {
                by = by % 50;
                self.data[row].rotate_right(by)
            },
            RotateCol { col, mut by } => {
                by = by % 6;
                let mut vals = (0..6).map(|i| self.data[i][col])
                    .collect_vec();

                vals.rotate_right(by);

                self.data.iter_mut().zip(vals.into_iter())
                    .for_each(|(row, val)| row[col] = val);
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let instructions = parse::instructions(input).unwrap().1;

    let screen = instructions.into_iter()
        .fold(Screen::new(), |mut acc, instr| {
            acc.apply(instr);
            acc
        });

    screen.data.iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum()
}

fn part2(input: &str) -> &str {
    let instructions = parse::instructions(input).unwrap().1;
    let screen = instructions.into_iter()
        .fold(Screen::new(), |mut acc, instr| {
            acc.apply(instr);
            acc
        });

    screen.print();

    "See above"
}

build_main!("day08.txt", "Part 1" => part1, "Part 2" => part2);