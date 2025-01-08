use adventofcode2016::build_main;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Token<'a> {
    Repeat(usize, Vec<Token<'a>>),
    Normal(&'a str),
}
use Token::*;

impl<'a> Token<'a> {
    fn decompressed_length(&self) -> usize {
        match self {
            Repeat(n, tokens) => {
                let children = tokens.iter()
                    .map(|t| t.decompressed_length())
                    .sum::<usize>();

                *n * children
            },
            Normal(s) => s.len()
        }
    }
}

mod parse {
    use super::Token;
    use super::Token::*;
    use nom::branch::alt;
    use nom::bytes::complete::take;
    use nom::character::complete::{alpha1, char, digit1};
    use nom::combinator::{flat_map, map, map_parser, map_res};
    use nom::multi::many1;
    use nom::sequence::{delimited, separated_pair};
    use nom::IResult;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn normal(input: &str) -> IResult<&str, Token> {
        map(alpha1, Normal)(input)
    }
    fn repeat1(input: &str) -> IResult<&str, Token> {
        flat_map(
            delimited(
                char('('),
                separated_pair(number, char('x'), number),
                char(')')
            ),
            |(num_chars, num_repeats)| {
                map(
                    take(num_chars),
                    move |s: &str| { Repeat(num_repeats, vec![Normal(s)]) }
                )
            }
        )(input)
    }

    fn token1(input: &str) -> IResult<&str, Token> {
        alt((repeat1, normal))(input)
    }

    pub fn tokenize1(input: &str) -> IResult<&str, Vec<Token>> {
        many1(token1)(input)
    }

    fn repeat2(input: &str) -> IResult<&str, Token> {
        flat_map(
            delimited(
                char('('),
                separated_pair(number, char('x'), number),
                char(')')
            ),
            |(num_chars, num_repeats)| {
                map(
                    map_parser(take(num_chars), tokenize2),
                    move |ts: Vec<Token>| { Repeat(num_repeats, ts) }
                )
            }
        )(input)
    }

    fn token2(input: &str) -> IResult<&str, Token> {
        alt((repeat2, normal))(input)
    }

    pub fn tokenize2(input: &str) -> IResult<&str, Vec<Token>> {
        many1(token2)(input)
    }
}

fn part1(input: &str) -> usize {
    parse::tokenize1(input).unwrap().1.iter()
        .map(|t| t.decompressed_length())
        .sum()
}

fn part2(input: &str) -> usize {
    parse::tokenize2(input).unwrap().1.iter()
        .map(|t| t.decompressed_length())
        .sum()
}

build_main!("day09.txt", "Part 1" => part1, "Part 2" => part2);