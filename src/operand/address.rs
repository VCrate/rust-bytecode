extern crate ux;
use self::ux::*;

use operand::operand::*;

#[derive(Copy, Clone, Debug)]
pub struct Address(u32);

impl Operand for Address {
    fn get_type_code_9bits(&self) -> u3 {
        u3::new(if self.fit_in_9bits() { 0x04 } else { 0x05 })
    }

    fn get_type_code_21bits(&self) -> u3 {
        u3::new(if self.fit_in_21bits() { 0x04 } else { 0x05 })
    }

    fn fit_in_9bits(&self) -> bool {
        (0..(1 << 9)).contains(&self.0)
    }

    fn fit_in_21bits(&self) -> bool {
        (0..(1 << 21)).contains(&self.0)
    }

    fn encode_9bits(&self) -> (u9, Option<u32>) {
        (
            u9::new((self.0 & 0x1FF) as u16),
            if self.fit_in_9bits() {
                None
            } else {
                Some(self.0)
            },
        )
    }

    fn encode_21bits(&self) -> (u21, Option<u32>) {
        (
            u21::new(self.0 & 0x1FFFFF),
            if self.fit_in_21bits() {
                None
            } else {
                Some(self.0)
            },
        )
    }
}
impl Readable for Address {}
impl Writable for Address {}
impl Addressable for Address {}
