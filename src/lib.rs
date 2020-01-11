#![feature(range_contains)]
#![feature(crate_visibility_modifier)]

extern crate ascii;
extern crate ux;

mod instruction;
mod operand;
mod vcx;

#[cfg(test)]
mod tests {
    use ascii::*;
    use instruction::instruction::*;
    use instruction::*;
    use operand::operand::*;
    use operand::*;
    use ux::*;
    use vcx::executable::*;

    fn test_register_helper(r: register::Register, index: u8) {
        let encoded9 = r.encode_9bits();
        let encoded21 = r.encode_21bits();
        let code9 = r.get_type_code_9bits();
        let code21 = r.get_type_code_21bits();

        assert_eq!(code9, u3::new(0x00), " for {:?}", r);
        assert_eq!(code21, u3::new(0x00), " for {:?}", r);
        assert_eq!(
            encoded9,
            (u9::new((index as u16) << 5), None),
            " for {:?}",
            r
        );
        assert_eq!(
            encoded21,
            (u21::new((index as u32) << 17), None),
            " for {:?}",
            r
        );
    }

    #[test]
    fn test_registers() {
        let registers = [
            register::Register::A,
            register::Register::B,
            register::Register::C,
            register::Register::D,
            register::Register::E,
            register::Register::F,
            register::Register::G,
            register::Register::H,
            register::Register::I,
            register::Register::J,
            register::Register::K,
            register::Register::L,
            register::Register::PC,
            register::Register::FG,
            register::Register::BP,
            register::Register::SP,
        ];
        for (i, r) in registers.iter().enumerate() {
            test_register_helper(*r, i as u8);
        }
    }

    #[test]
    fn test_register() {
        test_register_helper(register::Register::D, 0x03);
    }

    #[test]
    fn test_mov() {
        let mov = load_compare::mov::Mov {
            lhs: register::Register::A,
            rhs: register::Register::B,
        };
        let (is, ex0, ex1) = mov.encode();

        assert_eq!(
            mov.get_type_code(),
            0x07,
            "{:X?} != {:X?}",
            mov.get_type_code(),
            0x07
        );
        assert_eq!(is, 0x07_000_020, "{:b} != {:b}", is, 0x07_000_020);
        assert_eq!(ex0, None);
        assert_eq!(ex1, None);
    }

    #[ignore]
    #[test]
    fn print_executable() {
        let mut exe = Executable::new();

        exe.entry_point = 42;

        exe.code_section.push(0x42);
        exe.code_section.push(0x72);
        exe.code_section.push(0x88);
        exe.code_section.push(0xE5);

        exe.data_section.push(0x79);
        exe.data_section.push(0xFF);
        exe.data_section.push(0x1F);

        exe.jmp_table.push(0x10);
        exe.jmp_table.push(0x21);

        exe.symbols
            .insert(AsciiString::from_ascii("label").unwrap(), 0x02);
        exe.symbols
            .insert(AsciiString::from_ascii("end").unwrap(), 0x08);

        println!("{:?}", exe);
    }
}
