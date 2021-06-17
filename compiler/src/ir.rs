use num_derive::FromPrimitive;
use num::FromPrimitive;

/// Intermidate representation of a WebAssembly Module
pub struct Module {

}

impl Module {
    pub fn new() -> Self {
        Module {

        }
    }
}

/// Intermidate representation of a section
pub struct Section {
    typ: SectionType,
}

impl Section {
    pub fn from_bytes<I: Iterator<Item = u8>>(bytes: &mut I) -> Result<Self, ()> {
        let code = bytes.next().ok_or(())?;
        // TODO decode leb28 (size, u32)
        // For implementingparsers :eyes: https://docs.rs/nom/6.1.2/nom/

        let typ = FromPrimitive::from_u8(code).ok_or(())?;

        Ok(Self {
            typ
        })
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