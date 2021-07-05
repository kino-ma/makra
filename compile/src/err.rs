use parity_wasm::SerializationError;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Failure,
    TooLargeI32(i32),
    InvalidRegister(u8),
    NotImplemented,
    ParseFailure(SerializationError),
}
