use crate::assembunny::instruction::{Instruction, Instruction::*, Operand, Operand::*};
use crate::assembunny::interpreter::ErrType::{Done, ExpectedRegister, NegativePointer};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrType {
    Done(Option<isize>),
    ExpectedRegister(Operand),
    NegativePointer
}

type Res<T> = Result<T, ErrType>;

#[derive(Clone)]
pub struct Machine {
    registers: [isize; 4],
    cur_pointer: usize,
    advance: isize,
    instructions: Vec<Instruction>,
    overrides: Vec<Option<(Instruction, isize)>>
}

impl Machine {
    pub fn new_with_regs(instructions: Vec<Instruction>, registers: [isize; 4]) -> Machine {
        let cur_pointer = 0;
        let advance = 1;
        let overrides = vec![None; instructions.len()];
        Machine { registers, cur_pointer, advance, instructions, overrides }
    }

    pub fn new(instructions: Vec<Instruction>) -> Machine {
        Machine::new_with_regs(instructions, [0; 4])
    }

    fn evaluate_operand(&self, operand: Operand) -> Res<isize> {
        match operand {
            Literal(v) => Ok(v),
            Register(r) => Ok(self.registers[r])
        }
    }

    fn register_id(&self, operand: Operand) -> Res<usize> {
        match operand {
            Register(r) => Ok(r),
            other => Err(ExpectedRegister(other))
        }
    }

    fn eval_cur_instr(&mut self) -> Res<Option<isize>> {
        let (instr, advance) = self.overrides[self.cur_pointer]
            .unwrap_or((self.instructions[self.cur_pointer], 1));

        self.advance = advance;

        match instr {
            Cpy { src, tgt } => {
                let value = self.evaluate_operand(src)?;
                let target = self.register_id(tgt)?;
                self.registers[target] = value;
                Ok(None)
            },
            Inc { tgt } => {
                let target = self.register_id(tgt)?;
                self.registers[target] += 1;
                Ok(None)
            },
            Dec { tgt } => {
                let target = self.register_id(tgt)?;
                self.registers[target] -= 1;
                Ok(None)
            },
            Jnz { test, offset }  => {
                let test = self.evaluate_operand(test)?;
                let offset = self.evaluate_operand(offset)?;

                if test != 0 {
                    self.advance += offset - 1;
                }
                Ok(None)
            },
            Tgl { tgt } => {
                let offset = self.evaluate_operand(tgt)?;

                let target_ptr = self.cur_pointer as isize + offset;

                if target_ptr < 0 {
                    return Err(NegativePointer);
                }
                let target_ptr = target_ptr as usize;
                let num_instrs = self.instructions.len();
                if target_ptr < num_instrs {
                    let cur_instr = self.instructions[target_ptr];
                    self.instructions[target_ptr] = match cur_instr {
                        Cpy { src, tgt }  => Jnz { test: src, offset: tgt },
                        Inc { tgt } => Dec { tgt },
                        Dec { tgt } => Inc { tgt },
                        Jnz { test, offset } => Cpy { src: test, tgt: offset },
                        Tgl { tgt } => Inc { tgt },
                        _ => cur_instr
                    };

                    self.optimize();
                }
                Ok(None)
            },
            Add { src, tgt } => {
                let source = self.evaluate_operand(src)?;
                let target = self.register_id(tgt)?;

                self.registers[target] += source;

                match src {
                    Register(r) => self.registers[r] = 0,
                    _ => ()
                }

                Ok(None)
            },
            AddProd { src, src_clr, tgt, clr } => {
                let source1 = self.register_id(src)?;
                let source2 = self.register_id(src_clr)?;
                let target = self.register_id(tgt)?;
                let to_clear = self.register_id(clr)?;

                self.registers[target] += self.registers[source1] * self.registers[source2];
                self.registers[source2] = 0;
                self.registers[to_clear] = 0;
                Ok(None)

            },
            Out { src } => {
                let value = self.evaluate_operand(src)?;
                Ok(Some(value))
            }
        }
    }

    fn apply_advance(&mut self, step_val: Option<isize>) -> Res<()> {
        let new_pointer = self.cur_pointer as isize + self.advance;
        if new_pointer < 0 {
            return Err(NegativePointer)
        }
        self.cur_pointer = new_pointer as usize;
        self.advance = 1;

        if self.cur_pointer >= self.instructions.len() { Err(Done(step_val)) } else { Ok(()) }
    }

    fn step(&mut self, ignore_errs: bool) -> Res<Option<isize>> {
        match (self.eval_cur_instr(), ignore_errs) {
            (Ok(val), _) => {
                self.apply_advance(val)?;
                Ok(val)
            },
            (Err(Done(val)), _) => Ok(val),
            (Err(NegativePointer), _) => Err(NegativePointer),
            (Err(_), true) => {
                self.apply_advance(None)?;
                Ok(None)
            },
            (Err(other), false) => Err(other)
        }
    }

