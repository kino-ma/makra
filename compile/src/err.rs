pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    Failure,
    TooLargeI32(i32),
    NotImplemented,
}
