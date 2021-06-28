extern crate parity_wasm;

use crate::ir::Module;

pub fn parse(buf: &[u8]) -> Result<Module, ()> {
    Module::parse(buf)
}