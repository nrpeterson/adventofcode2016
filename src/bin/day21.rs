use crate::Instruction::SwapPosition;
use adventofcode2016::build_main;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnLetter(char),
    Reverse(usize, usize),
    Move(usize, usize)
}
use Instruction::*;

mod parse {
    use crate::Instruction;
    use crate::Instruction::*;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{anychar, digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, pair, preceded};
    use nom::IResult;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn swap_position(input: &str) -> IResult<&str, Instruction> {
        map(
            pair(
                delimited(tag("swap position "), number, tag(" with position ")),
                number
            ),
            |(x, y)| SwapPosition(x, y)
        )(input)
    }

    fn swap_letter(input: &str) -> IResult<&str, Instruction> {
        map(
            pair(
                delimited(tag("swap letter "), anychar, tag(" with letter ")),
                anychar
            ),
            |(x, y)| SwapLetter(x, y)
        )(input)
    }


    fn rotate(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(
                delimited(tag("rotate left "), number, alt((tag(" steps"), tag(" step")))),
                RotateLeft
            ),
            map(
                delimited(tag("rotate right "), number, alt((tag(" steps"), tag(" step")))),
                RotateRight
            )
        ))(input)
    }

    fn rotate_based_on_letter(input: &str) -> IResult<&str, Instruction> {
        map(
            preceded(tag("rotate based on position of letter "), anychar),
            RotateBasedOnLetter
        )(input)
    }

    fn reverse(input: &str) -> IResult<&str, Instruction> {
        map(
            pair(
                delimited(tag("reverse positions "), number, tag(" through ")),
                number
            ),
            |(x, y)| Reverse(x, y)
        )(input)
    }

    fn move_positions(input: &str) -> IResult<&str, Instruction> {
        map(
            pair(
                delimited(tag("move position "), number, tag(" to position ")),
                number
            ),
            |(x, y)| Move(x, y)
        )(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            swap_position,
            swap_letter,
            rotate,
            rotate_based_on_letter,
            reverse,
            move_positions
        ))(input)
    }

    pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(newline, instruction)(input)
    }
}

fn apply(instrs: &[Instruction], to: &Vec<char>) -> Vec<char> {
    instrs.into_iter()
        .fold(to.clone(), |mut acc, &instr| {
            match instr {
                SwapPosition(x, y) => {
                    acc.swap(x, y);
                },
                SwapLetter(x, y) => {
                    acc.iter_mut().for_each(|c| {
                        if *c == x {
                            *c = y;
                        }
                        else if *c == y {
                            *c = x;
                        }
                    });
                },
                RotateLeft(n) => {
                    acc.rotate_left(n);
                },
                RotateRight(n) => {
                    acc.rotate_right(n);
                },
                RotateBasedOnLetter(x) => {
                    let i = acc.iter()
                        .position(|c| *c == x)
                        .unwrap();

                    let n = 1 + i + (if i >= 4 { 1 } else { 0 });

                    acc.rotate_right(n % to.len());
                },
                Reverse(x, y) => {
                    acc[x..=y].reverse();
                },
                Move(x, y) => {
                    let c = acc.remove(x);
                    acc.insert(y, c);
                }
            };
            acc
        })
}

fn part1(input: &str) -> String {
    let instrs = parse::instructions(input).unwrap().1;
    let init = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    apply(&instrs, &init).into_iter().collect()
}

fn part2(input: &str) -> String {
    let instrs = parse::instructions(input).unwrap().1;

    let target = vec!['f', 'b', 'g', 'd', 'c', 'e', 'a', 'h'];

    for p in target.iter().cloned().permutations(8) {
        if apply(&instrs, &p) == target {
            return p.into_iter().collect();
        }
    }

    unreachable!()
}

build_main!("day21.txt", "Part 1" => part1, "Part 2" => part2);