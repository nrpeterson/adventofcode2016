use std::collections::{HashMap, VecDeque};
use adventofcode2016::build_main;

#[derive(Copy, Clone)]
enum Target {
    Bot(usize),
    Output(usize)
}
use Target::*;

struct StepDescription {
    bot: usize,
    values: (usize, usize)
}

struct Scene {
    bots: HashMap<usize, Vec<usize>>,
    outputs: HashMap<usize, Vec<usize>>,
    rules: HashMap<usize, (Target, Target)>,
    queue: VecDeque<usize>
}

impl Scene {
    fn new(values: Vec<(usize, usize)>, rules: Vec<(usize, (Target, Target))>) -> Scene {
        let mut bots: HashMap<usize, Vec<usize>> = HashMap::new();
        let outputs: HashMap<usize, Vec<usize>> = HashMap::new();
        let rules = HashMap::from_iter(rules.into_iter());
        let mut queue = VecDeque::new();

        values.into_iter()
            .for_each(|(value, to)| {
                bots.entry(to).or_default().push(value);
                queue.push_back(to);
            });

        Scene { bots, outputs, rules, queue }
    }
}

impl Iterator for Scene {
    type Item = StepDescription;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(bot) = self.queue.pop_front() {
            if self.bots[&bot].len() != 2 {
                continue;
            }

            self.bots.get_mut(&bot).unwrap().sort_unstable();

            let a = self.bots[&bot][0];
            let b = self.bots[&bot][1];
            let (a_target, b_target) = self.rules[&bot];
            for (val, target) in [(a, a_target), (b, b_target)] {
                match target {
                    Bot(n) => {
                        self.bots.entry(n).or_default().push(val);
                        self.queue.push_back(n);
                    },
                    Output(n) => self.outputs.entry(n).or_default().push(val)
                };
            }
            self.bots.get_mut(&bot).unwrap().clear();
            return Some(StepDescription { bot, values: (a, b) })
        }

        None
    }
}

mod parse {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, preceded, separated_pair, tuple};
    use crate::{Scene, Target};
    use crate::Target::{Bot, Output};

    enum Instruction {
        Value { value: usize, to: usize },
        Compare { bot: usize, low_to: Target, high_to: Target }
    }
    use Instruction::*;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn target(input: &str) -> IResult<&str, Target> {
        alt((
            map(preceded(tag("bot "), number), Bot),
            map(preceded(tag("output "), number), Output)
        ))(input)
    }

    fn value(input: &str) -> IResult<&str, Instruction> {
        map(
            preceded(
                tag("value "),
                separated_pair(number, tag(" goes to bot "), number)
            ),
            |(value, to)| Value { value, to }
        )(input)
    }

    fn compare(input: &str) -> IResult<&str, Instruction> {
        map(
            tuple((
                delimited(tag("bot "), number, tag(" gives low to ")),
                target,
                preceded(tag(" and high to "), target)
            )),
            |(bot, low_to, high_to)| Compare { bot, low_to, high_to }
        )(input)
    }

    fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(
            newline,
            alt((value, compare))
        )(input)
    }

    pub fn scene(input: &str) -> IResult<&str, Scene> {
        map(
            instructions,
            |instrs| {
                let mut values = Vec::new();
                let mut rules = Vec::new();

                instrs.into_iter()
                    .for_each(|instr| {
                        match instr {
                            Value { value, to } => values.push((value, to)),
                            Compare { bot, low_to, high_to } => {
                                rules.push((bot, (low_to, high_to)));
                            }
                        }
                    });

                Scene::new(values, rules)
            }
        )(input)
    }
}

fn part1(input: &str) -> usize {
    let mut scene = parse::scene(input).unwrap().1;
    scene.find(|desc| desc.values == (17, 61)).unwrap().bot
}

fn part2(input: &str) -> usize {
    let mut scene = parse::scene(input).unwrap().1;

    while (0..3).any(|i| scene.outputs.entry(i).or_default().is_empty()) {
        scene.next();
    }

    (0..3).map(|i| scene.outputs[&i][0]).product()
}

build_main!("day10.txt", "Part 1" => part1, "Part 2" => part2);