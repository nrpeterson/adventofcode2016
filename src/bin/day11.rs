use crate::Item::{Generator, Microchip};
use adventofcode2016::build_main;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, newline, space1};
use nom::combinator::{map, value};
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use nom::IResult;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Hash, Clone)]
enum Item {
    Generator(String),
    Microchip(String)
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct State {
    elevator: usize,
    locations: Vec<(usize, usize)>
}

impl State {
    fn new(items: Vec<Vec<Item>>) -> State {
        let elevator = 0;

        let chip_map: HashMap<String, usize> = items.iter().enumerate()
            .flat_map(|(i, floor)| {
                floor.iter().filter_map(move |item| {
                    match item {
                        Microchip(s) => Some((s.clone(), i)),
                        Generator(_) => None
                    }
                })
            })
            .collect();

        let generator_map: HashMap<String, usize> = items.iter().enumerate()
            .flat_map(|(i, floor)| {
                floor.iter().filter_map(move |item| {
                    match item {
                        Microchip(_) => None,
                        Generator(s) => Some((s.clone(), i))
                    }
                })
            })
            .collect();

        let mut locations = chip_map.keys()
            .map(|key| (chip_map[key].clone(), generator_map[key].clone()))
            .collect_vec();

        locations.sort();

        State { elevator, locations }
    }

    fn apply_move(&self, target: usize, chips: &[usize], gens: &[usize]) -> Option<State> {
        let mut new_locations = self.locations.clone();
        chips.iter().for_each(|&i| new_locations[i].0 = target);
        gens.iter().for_each(|&i| new_locations[i].1 = target);
        new_locations.sort();

        let new_state = State { elevator: target, locations: new_locations };

        Some(new_state).filter(|state| state.is_legal())
    }

    fn is_legal(&self) -> bool {
        let mut contains_generator = [false; 4];
        for &(_, gen_floor) in self.locations.iter() {
            contains_generator[gen_floor] = true;
        }

        for &(chip_floor, gen_floor) in self.locations.iter() {
            if chip_floor != gen_floor && contains_generator[chip_floor] {
                return false;
            }
        }

        true
    }

    fn moves(&self) -> Vec<State> {
        let mut results = Vec::new();

        let mut targets = Vec::new();
        if self.elevator > 0 {
            targets.push(self.elevator - 1);
        }

        if self.elevator < 3 {
            targets.push(self.elevator + 1);
        }

        let chips = self.locations.iter().enumerate()
            .filter(|(_, &(chip_floor, _))| chip_floor == self.elevator)
            .map(|(i, _)| i)
            .collect_vec();

        let gens = self.locations.iter().enumerate()
            .filter(|(_, &(_, gen_floor))| gen_floor == self.elevator)
            .map(|(i, _)| i)
            .collect_vec();

        let pairs = self.locations.iter().enumerate()
            .filter(|(_, &(chip_floor, gen_floor))| {
                chip_floor == gen_floor && chip_floor == self.elevator
            })
            .map(|(i, _)| i)
            .collect_vec();

        for target in targets {
            for &chip in chips.iter() {
                self.apply_move(target, &[chip], &[]).into_iter()
                    .for_each(|s| results.push(s));
            }

            for chip_pair in chips.iter().cloned().combinations(2) {
                self.apply_move(target, &chip_pair, &[]).into_iter()
                    .for_each(|s| results.push(s));
            }

            for &gen in gens.iter() {
                self.apply_move(target, &[], &[gen]).into_iter()
                    .for_each(|s| results.push(s));
            }

            for gen_pair in gens.iter().cloned().combinations(2) {
                self.apply_move(target, &[], &gen_pair).into_iter()
                    .for_each(|s| results.push(s));
            }

            for &pair_i in pairs.iter() {
                self.apply_move(target, &[pair_i], &[pair_i]).into_iter()
                    .for_each(|s| results.push(s));
            }
        }

        results
    }

    fn is_complete(&self) -> bool {
        self.locations.iter().all(|&(i, j)| i == 3 && j == 3)
    }

    fn num_steps(&self) -> usize {
        let mut seen: HashSet<State> = HashSet::new();
        seen.insert(self.clone());

        let mut queue = VecDeque::new();
        queue.push_back((self.clone(), 0));

        while let Some((state, steps)) = queue.pop_front() {
            if state.is_complete() {
                return steps;
            }

            for next_state in state.moves() {
                if seen.insert(next_state.clone()) {
                    queue.push_back((next_state, steps + 1));
                }
            }
        }

        unreachable!()
    }
}

fn parse_input(input: &str) -> IResult<&str, State> {
    map(
        separated_list1(
            newline,
            many1(
                alt((
                    map(
                        terminated(alpha1, tag("-compatible microchip")),
                        |name: &str| Some(Microchip(name.to_owned()))
                    ),
                    map(
                        terminated(alpha1, tag(" generator")),
                        |name: &str| Some(Generator(name.to_owned()))
                    ),
                    alt((
                        value(None, alpha1),
                        value(None, char(',')),
                        value(None, char('.')),
                        value(None, space1)
                    ))
                ))
            )
        ),
        |tokens| {
            let items = tokens.into_iter()
                .map(|line| line.into_iter().filter_map(|x| x).collect_vec())
                .collect_vec();

            State::new(items)
        }
    )(input)
}

fn part1(input: &str) -> usize {
    let initial_state = parse_input(input).unwrap().1;
    initial_state.num_steps()
}

fn part2(input: &str) -> usize {
    let mut state = parse_input(input).unwrap().1;
    state.locations.push((0, 0));
    state.locations.push((0, 0));
    state.locations.sort();

    state.num_steps()
}

build_main!("day11.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_states() {
        let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

        assert_eq!(part1(input), 11);
    }
}