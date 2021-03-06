use alloc::string::String;
use parity_wasm::SerializationError;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Failure,
    TooLargeI32(i32),
    TooLargeOffset(i32),
    InvalidRegister(u8),
    InvalidOffsetAlignment(u32, u32),
    StackEmpty,
    InvalidStackSubtract(usize, isize),
    NotImplemented(&'static str, Option<String>),
    ParseFailure(SerializationError),
}
