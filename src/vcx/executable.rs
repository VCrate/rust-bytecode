extern crate ascii;

use std::collections::HashMap;
use std::fmt;
use std::mem::size_of;

pub struct Executable {
    pub entry_point: u32,
    pub jmp_table: Vec<u32>,
    pub data_section: Vec<u32>,
    pub code_section: Vec<u32>,
    pub symbols: HashMap<ascii::AsciiString, u32>,
}

impl Executable {
    pub fn new() -> Self {
        Executable {
            entry_point: 0,
            jmp_table: Vec::<u32>::new(),
            data_section: Vec::<u32>::new(),
            code_section: Vec::<u32>::new(),
            symbols: HashMap::<ascii::AsciiString, u32>::new(),
        }
    }
}

impl fmt::Debug for Executable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size_header = size_of::<u32>() * 4;
        let size_symbols = self
            .symbols
            .iter()
            .map(|p| {
                size_of::<u32>() + (p.0.len() + 1/* '\0' */) * size_of::<ascii::AsciiChar>()
            }).sum::<usize>();

        let size_jmp_table = self.jmp_table.len() * size_of::<u32>();
        let size_data_section = self.data_section.len() * size_of::<u32>();
        let size_code_section = self.code_section.len() * size_of::<u32>();

        let size =
            size_header + size_symbols + size_jmp_table + size_data_section + size_code_section;

        write!(f, "Excutable {{\n")?;
        write!(f, "  Entry point: {},\n", self.entry_point)?;
        write!(f, "  Size: {} bytes\n", size)?;
        write!(f, "    Header: {} bytes\n", size_header)?;
        write!(f, "    Symbols: {} bytes\n", size_symbols)?;
        write!(f, "    Jump Table: {} bytes\n", size_jmp_table)?;
        write!(f, "    Data: {} bytes\n", size_data_section)?;
        write!(f, "    Code: {} bytes\n", size_code_section)?;
        write!(f, "  Symbols:\n")?;
        for (name, pos) in &self.symbols {
            write!(f, "    {} -> {}\n", name, pos)?;
        }

        write!(f, "}}")
    }
}
