mod native;
mod reg;

use alloc::prelude::v1::*;

use parity_wasm::elements::{
    FuncBody,
    Instruction::{self, *},
};

use crate::err::{Error::*, Result};

pub type Code = [u8; 4];

pub fn sample_binary() -> Vec<u8> {
    let mut v = Vec::new();
    v.push(native::mov(0, 57).unwrap());
    v.push(native::ret());
    v.concat()
}

pub fn sample_binary2() -> Vec<u8> {
    vec![
        0xfd, 0x8f, 0x00, 0xf8, 0xe2, 0x8f, 0x00, 0xf8, 0xe1, 0x8f, 0x00, 0xf8, 0x40, 0x01, 0x80,
        0xd2, 0xe0, 0x8f, 0x00, 0xf8, 0x80, 0x02, 0x80, 0xd2, 0xe0, 0x8f, 0x00, 0xf8, 0xe1, 0x87,
        0x40, 0xf8, 0xe2, 0x87, 0x40, 0xf8, 0x20, 0x00, 0x02, 0x8b, 0xe0, 0x8f, 0x00, 0xf8, 0xe0,
        0x87, 0x40, 0xf8, 0xe1, 0x87, 0x40, 0xf8, 0xe2, 0x87, 0x40, 0xf8, 0xfd, 0x87, 0x40, 0xf8,
        0xc0, 0x03, 0x5f, 0xd6,
    ]
}

pub fn generate_func(body: &FuncBody) -> Result<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    //TODO LOCALS
    // prologue
    // we use r0 to return result
    let registers = [1, 2];
    v.extend(prologue(&registers)?.concat());

    for i in body.code().elements().iter() {
        let code = wasm2bin(i)?;
        v.extend(code.concat());
    }

    // epilogue
    let mut registers = registers.to_vec();
    registers.push(0);
    v.extend(epilogue(&registers)?.concat());

    v.extend(native::ret());

    Ok(v)
}

fn wasm2bin(inst: &Instruction) -> Result<Vec<Code>> {
    // for now, we use r0, r1, r2 to general operations
    match inst {
        I32Const(x) => {
            let x = *x;
            let mov_r0 = native::mov(0, x)?;
            let push_r0 = native::push(0)?;
            Ok(vec![mov_r0, push_r0])
        }

        I32Add => {
            let pop_1 = native::pop(1)?;
            let pop_2 = native::pop(2)?;
            let add_ = native::add(0, 1, 2)?;
            let push_r0 = native::push(0)?;
            Ok(vec![pop_1, pop_2, add_, push_r0])
        }

        End => Ok(vec![]),

        _ => Err(NotImplemented),
    }
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

/// Push given registers in reversed order
fn prologue(registers: &[u8]) -> Result<Vec<Code>> {
    let mut registers_owned = registers.to_owned();
    // x29, frame pointer;
    registers_owned.push(29);

    registers_owned.dedup();

    // sort in reversed order
    registers_owned.sort_by(|a, b| b.cmp(a));

    registers_owned.iter().copied().map(native::push).collect()
}

/// Pop given registers
fn epilogue(registers: &[u8]) -> Result<Vec<Code>> {
    let mut registers_owned = registers.to_owned();
    // x29, frame pointer;
    registers_owned.push(29);

    registers_owned.dedup();

    // sort in reversed order
    registers_owned.sort();

    registers_owned.iter().copied().map(native::pop).collect()
}

#[cfg(test)]
mod test {
    extern crate std;
    use super::*;

    #[test]
    fn func2code() {
        // wasm function to machine code
        let bin = get_wasm_binary();
        let module: parity_wasm::elements::Module =
            parity_wasm::deserialize_buffer(&bin).expect("failed to deserialize");
        let bodies = module.code_section().expect("no code section").bodies();
        let body = &bodies[0];

        let expect = {
            let push_fp = 0xf81f8ffdu32.to_le_bytes();
            let push_r2 = 0xf81f8fe2u32.to_le_bytes();
            let push_r1 = 0xf81f8fe1u32.to_le_bytes();
            let mov10 = 0xd2800140u32.to_le_bytes();
            let push10 = 0xf81f8fe0u32.to_le_bytes();
            let mov20 = 0xd2800280u32.to_le_bytes();
            let push20 = 0xf81f8fe0u32.to_le_bytes();
            let pop10 = 0xf84087e1u32.to_le_bytes();
            let pop20 = 0xf84087e2u32.to_le_bytes();
            let add10_20 = 0x8b020020u32.to_le_bytes();
            let push_res = 0xf81f8fe0u32.to_le_bytes();
            let pop_res = 0xf84087e0u32.to_le_bytes();
            let pop_r1 = 0xf84087e1u32.to_le_bytes();
            let pop_r2 = 0xf84087e2u32.to_le_bytes();
            let pop_fp = 0xf84087fdu32.to_le_bytes();
            let ret = 0xd65f03c0u32.to_le_bytes();

            vec![
                push_fp, push_r2, push_r1, mov10, push10, mov20, push20, pop10, pop20, add10_20,
                push_res, pop_res, pop_r1, pop_r2, pop_fp, ret,
            ]
            .concat()
        };

        let result = generate_func(body).expect("failed to generate");

        assert_eq!(result, expect);
    }

    #[test]
    fn i32_const() {
        // i32.const 10
        // push(r0)
        let inst = I32Const(10);
        let expect = {
            let mov10 = 0xd2800140u32.to_le_bytes();
            let push10 = 0xf81f8fe0u32.to_le_bytes();
            vec![mov10, push10]
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
        match result {
            expect => (),
            _ => panic!("invalid error"),
        }
    }

    #[test]
    fn i32_add() {
        // i32.add 10 20
        // r0 = r1 + r2

        let inst = I32Add;
        let expect = {
            let pop_n = 0xf84087e1u32.to_le_bytes();
            let pop_m = 0xf84087e2u32.to_le_bytes();
            let add10_20 = 0x8b020020u32.to_le_bytes();
            let push_res = 0xf81f8fe0u32.to_le_bytes();

            vec![pop_n, pop_m, add10_20, push_res]
        };
        let result = wasm2bin(&inst).expect("failed to convert");
        assert_eq!(result, expect);
    }

    #[test]
    fn te_le_correct() {
        let expect = [4, 3, 2, 1];
        let result = to_le([1, 2, 3, 4]);
        assert_eq!(result, expect);
    }

    fn get_wasm_binary() -> Vec<u8> {
        use std::fs;
        use std::io::Read;

        let mut f = fs::File::open("wasm-binaries/test.wasm").expect("failed to open wasm: ");
        let mut buf = Vec::new();
        if f.read_to_end(&mut buf).expect("fail reading") == 0 {
            panic!("not enaugh content")
        };

        buf
    }
}
