use instruction::instruction::*;
use operand::operand::*;

struct Ret {}

impl Instruction for Ret {
    fn get_type_code(&self) -> u8 {
        0x20u8
    }
    fn encode(&self) -> (u32, Option<u32>, Option<u32>) {
        ((self.get_type_code() as u32) << 24, None, None)
    }
}
