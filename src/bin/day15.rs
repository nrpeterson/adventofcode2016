use adventofcode2016::build_main;
use itertools::Itertools;
use std::cmp::Reverse;

mod parse {
    use crate::Disc;
    use itertools::Itertools;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, newline, one_of};
    use nom::combinator::{map, map_res, value};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{preceded, terminated};
    use nom::IResult;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn position(input: &str) -> IResult<&str, usize> {
        preceded(tag("position "), number)(input)
    }

    fn num_positions(input: &str) -> IResult<&str, usize> {
        terminated(number, tag(" positions"))(input)
    }

    const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz =,;.0";

    fn token(input: &str) -> IResult<&str, Option<usize>> {
        alt((
            map(position, |n| Some(n)),
            map(num_positions, |n| Some(n)),
            map(preceded(tag("Disc #"), number), |n| Some(n)),
            value(None, one_of(CHARS))
        ))(input)
    }

    fn disc(input: &str) -> IResult<&str, Disc> {
        map(many1(token), |tokens| {
            let good_tokens = tokens.into_iter().filter_map(|x| x).collect_vec();
            Disc { number: good_tokens[0], num_positions: good_tokens[1], initial: good_tokens[2] }
        })(input)
    }

    pub fn discs(input: &str) -> IResult<&str, Vec<Disc>> {
        separated_list1(newline, disc)(input)
    }
}

#[derive(Debug)]
struct Disc {
    number: usize,
    num_positions: usize,
    initial: usize
}

fn crt(mut relations: Vec<(usize, usize)>) -> usize {
    relations.sort_by_key(|&(_, ni)| Reverse(ni));

    let (mut result, mut prod) = relations[0];

    for &(xi, ni) in relations[1..].into_iter() {
        while result % ni != xi {
            result += prod;
        }
        prod *= ni;
    }

    result
}
fn part1(input: &str) -> usize {
    let discs = parse::discs(input).unwrap().1;
    let relations = discs.into_iter().map(|disc| {
        let pos_mod = (disc.initial + disc.number) % disc.num_positions;
        let xi = (disc.num_positions - pos_mod) % disc.num_positions;
        let ni = disc.num_positions;
        (xi, ni)
    }).collect_vec();
    crt(relations)
}

fn part2(input: &str) -> usize {
    let mut discs = parse::discs(input).unwrap().1;
    discs.push(Disc { number: discs.len() + 1, num_positions: 11, initial: 0});
    let relations = discs.into_iter().map(|disc| {
        let pos_mod = (disc.initial + disc.number) % disc.num_positions;
        let xi = (disc.num_positions - pos_mod) % disc.num_positions;
        let ni = disc.num_positions;
        (xi, ni)
    }).collect_vec();
    crt(relations)

}

build_main!("day15.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 5);
    }
}