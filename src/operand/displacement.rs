extern crate ux;
use self::ux::*;

use operand::operand::*;
use operand::register::*;

#[derive(Copy, Clone, Debug)]
pub struct Displacement(Register, u32);

impl Operand for Displacement {
    fn get_type_code_9bits(&self) -> u3 {
        u3::new(if self.fit_in_9bits() { 0x02 } else { 0x03 })
    }

    fn get_type_code_21bits(&self) -> u3 {
        u3::new(if self.fit_in_21bits() { 0x02 } else { 0x03 })
    }

    fn fit_in_9bits(&self) -> bool {
        (0..(1 << 5)).contains(&self.1)
    }

    fn fit_in_21bits(&self) -> bool {
        (0..(1 << 17)).contains(&self.1)
    }

    fn encode_9bits(&self) -> (u9, Option<u32>) {
        let encoded = self.0.encode_9bits();
        (
            (encoded.0 << 5) | u9::new((self.1 & 0x1F) as u16),
            if self.fit_in_9bits() {
                None
            } else {
                Some(self.1)
            },
        )
    }

    fn encode_21bits(&self) -> (u21, Option<u32>) {
        let encoded = self.0.encode_21bits();
        (
            (encoded.0 << 5) | u21::new((self.1 & 0x1FFFF) as u32),
            if self.fit_in_21bits() {
                None
            } else {
                Some(self.1)
            },
        )
    }
}
impl Readable for Displacement {}
impl Writable for Displacement {}
impl Addressable for Displacement {}
