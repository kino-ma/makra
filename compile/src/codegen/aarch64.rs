use alloc::prelude::v1::*;

use parity_wasm::elements::{
    FuncBody,
    Instruction::{self, *},
};

use crate::err::{Error::*, Result};

type Code = [u8; 4];

pub fn generate_func(body: &FuncBody) -> Result<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    for i in body.code().elements().iter() {
        let code = wasm2bin(i)?;
        v.extend(code.concat());
    }

    Ok(v)
}

fn wasm2bin(inst: &Instruction) -> Result<Vec<Code>> {
    // for now, we use r0, r1, r2 to general operations
    match inst {
        I32Const(x) => {
            let x = *x;
            let mov_r0 = mov(0, x)?;
            let push_r0 = push(0)?;
            Ok(vec![mov_r0, push_r0])
        }

        I32Add => {
            let pop_1 = pop(1)?;
            let pop_2 = pop(2)?;
            let add_ = add(0, 1, 2)?;
            let push_r0 = push(0)?;
            Ok(vec![pop_1, pop_2, add_, push_r0])
        }

        _ => Err(NotImplemented),
    }
}

fn mov(dist: u8, val: i32) -> Result<Code> {
    // 1110_101[S]_0000_[Reg; 4][#imm; 12]
    if val > 1i32.wrapping_shl(11) {
        Err(TooLargeI32(val))
    } else {
        let lower8 = (val & 0xff) as u8;
        let upper4 = ((val >> 8) & 0b1111) as u8;
        let be = [0xe3, 0xa0, (dist << 4) & upper4, lower8];
        Ok(to_le(be))
    }
}

fn add(dist: u8, src_n: u8, src_m: u8) -> Result<Code> {
    // 1110_00_0_0100_0_[src_n; 4]_[dist; 4]_[shift; 5]_00_0_[src_m; 4]
    Ok((0xe080_0000_u32
        | (src_n as u32).wrapping_shl(16)
        | (dist as u32).wrapping_shl(12)
        | src_m as u32)
        .to_le_bytes())
}

fn push(src: u8) -> Result<Code> {
    Ok(to_le([0xe5, 0x2d, src << 4, 0x04]))
}

fn pop(dist: u8) -> Result<Code> {
    Ok(to_le([0xe4, 0x9d, dist << 4, 0x04]))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn i32_const() {
        // i32.const 10
        // push(r0)
        let inst = I32Const(10);
        let expect = {
            let mov10 = 0xe3a0000a_u32.to_le_bytes();
            let push_r0 = 0xe52d0004_u32.to_le_bytes();
            vec![mov10, push_r0]
        };
        let result = wasm2bin(&inst).expect("failed to convert");
        assert_eq!(result, expect);
    }

    #[test]
    fn i32_const_failes_larger_int() {
        // arm can't mov x if x > 2^12
        let num = 123456;
        let inst = I32Const(num);
        let expect = TooLargeI32(num);
        let result = wasm2bin(&inst).expect_err("succeed to parse");
        assert_eq!(result, expect);
    }

    #[test]
    fn i32_add() {
        // i32.add 10 20
        // r0 = r1 + r2

        let inst = I32Add;
        let expect = {
            let pop_n = 0xe49d1004u32.to_le_bytes();
            let pop_m = 0xe49d2004u32.to_le_bytes();
            let add_ = 0xe0810002u32.to_le_bytes();
            let push_res = 0xe52d0004u32.to_le_bytes();

            vec![pop_n, pop_m, add_, push_res]
        };
        let result = wasm2bin(&inst).expect("failed to convert");
        assert_eq!(result, expect);
    }
}
