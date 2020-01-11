extern crate ux;
use self::ux::*;

pub trait Operand {
    fn get_type_code_9bits(&self) -> u3;
    fn get_type_code_21bits(&self) -> u3;
    fn fit_in_9bits(&self) -> bool {
        self.encode_9bits().1.is_none()
    }
    fn fit_in_21bits(&self) -> bool {
        self.encode_21bits().1.is_none()
    }
    fn encode_9bits(&self) -> (u9, Option<u32>);
    fn encode_21bits(&self) -> (u21, Option<u32>);
}

pub fn encode_12bits(operand: &Operand) -> (u12, Option<u32>) {
    let encoded = operand.encode_9bits();
    (
        u12::new(u16::from(operand.get_type_code_9bits()) << 9) | u12::new(u16::from(encoded.0)),
        encoded.1,
    )
}

pub fn encode_24bits(operand: &Operand) -> (u24, Option<u32>) {
    let encoded = operand.encode_21bits();
    (
        u24::new(u32::from(operand.get_type_code_21bits() << 21)) | u24::new(u32::from(encoded.0)),
        encoded.1,
    )
}

pub trait Writable: Operand {}
pub trait Readable: Operand {}
pub trait Addressable: Operand {}
