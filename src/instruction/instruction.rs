pub trait Instruction {
    fn get_type_code(&self) -> u8 {
        0
    }
    fn encode(&self) -> (u32, Option<u32>, Option<u32>) {
        (0, None, None)
    }
}
