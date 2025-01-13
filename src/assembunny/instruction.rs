#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operand {
    Literal(isize),
    Register(usize)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Cpy { src: Operand, tgt: Operand },
    Inc { tgt: Operand },
    Dec { tgt: Operand },
    Jnz { test: Operand, offset: Operand },
    Tgl { tgt: Operand },
    Add { src: Operand, tgt: Operand },
    AddProd { src: Operand, src_clr: Operand, tgt: Operand, clr: Operand },
    Out { src: Operand }
}