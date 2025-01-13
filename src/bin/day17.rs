use std::collections::VecDeque;
use itertools::Itertools;
use adventofcode2016::build_main;

fn edges((i, j): (usize, usize), seed: &str, path: &str) -> Vec<((usize, usize), String)> {
    let mut options = vec![];

    let udlr = format!("{:x}", md5::compute(format!("{seed}{path}")))
        .chars()
        .take(4)
        .map(|c| c >= 'b')
        .collect_vec();

    if i > 0 && udlr[0] {
        options.push(((i - 1, j), format!("{path}U")));
    }
    if i < 3 && udlr[1]{
        options.push(((i + 1, j), format!("{path}D")));
    }
    if j > 0 && udlr[2] {
        options.push(((i, j - 1), format!("{path}L")));
    }
    if j < 3 && udlr[3] {
        options.push(((i, j + 1), format!("{path}R")));
    }

    options
}

fn part1(input: &str) -> String {
    let seed = input.trim();
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), "".to_string()));

    while let Some((pos, path)) = queue.pop_front() {
        if pos == (3, 3) {
            return path;
        }

        queue.extend(edges(pos, seed, &path));
    }

    unreachable!()
}

fn part2(input: &str) -> usize {
    let seed = input.trim();
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), "".to_string()));

    let mut best = "".to_owned();

    while let Some((pos, path)) = queue.pop_front() {
        if pos == (3, 3) {
            best = path;
        }
        else {
            queue.extend(edges(pos, seed, &path));
        }
    }

    best.len()
}

build_main!("day17.txt", "Part 1" => part1, "Part 2" => part2);