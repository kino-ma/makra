/// Intermidate representation
pub struct IR {
    /// Raw insturction before converted to IR
    raw_instruction: [u8],
    typ: IRType,
}

/// What kind of entry
pub enum IRType {

}

#[cfg(test)]
mod test {
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}