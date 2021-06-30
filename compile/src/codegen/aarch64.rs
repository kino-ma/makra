use alloc::prelude::v1::*;

use parity_wasm::elements::FuncBody;

use crate::err::{Error::*, Result};

pub fn generate_func(body: &FuncBody) -> Result<Vec<u8>> {
    Err(Failure)
}
