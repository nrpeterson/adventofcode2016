use adventofcode2016::build_main;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Df {
    size: usize,
    used: usize,
    avail: usize
}

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    df: Df
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Grid {
    nodes: Vec<Vec<Df>>,
    rows: usize,
    cols: usize,
    goal_data_loc: (usize, usize)
}

impl Index<(usize, usize)> for Grid {
    type Output = Df;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.nodes[i][j]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.nodes[y][x]
    }
}


mod parse {
    use crate::{Df, Grid, Node};
    use itertools::Itertools;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1, newline, space1};
    use nom::combinator::{map, map_res};
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
    use nom::IResult;
    use std::collections::HashMap;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn size(input: &str) -> IResult<&str, usize> {
        terminated(number, char('T'))(input)
    }

    fn node_id(input: &str) -> IResult<&str, (usize, usize)> {
        preceded(
            tag("/dev/grid/node-"),
            separated_pair(
                preceded(char('x'), number),
                char('-'),
                preceded(char('y'), number)
            )
        )(input)
    }

    fn node(input: &str) -> IResult<&str, Node> {
        map(
            tuple((
                node_id,
                preceded(space1, size),
                preceded(space1, size),
                delimited(space1, size, tuple((space1, number, char('%'))))
            )),
            |((x, y), size, used, avail)| {
                Node { x, y, df: Df { size, used, avail } }
            }
        )(input)
    }

    pub fn df_output(input: &str) -> IResult<&str, Grid> {
        map(
            preceded(
                tuple((
                    tag("root@ebhq-gridcenter# df -h"),
                    newline,
                    tag("Filesystem              Size  Used  Avail  Use%"),
                    newline
                )),
                separated_list1(newline, node)
            ),
            |node_list| {
                let rows = node_list.iter().map(|node| node.y).max().unwrap() + 1;
                let cols = node_list.iter().map(|node| node.x).max().unwrap() + 1;

                let node_map: HashMap<(usize, usize), Df> = node_list.into_iter()
                    .map(|Node { x, y, df }| ((x, y), df))
                    .collect();

                let nodes = (0..rows).map(|y| {
                    (0..cols).map(|x| *node_map.get(&(x, y)).unwrap())
                        .collect_vec()
                }).collect();

                Grid { rows, cols, nodes, goal_data_loc: (rows - 1, cols - 1) }
            }
        )(input)
    }
}

fn part1(input: &str) -> usize {
    let grid = parse::df_output(input).unwrap().1;

    (0..grid.rows).cartesian_product(0..grid.cols)
        .map(|pos| grid[pos])
        .permutations(2)
        .map(|v| (v[0], v[1]))
        .filter(|(df_a, df_b)| {
            df_a.used > 0 && df_a.used <= df_b.avail
        }).count()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    goal: (usize, usize),
    empty: (usize, usize)
}

struct Graph {
    grid: Grid,
    walls: HashSet<(usize, usize)>,
    initial_state: State
}

impl Graph {
    fn new(grid: Grid) -> Graph {
        let init_goal = (0, grid.cols - 1);

        let init_empty = (0..grid.rows).cartesian_product(0..grid.cols)
            .find(|&(i, j)| grid[(i, j)].used == 0)
            .unwrap();

        let initial_state = State { goal: init_goal, empty: init_empty };

        let empty_size = grid[init_empty].size;
        let walls: HashSet<(usize, usize)> = (0..grid.rows).cartesian_product(0..grid.cols)
            .filter(|&pos| grid[pos].used > empty_size)
            .collect();

        Graph { grid, walls, initial_state }
    }

    fn edges(&self, state: &State) -> Vec<State> {
        let mut neighbors = vec![];
        let (i, j) = state.empty;
        if i > 0 {
            neighbors.push((i - 1, j));
        }
        if i < self.grid.rows - 1 {
            neighbors.push((i + 1, j));
        }
        if j > 0 {
            neighbors.push((i, j - 1));
        }
        if j < self.grid.cols - 1 {
            neighbors.push((i, j + 1));
        }

        neighbors.into_iter()
            .filter(|pos| !self.walls.contains(pos))
            .map(|pos| {
                if pos == state.goal { State { goal: state.empty, empty: state.goal }}
                else { State { goal: state.goal, empty: pos }}
            })
            .collect()
    }
}

fn part2(input: &str) -> usize {
    let grid = parse::df_output(input).unwrap().1;
    let graph = Graph::new(grid);

    let mut seen = HashSet::new();
    seen.insert(graph.initial_state);

    let mut queue = VecDeque::new();
    queue.push_back((graph.initial_state, 0));

    while let Some((state, steps)) = queue.pop_front() {
        if state.goal == (0, 0) {
            return steps;
        }

        for next_state in graph.edges(&state) {
            if seen.insert(next_state) {
                queue.push_back((next_state, steps + 1));
            }
        }
    }

    unreachable!()
}

build_main!("day22.txt", "Part 1" => part1, "Part 2" => part2);