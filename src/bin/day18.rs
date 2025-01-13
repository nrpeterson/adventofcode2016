use itertools::{chain, Itertools};
use adventofcode2016::build_main;

fn next_row(prev_row: Vec<bool>) -> (Vec<bool>, usize) {
    let n = prev_row.len();
    chain!([true], prev_row, [true]).tuple_windows()
        .map(|(l, _, r)| l == r)
        .fold((Vec::with_capacity(n), 0), |(mut v, mut c), b| {
            v.push(b);
            if b {
                c += 1;
            }
            (v, c)
        })
}

fn part1(input: &str) -> usize {
    let mut cur = input.chars().map(|c| c == '.').collect_vec();
    let mut result = cur.iter().filter(|&&b| b).count();

    for _ in 1..40 {
        match next_row(cur) {
            (new_row, count) => {
                cur = new_row;
                result += count;
            }
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let mut cur = input.chars().map(|c| c == '.').collect_vec();
    let mut result = cur.iter().filter(|&&b| b).count();

    for _ in 1..400000 {
        match next_row(cur) {
            (new_row, count) => {
                cur = new_row;
                result += count;
            }
        }
    }

    result
}

build_main!("day18.txt", "Part 1" => part1, "Part 2" => part2);