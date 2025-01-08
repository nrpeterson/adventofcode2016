use std::collections::HashSet;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{alpha1, char, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use adventofcode2016::build_main;
use crate::Part::{Bracketed, Regular};

#[derive(Debug)]
enum Part {
    Regular(String),
    Bracketed(String)
}

struct IP {
    regular: Vec<String>,
    bracketed: Vec<String>
}

impl IP {
    fn from_parts(parts: Vec<Part>) -> IP {
        parts.into_iter()
            .fold(IP { regular: Vec::new(), bracketed: Vec::new() },|mut acc, part| {
                match part {
                    Regular(s) => acc.regular.push(s),
                    Bracketed(s) => acc.bracketed.push(s)
                };

                acc
            })
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<IP>> {
    map(
        separated_list1(
            newline,
            many1(
                alt((
                    map(delimited(char('['), alpha1, char(']')), |s: &str| Bracketed(s.to_owned())),
                    map(alpha1, |s: &str| Regular(s.to_owned()))
                ))
            )
        ),
        |parts| { parts.into_iter().map(IP::from_parts).collect_vec() }
    )(input)
}

fn contains_abba(s: &str) -> bool {
    s.chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn supports_tls(ip: &IP) -> bool {
    ip.regular.iter().any(|s| contains_abba(s)) &&
        !ip.bracketed.iter().any(|s| contains_abba(s))
}

fn part1(input: &str) -> usize {
    parse_input(input).unwrap().1.into_iter()
        .filter(|v| supports_tls(v))
        .count()
}

fn abas<'a>(s: &'a str) -> impl Iterator<Item=(char, char)> + 'a {
    s.chars()
        .tuple_windows()
        .filter(|&(a, b, c)| a != b &&  a == c)
        .map(|(a, b, _)| (a, b))
}

fn supports_ssl(ip: &IP) -> bool {
    let seen: HashSet<(char, char)> = ip.regular.iter()
        .flat_map(|s| abas(s)).collect();

    ip.bracketed.iter()
        .flat_map(|s| abas(s))
        .any(|(a, b)| seen.contains(&(b, a)))
}

fn part2(input: &str) -> usize {
    parse_input(input).unwrap().1.into_iter()
        .filter(supports_ssl)
        .count()
}

build_main!("day07.txt", "Part 1" => part1, "Part 2" => part2);
