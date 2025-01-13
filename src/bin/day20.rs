use adventofcode2016::build_main;
use std::cmp::max;

mod parse {
    use super::Interval;
    use nom::character::complete::{char, digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;
    use nom::IResult;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn interval(input: &str) -> IResult<&str, Interval> {
        map(
            separated_pair(number, char('-'), number),
            |(a, b)| Interval(a, b)
        )(input)
    }

    pub fn intervals(input: &str) -> IResult<&str, Vec<Interval>> {
        separated_list1(newline, interval)(input)
    }
}

#[derive(Copy, Clone)]
struct Interval(usize, usize);

fn part1(input: &str) -> usize {
    let mut ints = parse::intervals(input).unwrap().1;
    ints.sort_by_key(|interval| (interval.0, interval.1));

    ints.into_iter().fold(0, |cur, Interval(low, high)| {
        if low > cur {
            return cur;
        }

        max(cur, high + 1)
    })
}

fn part2(input: &str) -> usize {
    let mut ints = parse::intervals(input).unwrap().1;
    ints.sort_by_key(|interval| (interval.0, interval.1));

    let (mut result, cur) = ints.into_iter()
        .fold((0, 0), | (mut acc, mut cur), Interval(low, high)| {
            if cur < low {
                acc += low - cur;
            }
            cur = max(high + 1, cur);
            (acc, cur)
        });


    if cur <= (u32::MAX as usize) {
        result += (u32::MAX as usize) - cur;
    }


    result
}

build_main!("day20.txt", "Part 1" => part1, "Part 2" => part2);