use adventofcode2016::assembunny::interpreter::Machine;
use adventofcode2016::assembunny::parse;
use adventofcode2016::build_main;

fn part1(input: &str) -> usize {
    let instrs = parse::instructions(input).unwrap().1;

    for a in 0.. {
        let mut machine = Machine::new_with_regs(instrs.clone(), [a as isize, 0, 0, 0]);
        machine.optimize();

        for (i, e) in machine.into_iter(true).enumerate() {
            match (i, e) {
                (1000, _) => return a,
                (i, e) if e != (i % 2) as isize => break,
                _ => continue
            }
        }
    }

    unreachable!()
}

build_main!("day25.txt", "Part 1" => part1);