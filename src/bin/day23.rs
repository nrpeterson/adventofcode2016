use adventofcode2016::{assembunny, build_main};
use adventofcode2016::assembunny::instruction::Instruction::Out;
use adventofcode2016::assembunny::instruction::Operand::Register;

fn part1(input: &str) -> isize {
    let mut instructions = assembunny::parse::instructions(input).unwrap().1;
    instructions.push(Out { src: Register(0) });
    let mut machine = assembunny::interpreter::Machine::new_with_regs(instructions, [7, 0, 0, 0]);
    machine.optimize();
    machine.run(true).expect("should run")[0]
}

fn part2(input: &str) -> isize {
    let mut instructions = assembunny::parse::instructions(input).unwrap().1;
    instructions.push(Out { src: Register(0) });
    let mut machine = assembunny::interpreter::Machine::new_with_regs(instructions, [12, 0, 0, 0]);
    machine.optimize();
    machine.run(true).expect("should run")[0]
}

build_main!("day23.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

        assert_eq!(part1(input), 3)
    }
}