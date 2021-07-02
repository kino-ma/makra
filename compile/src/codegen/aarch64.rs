use alloc::prelude::v1::*;

use parity_wasm::elements::{
    FuncBody,
    Instruction::{self, *},
};

use crate::err::{Error::*, Result};

type Code = [u8; 4];

pub fn generate_func(body: &FuncBody) -> Result<Vec<u8>> {
    //TODO test.wasmのアレを変換できるようにする
    body.code().elements().iter().map(wasm2bin).collect()
}

fn wasm2bin(inst: &Instruction) -> Result<Vec<Code>> {
    // for now, we use r0, r1, r2 to general operations
    match inst {
        I32Const(x) => {
            if x > 1 << 32 {
                Err(TooLargeI32(x))
            } else {
                // 1110_101[S]_0000_[Reg; 4][#imm; 12]
                let mov_r0 = mov(0, x)?;
                let push_r0 = push(0)?;
                Ok(vec![mov_r0, push_r0])
            }
        }

        _ => Err(NotImplemented),
    }
}

fn mov(dist: u8, val: i32) -> Result<Code> {
    if val > 1 << 32 {
        Err(TooLargeI32(val))
    } else {
        let lower8 = (val & 0xff) as u8;
        let upper4 = ((val >> 8) & 0b1111) as u8;
        let be = [0xe3, 0xa0, (dist << 4) & upper4, lower8];
        Ok(to_le(be))
    }
}

fn push(src: u8) -> Result<Code> {
    Ok(to_le([0xe5, 0x2d, src << 4, 0x04]))
}

fn to_le(mut code: Code) -> Code {
    let mut t = code[3];
    code[3] = code[0];
    code[0] = t;

    t = code[2];
    code[2] = code[1];
    code[1] = t;

    code
}
