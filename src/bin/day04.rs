use std::cmp::Reverse;
use itertools::Itertools;
use adventofcode2016::build_main;

struct Room<'a> {
    name: Vec<&'a str>,
    sector_id: usize,
    checksum: &'a str
}

impl Room<'_> {
    fn is_valid(&self) -> bool {
        let mut counts = [0; 26];
        self.name.iter()
            .flat_map(|name| name.chars())
            .for_each(|c| counts[c as usize - ('a' as usize)] += 1);

        let mut pairs = counts.into_iter().enumerate()
            .map(|(i, n)| ((i as u8 + ('a' as u8)) as char, n))
            .collect_vec();

        pairs.sort_by_key(|&(i, n)| (Reverse(n), i));

        self.checksum.chars()
            .zip(pairs.iter().map(|&(c, _)| c))
            .all(|(c1, c2)| c1 == c2)
    }

    fn decrypt(&self) -> String {
        self.name.iter()
            .map(|&word| {
                word.chars()
                    .map(|c| {
                        let ord = (c as usize) - ('a' as usize);
                        let new_ord = ((ord + self.sector_id) % 26) as u8 + ('a' as u8);
                        new_ord as char
                    })
                    .collect::<String>()
            })
            .join(" ")
    }
}

mod parse {
    use nom::character::complete::{alpha1, char, digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, preceded, tuple};
    use super::Room;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse::<usize>)(input)
    }

    fn name(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(char('-'), alpha1)(input)
    }

    fn room(input: &str) -> IResult<&str, Room> {
        map(
            tuple((
                name,
                preceded(char('-'), number),
                delimited(char('['), alpha1, char(']'))
            )),
            |(name, sector_id, checksum)| Room { name, sector_id, checksum }
        )(input)
    }

    pub fn rooms(input: &str) -> IResult<&str, Vec<Room>> {
        separated_list1(newline, room)(input)
    }
}

fn part1(input: &str) -> usize {
    parse::rooms(input).unwrap().1.into_iter()
        .filter(|room| room.is_valid())
        .map(|room| room.sector_id)
        .sum()
}

fn part2(input: &str) -> usize {
    parse::rooms(input).unwrap().1.into_iter()
        .filter(|room| room.is_valid())
        .filter(|room| room.decrypt().contains("north"))
        .map(|room| room.sector_id)
        .next()
        .unwrap()
}

build_main!("day04.txt", "Part 1" => part1, "Part 2" => part2);