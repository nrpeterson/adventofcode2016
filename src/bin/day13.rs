use std::collections::{HashSet, VecDeque};
use itertools::Itertools;
use adventofcode2016::build_main;

fn is_open(seed: usize, x: usize, y: usize) -> bool {
    let z = x*x + 3*x + 2*x*y + y + y*y + seed;
    z.count_ones() % 2 == 0
}

fn neighbors(seed: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = vec![(x + 1, y), (x, y + 1)];
    if x > 0 {
        result.push((x - 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }

    result.into_iter().filter(|&(x, y)| is_open(seed, x, y)).collect_vec()
}

fn part1(input: &str) -> usize {
    let seed = input.parse::<usize>().unwrap();
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back(((1, 1), 0));
    seen.insert((1, 1));

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == (31, 39) {
            return steps;
        }

        for neighbor in neighbors(seed, x, y) {
            if seen.insert(neighbor) {
                queue.push_back((neighbor, steps + 1));
            }
        }
    }

    unreachable!()
}

fn part2(input: &str) -> usize {
    let seed = input.parse::<usize>().unwrap();
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back(((1, 1), 0));
    seen.insert((1, 1));

    while let Some(((x, y), steps)) = queue.pop_front() {
        for neighbor in neighbors(seed, x, y) {
            if steps + 1 <= 50 && seen.insert(neighbor) {
                queue.push_back((neighbor, steps + 1));
            }
        }
    }

    seen.len()
}

build_main!("day13.txt", "Part 1" => part1, "Part 2" => part2);