use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline, space1};
use nom::combinator::{map, map_res, opt, recognize, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair};
use crate::assembunny::instruction::{Instruction, Instruction::*, Operand};

fn register(input: &str) -> IResult<&str, usize> {
    alt((
        value(0, char('a')),
        value(1, char('b')),
        value(2, char('c')),
        value(3, char('d'))
    ))(input)
}

fn number(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(pair(opt(char('-')), digit1)),
        |s: &str| s.parse::<isize>()
    )(input)
}
fn operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(register, Operand::Register),
        map(number, Operand::Literal)
    ))(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(
            preceded(tag("cpy "), separated_pair(operand, space1, operand)),
            |(src, tgt)| Cpy { src, tgt }
        ),
        map(
            preceded(tag("inc "), operand),
            |tgt| Inc { tgt }
        ),
        map(
            preceded(tag("dec "), operand),
            |tgt| Dec { tgt }
        ),
        map(
            preceded(tag("jnz "), separated_pair(operand, space1, operand)),
            |(test, offset)| Jnz { test, offset }
        ),
        map(
            preceded(tag("tgl "), operand),
            |tgt| Tgl { tgt }
        ),
        map(
            preceded(tag("out "), operand),
            |src| Out { src }
        )
    ))(input)
}

pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, instruction)(input)
}