    fn run_til_output(&mut self, ignore_errs: bool) -> Res<isize> {
        loop {
            match self.step(ignore_errs) {
                Ok(Some(o)) => return Ok(o),
                Ok(None) => continue,
                Err(e) => return Err(e)
            }
        }
    }

    pub fn run(&mut self, ignore_errs: bool) -> Res<Vec<isize>> {
        let mut output = Vec::new();

        loop {
            match self.step(ignore_errs) {
                Err(Done(Some(last_val))) => {
                    output.push(last_val);
                    return Ok(output)
                },
                Err(Done(None)) => {
                    return Ok(output)
                }
                Err(other) => return Err(other),
                Ok(Some(o)) => {
                    output.push(o);
                },
                Ok(None) => continue
            }
        }
    }

    pub fn profile(&mut self, ignore_errs: bool) -> Res<Vec<(usize, usize)>> {
        let mut counts = vec![(0, 0); self.instructions.len()];

        loop {
            if self.overrides[self.cur_pointer].is_some() {
                counts[self.cur_pointer].1 += 1;
            }
            else {
                counts[self.cur_pointer].0 += 1;
            }
            match self.step(ignore_errs) {
                Err(Done(_)) => return Ok(counts),
                Err(other) => return Err(other),
                _ => continue
            }
        }
    }

    fn optimize_at(&mut self, i: usize) -> Option<(Instruction, isize)> {
        // cpy b c
        // inc a
        // dec c
        // jnz c -2
        // dec d
        // jnz d -5
        // Net effect: add b*d to a, and clear c and d.
        if i + 6 <= self.instructions.len() {
            match self.instructions[i..=i+5] {
                [
                Cpy { src: Register(r1), tgt: Register(r2) },
                Inc { tgt: Register(r3) },
                Dec { tgt: Register(r4) },
                Jnz { test: Register(r5), offset: Literal(-2) },
                Dec { tgt: Register(r6) },
                Jnz { test: Register(r7), offset: Literal(-5) },
                ] => {
                    let a = r3;
                    let b = r1;
                    let c = r2;
                    let d = r6;
                    if a != b && a != c && a != d && b != c && b != d && c != d {
                        if r4 == c && r5 == c && r7 == d {
                            let instr = AddProd {
                                src: Register(b),
                                src_clr: Register(d),
                                tgt: Register(a),
                                clr: Register(c)
                            };

                            return Some((instr, 6))
                        }
                    }
                },
                _ => ()
            }
        }

        // Add: Inc { reg1 }; Dec { reg2 }; Jnz { reg2; -2 }
        if i + 3 <= self.instructions.len() {
            match self.instructions[i..i+3] {
                [
                    Inc { tgt: Register(r1) },
                    Dec { tgt: Register(r2) },
                    Jnz { test: Register(r3), offset: Literal(-2) }
                ] => {
                    if r2 != r1 && r3 == r2  {
                        let instr = Add { src: Register(r2), tgt: Register(r1) };
                        return Some((instr, 3))
                    }
                },
                _ => ()
            }
        }

        None
    }

    pub fn optimize(&mut self) {
        let num_instrs = self.instructions.len();
        (0..num_instrs).for_each(|i| self.overrides[i] = self.optimize_at(i));
    }

    pub fn into_iter(self, ignore_errs: bool) -> OutputIterator {
        OutputIterator { machine: self, ignore_errs }
    }
}

pub struct OutputIterator {
    machine: Machine,
    ignore_errs: bool
}

