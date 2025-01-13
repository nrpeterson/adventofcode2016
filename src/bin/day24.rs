use adventofcode2016::build_main;
use itertools::Itertools;
use std::cmp::min;
use std::collections::VecDeque;

#[derive(Debug)]
struct Maze {
    rows: usize,
    cols: usize,
    is_open: Vec<Vec<bool>>,
    number_positions: Vec<(usize, usize)>
}

fn parse_maze(input: &str) -> Maze {
    let (is_open, mut numbers) = input.lines().enumerate()
        .fold((vec![], vec![]), |(mut rows, mut numbers), (i, line)| {
            let (row, line_nums) = line.chars().enumerate()
                .fold((vec![], vec![]), |(mut row, mut nums), (j, c)| {
                    row.push(c != '#');
                    if c != '#' && c != '.' {
                        let n = c as usize - '0' as usize;
                        nums.push((n, (i, j)));
                    }
                    (row, nums)
                });
            rows.push(row);
            numbers.extend_from_slice(&line_nums);
            (rows, numbers)
        });

    let rows = is_open.len();
    let cols = is_open[0].len();

    numbers.sort_by_key(|&(n, _)| n);
    let number_positions = numbers.into_iter().map(|(_, pos)| pos).collect_vec();
    Maze { rows, cols, is_open, number_positions }
}

impl Maze {
    fn num_dists(&self, from: usize) -> Vec<usize> {
        let (i0, j0) = self.number_positions[from];
        let mut dists = vec![vec![usize::MAX; self.cols]; self.rows];
        dists[i0][j0] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(((i0, j0), 0));

        while let Some(((i, j), d)) = queue.pop_front() {
            dists[i][j] = d;

            let mut neighbors = vec![];
            if i > 0 {
                neighbors.push((i - 1, j));
            }
            if i < self.rows - 1 {
                neighbors.push((i + 1, j));
            }
            if j > 0 {
                neighbors.push((i, j - 1));
            }
            if j < self.cols - 1 {
                neighbors.push((i, j + 1));
            }

            neighbors.into_iter().filter(|&(i, j)| self.is_open[i][j])
                .for_each(|(u, v)| {
                    if d + 1 < dists[u][v] {
                        dists[u][v] = d + 1;
                        queue.push_back(((u, v), d + 1));
                    }
                })
        }

        self.number_positions.iter()
            .map(|&(i, j)| dists[i][j])
            .collect_vec()
    }
}

fn part1(input: &str) -> usize {
    let maze = parse_maze(input);
    let dists = (0..maze.number_positions.len())
        .map(|i| maze.num_dists(i))
        .collect_vec();

    let mut stack = Vec::new();
    stack.push((vec![0], 0));

    let mut best = usize::MAX;

    while let Some((path, d)) = stack.pop() {
        if d >= best {
            continue;
        }

        let last = *path.last().unwrap();

        if path.len() == maze.number_positions.len() {
            best = min(best, d);
            continue;
        }

        for i in 1..maze.number_positions.len() {
            if !path.contains(&i) {
                let mut new_path = path.clone();
                new_path.push(i);
                stack.push((new_path, d + dists[last][i]))
            }
        }
    }

    best
}

fn part2(input: &str) -> usize {
    let maze = parse_maze(input);
    let dists = (0..maze.number_positions.len())
        .map(|i| maze.num_dists(i))
        .collect_vec();

    let mut stack = Vec::new();
    stack.push((vec![0], 0));

    let mut best = usize::MAX;

    while let Some((path, d)) = stack.pop() {
        if d >= best {
            continue;
        }

        let last = *path.last().unwrap();

        if path.len() == maze.number_positions.len() {
            best = min(best, d + dists[last][0]);
            continue;
        }

        for i in 1..maze.number_positions.len() {
            if !path.contains(&i) {
                let mut new_path = path.clone();
                new_path.push(i);
                stack.push((new_path, d + dists[last][i]))
            }
        }
    }

    best
}

build_main!("day24.txt", "Part 1" => part1, "Part 2" => part2);