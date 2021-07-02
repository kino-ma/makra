pub type Result<T> = core::result::Result<T, Error>;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Error {
    Failure,
    TooLargeI32(i32),
    InvalidRegister(u8),
    NotImplemented,
}
