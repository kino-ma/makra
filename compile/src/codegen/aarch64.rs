use alloc::prelude::v1::*;

use parity_wasm::elements::{FuncBody, Instruction};

use crate::err::{Error::*, Result};

pub fn generate_func(body: &FuncBody) -> Result<Vec<u8>> {
    //TODO test.wasmのアレを変換できるようにする
    Err(Failure)
}

fn wasm2bin(inst: Instruction) -> [u8; 4] {
    [0; 4]
}
