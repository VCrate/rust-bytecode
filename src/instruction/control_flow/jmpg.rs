use instruction::instruction::*;
use operand::operand::*;

pub struct Jmpg<A: Readable> {
    operand: A,
}

impl<A: Readable> Instruction for Jmpg<A> {
    fn get_type_code(&self) -> u8 {
        0x0Eu8
    }
    fn encode(&self) -> (u32, Option<u32>, Option<u32>) {
        let operand = encode_24bits(&self.operand);

        (
            (self.get_type_code() as u32) << 24 | u32::from(operand.0),
            operand.1,
            None,
        )
    }
}