impl Iterator for OutputIterator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.machine.run_til_output(self.ignore_errs);
        result.ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpy() {
        let mut machine = Machine::new(vec![
            Cpy { src: Literal(23), tgt: Register(0) },
            Cpy { src: Register(0), tgt: Register(1) },
            Cpy { src: Register(1), tgt: Literal(2) }
        ]);

        machine.step(false).expect("Should work");
        assert_eq!(machine.registers, [23, 0, 0, 0]);

        machine.step(false).expect("Should work");
        assert_eq!(machine.registers, [23, 23, 0, 0]);

        machine.step(false).expect_err("Should not work");
        assert_eq!(machine.registers, [23, 23, 0, 0]);
    }

    #[test]
    fn test_inc_dec() {
        let mut machine = Machine::new(vec![
            Inc { tgt: Register(0) },
            Dec { tgt: Register(0) },
            Dec { tgt: Register(1) },
            Inc { tgt: Register(1) },
            Inc { tgt: Literal(2) }
        ]);

        machine.step(false).expect("Should work");
        assert_eq!(machine.registers, [1, 0, 0, 0]);

        machine.step(false).expect("Should work");
        assert_eq!(machine.registers, [0, 0, 0, 0]);

        machine.step(false).expect("Should work");
        assert_eq!(machine.registers, [0, -1, 0, 0]);

        machine.step(false).expect("Should work");
        assert_eq!(machine.registers, [0, 0, 0, 0]);

        machine.step(false).expect_err("Should be done");
    }

    #[test]
    fn test_jnz() {
        let mut machine = Machine::new(vec![
            Jnz { test: Register(0), offset: Literal(5) },
            Inc { tgt: Register(0) },
            Jnz { test: Register(1), offset: Literal(10) },
            Jnz { test: Register(0), offset: Literal(3) },
            Cpy { src: Literal(1), tgt: Register(0) },
            Cpy { src: Literal(2), tgt: Register(1) },
            Cpy { src: Literal(3), tgt: Register(2) },
            Cpy { src: Literal(4), tgt: Register(3) }
        ]);

        machine.step(false).expect("Shouldn't jump");
        assert_eq!(machine.registers, [0, 0, 0, 0]);
        assert_eq!(machine.cur_pointer, 1);

        machine.step(false).expect("Increment...");
        assert_eq!(machine.registers, [1, 0, 0, 0]);
        assert_eq!(machine.cur_pointer, 2);

        machine.step(false).expect("Shouldn't jump");
        assert_eq!(machine.registers, [1, 0, 0, 0]);
        assert_eq!(machine.cur_pointer, 3);

        machine.step(false).expect("Should jump");
        assert_eq!(machine.registers, [1, 0, 0, 0]);
        assert_eq!(machine.cur_pointer, 6);

        machine.step(false).expect("Copy...");
        assert_eq!(machine.registers, [1, 0, 3, 0]);
        assert_eq!(machine.cur_pointer, 7);

        assert_eq!(machine.step(false).unwrap_err(), Done(None));
        assert_eq!(machine.registers, [1, 0, 3, 4]);
    }

    #[test]
    fn test_add() {
        let mut machine = Machine::new(vec![
            Cpy { src: Literal(-137), tgt: Register(0) },
            Cpy { src: Literal(574), tgt: Register(1) },
            Cpy { src: Literal(222), tgt: Register(2) },
            Add { src: Register(0), tgt: Register(1) },
            Out { src: Register(0) },
            Out { src: Register(1) },
            Out { src: Register(2) },
            Out { src: Register(3) }
        ]);

        let result = machine.run(false).expect("Should complete");
        assert_eq!(result, vec![0, -137 + 574, 222, 0]);
    }

    #[test]
    fn test_optimize_add() {
        let mut machine_orig = Machine::new(vec![
            Cpy { src: Literal(37), tgt: Register(2) },
            Cpy { src: Literal(525), tgt: Register(3) },
            Inc { tgt: Register(2) },
            Dec { tgt: Register(3) },
            Jnz { test: Register(3), offset: Literal(-2) }
        ]);

        let mut machine_new = machine_orig.clone();

        machine_new.optimize();

        let optimized = machine_new.overrides[2];
        assert_eq!(
            optimized,
            Some((Add { src: Register(3), tgt: Register(2) }, 3))
        );

        assert_eq!(machine_orig.run(false), machine_new.run(false));
    }

    #[test]
    fn test_optimize_mul() {
        // This should do a += b * d;
        let mut machine_orig = Machine::new(vec![
            Cpy { src: Literal(25), tgt: Register(0) },
            Cpy { src: Literal(13), tgt: Register(1) },
            Cpy { src: Literal(2), tgt: Register(3) },
            // cpy b c
            Cpy { src: Register(1), tgt: Register(2) },
            // inc a
            Inc { tgt: Register(0) },
            // dec c
            Dec { tgt: Register(2) },
            // jnz c -2
            Jnz { test: Register(2), offset: Literal(-2) },
            // dec d
            Dec { tgt: Register(3) },
            // jnz d -5
            Jnz { test: Register(3), offset: Literal(-5) }
            // Net effect: add b*d to a, and clear c and d.
        ]);
        machine_orig.registers = [25, 30, 17, 21];

        let mut machine_new = machine_orig.clone();
        machine_new.optimize();

        let result_orig = machine_orig.run(false).expect("Should complete");
        let result_new = machine_new.run(false).expect("Should complete");

        assert_eq!(result_orig, result_new);
        assert_eq!(machine_orig.cur_pointer, machine_new.cur_pointer);
        assert_eq!(machine_new.overrides[3].is_some(), true);
        println!("{:?}", machine_new.overrides[3].unwrap());
        assert_eq!(machine_orig.overrides[3].is_some(), false);
    }
}