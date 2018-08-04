#[derive(Debug)]
pub enum RegisterID {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    PC,
    FG,
    BP,
    SP,
}

#[derive(Debug)]
pub enum Operand {
    Register(RegisterID),
    Displacement(RegisterID, u32),
    Address(u32),
    Value(i32),
}

fn is_writable(op: Operand) -> bool {
    match op {
        _ => true,
    }
}

fn is_readable(op: Operand) -> bool {
    match op {
        Operand::Value(_) => false,
        _ => true,
    }
}

fn is_addressable(op: Operand) -> bool {
    match op {
        Operand::Register(_) | Operand::Value(_) => false,
        _ => true,
    }
}
