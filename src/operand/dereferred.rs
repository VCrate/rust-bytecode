extern crate ux;
use self::ux::*;

use operand::operand::*;
use operand::register::*;

#[derive(Copy, Clone, Debug)]
pub struct Dereferred(Register);

impl Operand for Dereferred {
    fn get_type_code_9bits(&self) -> u3 {
        u3::new(0x01)
    }

    fn get_type_code_21bits(&self) -> u3 {
        u3::new(0x01)
    }

    fn fit_in_9bits(&self) -> bool {
        self.0.fit_in_9bits()
    }

    fn fit_in_21bits(&self) -> bool {
        self.0.fit_in_21bits()
    }

    fn encode_9bits(&self) -> (u9, Option<u32>) {
        self.0.encode_9bits()
    }

    fn encode_21bits(&self) -> (u21, Option<u32>) {
        self.0.encode_21bits()
    }
}
impl Readable for Dereferred {}
impl Writable for Dereferred {}
impl Addressable for Dereferred {}
