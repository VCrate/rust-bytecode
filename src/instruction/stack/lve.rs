use instruction::instruction::*;
use operand::operand::*;

pub struct Lve {}

impl Instruction for Lve {
    fn get_type_code(&self) -> u8 {
        0x22u8
    }
    fn encode(&self) -> (u32, Option<u32>, Option<u32>) {
        ((self.get_type_code() as u32) << 24, None, None)
    }
}
