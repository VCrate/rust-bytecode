#![feature(range_contains)]
#![feature(crate_visibility_modifier)]

mod instruction;
mod operand;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
