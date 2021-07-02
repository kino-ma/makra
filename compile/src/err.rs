pub type Result<T> = core::result::Result<T, Error>;

#[cfg_attr(test, derive(Debug))]
pub enum Error {
    Failure,
    TooLargeI32(i32),
    NotImplemented,
}
