use adventofcode2016::assembunny::instruction::Instruction::Out;
use adventofcode2016::assembunny::instruction::Operand::Register;
use adventofcode2016::build_main;
use adventofcode2016::assembunny::parse;
use adventofcode2016::assembunny::interpreter::Machine;


fn part1(input: &str) -> isize {
    let mut instructions = parse::instructions(input).unwrap().1;
    instructions.push(Out { src: Register(0) });
    let mut machine = Machine::new(instructions);
    machine.optimize();
    machine.run(false).expect("Should work")[0]
}

fn part2(input: &str) -> isize {
    let mut instructions = parse::instructions(input).unwrap().1;
    instructions.push( Out { src: Register(0) });
    let mut machine = Machine::new_with_regs(instructions, [0, 0, 1, 0]);
    machine.optimize();
    machine.run(false).expect("Should work")[0]
}

build_main!("day12.txt", "Part 1" => part1, "Part 2" => part2);