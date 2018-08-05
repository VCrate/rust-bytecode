extern crate ux;
use self::ux::*;

use operand::operand::*;

#[derive(Copy, Clone, Debug)]
pub struct Value(i32);

impl Operand for Value {
    fn get_type_code_9bits(&self) -> u3 {
        u3::new(if self.fit_in_9bits() { 0x06 } else { 0x07 })
    }

    fn get_type_code_21bits(&self) -> u3 {
        u3::new(if self.fit_in_21bits() { 0x06 } else { 0x07 })
    }

    fn fit_in_9bits(&self) -> bool {
        (0..(1 << 8)).contains(&self.0.abs())
    }

    fn fit_in_21bits(&self) -> bool {
        (0..(1 << 20)).contains(&self.0.abs())
    }

    fn encode_9bits(&self) -> (u9, Option<u32>) {
        let sign = if self.0 < 0 { 0x1 } else { 0x0 };
        (
            u9::new((sign << 8 | self.0 & 0xFF) as u16),
            if self.fit_in_21bits() {
                None
            } else {
                Some(self.0 as u32)
            },
        )
    }

    fn encode_21bits(&self) -> (u21, Option<u32>) {
        let sign = if self.0 < 0 { 0x1 } else { 0x0 };
        (
            u21::new((sign << 20 | self.0 & 0xFFFF) as u32),
            if self.fit_in_21bits() {
                None
            } else {
                Some(self.0 as u32)
            },
        )
    }
}
impl Readable for Value {}
