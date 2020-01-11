extern crate ux;
use self::ux::*;

use operand::operand::*;

#[derive(Copy, Clone, Debug)]
pub enum Register {
    A = 0x0,
    B = 0x1,
    C = 0x2,
    D = 0x3,
    E = 0x4,
    F = 0x5,
    G = 0x6,
    H = 0x7,
    I = 0x8,
    J = 0x9,
    K = 0xA,
    L = 0xB,
    PC = 0xC,
    FG = 0xD,
    BP = 0xE,
    SP = 0xF,
}

impl Operand for Register {
    fn get_type_code_9bits(&self) -> u3 {
        u3::new(0x00)
    }

    fn get_type_code_21bits(&self) -> u3 {
        u3::new(0x00)
    }

    fn fit_in_9bits(&self) -> bool {
        true
    }

    fn fit_in_21bits(&self) -> bool {
        true
    }

    fn encode_9bits(&self) -> (u9, Option<u32>) {
        (u9::new(*self as u16) << 5, None)
    }

    fn encode_21bits(&self) -> (u21, Option<u32>) {
        (u21::new(*self as u32) << 17, None)
    }
}
impl Readable for Register {}
impl Writable for Register {}
