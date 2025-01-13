use adventofcode2016::build_main;
use std::collections::HashSet;
use std::ops::{AddAssign, Mul};

#[derive(Copy, Clone)]
enum Direction { Left, Right }
use Direction::*;

#[derive(Copy, Clone)]
struct Instruction { direction: Direction, steps: isize }

mod parse {
    use crate::Direction::{Left, Right};
    use crate::Instruction;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1};
    use nom::combinator::{map, map_res, value};
    use nom::multi::separated_list1;
    use nom::sequence::pair;
    use nom::IResult;

    fn number(input: &str) -> IResult<&str, isize> {
        map_res(digit1, str::parse::<isize>)(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        map(
            pair(
                alt((
                    value(Left, char('L')),
                    value(Right, char('R'))
                )),
                number
            ),
            |(direction, steps)| Instruction { direction, steps }
        )(input)
    }

    pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(tag(", "), instruction)(input)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pair(isize, isize);

impl Pair {
    fn l1(&self) -> isize {
        self.0.abs() + self.1.abs()
    }
}

impl AddAssign for Pair {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Mul<isize> for Pair {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output { Pair(self.0 * rhs, self.1 * rhs) }
}

struct State { position: Pair, direction: Pair }

impl State {
    fn turn(&mut self, direction: Direction) {
        self.direction = match direction {
            Left => Pair(-self.direction.1, self.direction.0),
            Right => Pair(self.direction.1, -self.direction.0)
        }
    }

    fn advance(&mut self, steps: isize) {
        self.position += self.direction * steps;
    }

    fn apply(&mut self, instr: Instruction) {
        self.turn(instr.direction);
        self.advance(instr.steps);
    }
}

fn part1(input: &str) -> isize {
    let instructions = parse::instructions(input).unwrap().1;
    let state = State { position: Pair(0, 0), direction: Pair(0, 1) };

    instructions.iter().fold(state, |mut cur, &instr| {
        cur.apply(instr);
        cur
    }).position.l1()
}

fn part2(input: &str) -> isize {
    let instructions = parse::instructions(input).unwrap().1;
    let mut seen = HashSet::new();
    seen.insert(Pair(0, 0));
    let mut state = State { position: Pair(0, 0), direction: Pair(0, 1) };

    for instr in instructions {
        state.turn(instr.direction);

        for _ in 0..instr.steps {
            state.advance(1);
            if !seen.insert(state.position) {
                return state.position.l1()
            }
        }
    }

    panic!()
}

build_main!("day01.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R8, R4, R4, R8";

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4)
    }
}