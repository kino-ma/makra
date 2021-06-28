extern crate parity_wasm;

use crate::ir::Module;

use num_traits::{Unsigned, NumCast};

pub fn parse(buf: &[u8]) -> Result<Module, ()> {
    Module::parse(buf)?;
}
