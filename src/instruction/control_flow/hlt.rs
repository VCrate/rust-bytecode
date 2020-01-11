use instruction::instruction::*;
use operand::operand::*;

pub struct Hlt {}

impl Instruction for Hlt {
    fn get_type_code(&self) -> u8 {
        0x23u8
    }
    fn encode(&self) -> (u32, Option<u32>, Option<u32>) {
        ((self.get_type_code() as u32) << 24, None, None)
    }
}
