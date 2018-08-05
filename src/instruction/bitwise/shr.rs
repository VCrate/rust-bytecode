use instruction::instruction::*;
use operand::operand::*;

struct Shr<A: Writable, B: Readable> {
    lhs: A,
    rhs: B,
}

impl<A: Writable, B: Readable> Instruction for Shr<A, B> {
    fn get_type_code(&self) -> u8 {
        0x16u8
    }
    fn encode(&self) -> (u32, Option<u32>, Option<u32>) {
        let lhs = encode_12bits(&self.lhs);
        let rhs = encode_12bits(&self.rhs);

        let extra0 = if lhs.1.is_some() {
            lhs.1
        } else if rhs.1.is_some() {
            rhs.1
        } else {
            None
        };

        let extra1 = if lhs.1.is_some() && rhs.1.is_some() {
            rhs.1
        } else {
            None
        };

        (
            (self.get_type_code() as u32) << 24
                | u16::from(lhs.0 << 12) as u32
                | u16::from(rhs.0 << 12) as u32,
            extra0,
            extra1,
        )
    }
}
