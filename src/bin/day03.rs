use itertools::Itertools;
use adventofcode2016::build_main;

mod parse {
    use itertools::Itertools;
    use nom::character::complete::{digit1, multispace1, newline, space1};
    use nom::combinator::{map, map_res};
    use nom::IResult;
    use nom::multi::{count, many1, separated_list1};
    use nom::sequence::preceded;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn triple(input: &str) -> IResult<&str, [usize; 3]> {
        map(
            preceded(space1, separated_list1(space1, number)),
            |v| {
                let mut arr = [v[0], v[1], v[2]];
                arr.sort_unstable();
                arr
            }
        )(input)
    }

    pub fn triples(input: &str) -> IResult<&str, Vec<[usize; 3]>> {
        separated_list1(newline, triple)(input)
    }

    fn triple_columns(input: &str) -> IResult<&str, Vec<[usize; 3]>> {
        map(
            count(preceded(multispace1, number), 9),
            |vs| {
                vec![
                    [vs[0], vs[3], vs[6]],
                    [vs[1], vs[4], vs[7]],
                    [vs[2], vs[5], vs[8]]
                ]
            }
        )(input)
    }

    pub fn all_triple_columns(input: &str) -> IResult<&str, Vec<[usize; 3]>> {
        map(
            many1(triple_columns),
            |vs| vs.into_iter().flatten().collect_vec()
        )(input)
    }
}

fn part1(input: &str) -> usize {
    let triples = parse::triples(input).unwrap().1;

    triples.iter()
        .filter(|&&[a, b, c]| a + b > c)
        .count()
}

fn part2(input: &str) -> usize {
    let triples = parse::all_triple_columns(input).unwrap().1;

    triples.iter()
        .filter(|&t| {
            let mut sorted = t.clone();
            sorted.sort_unstable();
            sorted[0] + sorted[1] > sorted[2]
        })
        .count()
}

build_main!("day03.txt", "Part 1" => part1, "Part 2" => part2);