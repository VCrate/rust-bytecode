use super::operand::*;

#[derive(Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
    Lea(Operand, Operand),
    Swp(Operand, Operand),
    Cmp(Operand, Operand),
    Cmpu(Operand, Operand),

    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mod(Operand, Operand),
    Div(Operand, Operand),
    Divu(Operand, Operand),
    Mul(Operand, Operand),
    Mulu(Operand, Operand),
    Inc(Operand),
    Dec(Operand),

    And(Operand, Operand),
    Or(Operand, Operand),
    Xor(Operand, Operand),
    Not(Operand),
    Shl(Operand, Operand),
    Shr(Operand, Operand),
    Rtl(Operand, Operand),
    Rtr(Operand, Operand),

    Jmp(Operand),
    Jmpe(Operand),
    Jmpne(Operand),
    Jmpg(Operand),
    Jmpge(Operand),
    Call(Operand),
    Ret,
    Hlt,

    Push(Operand),
    Pop(Operand),
    Etr,
    Lve,

    New(Operand, Operand),
    Del(Operand),

    Out(Operand),
    Dbg(Operand),
}
