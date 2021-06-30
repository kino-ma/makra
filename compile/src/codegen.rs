#[cfg(aarch64)]
mod aarch64;

#[cfg(aarch64)]
pub use aarch64::*;

use crate::err::{Result, Error};