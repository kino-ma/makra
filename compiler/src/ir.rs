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

}

impl Section {
    pub fn new() -> Self {
        Self {

        }
    }
}

/// Intermidate representation
pub struct IR<'a> {
    /// Raw insturction before converted to IR
    raw_instruction: &'a [u8],
    typ: IRType,
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
            raw_instruction,
            typ: IRType::I32,
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