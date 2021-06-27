pub(crate) use alloc::vec::Vec;

use num_derive::FromPrimitive;

/// Intermidate representation of a WebAssembly Module
pub struct Module {
    version: u32,
    sections: Vec<Section>,
}

impl Module {
    pub fn new((_magic, version, sections): (&[u8], u32, Vec<Section>)) -> Self {
        Self {
            version,
            sections,
        }
    }
}

/// Intermidate representation of a section
pub struct Section {
    code: u8,
    size: u32,
    content: Vec<u8>,
}

impl Section {
    pub fn new((code, size, content): (u8, u32, Vec<u8>)) -> Self {
        Self {
            code,
            size,
            content
        }
    }
}

#[derive(FromPrimitive)]
pub enum SectionType {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12,
}

/// Intermidate representation
pub struct IR<'a> {
    /// Raw insturction before converted to IR
    _raw_instruction: &'a [u8],
    _typ: IRType,
}

/// What kind of entry
pub enum IRType {
    I32,
    I64,
    F32,
    F64,
}

impl<'a> IR<'a> {
    pub fn new(raw_instruction: &'a [u8]) -> Self {
        Self {
            _raw_instruction: raw_instruction,
            _typ: IRType::I32,
        }
    }
}

pub struct Function {

}

impl Function {
    pub fn new() -> Self {
        Self {

        }
    }
}

#[cfg(test)]
mod test {
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}