mod native;
mod reg;

use alloc::prelude::v1::*;

use parity_wasm::elements::{
    FuncBody,
    Instruction::{self, *},
    Local,
};

use crate::err::{Error::*, Result};

pub type Code = [u8; 4];

pub fn generate_func(body: &FuncBody) -> Result<Vec<u8>> {
    let mut v: Vec<u8> = Vec::new();
    // prologue
    // we use r0 to return result
    let registers = [1, 2];
    let locals = body.locals();

    v.extend(create_frame(&registers, locals)?.concat());

    for i in body.code().elements().iter() {
        let code = wasm2bin(i)?;
        v.extend(code.concat());
    }

    // epilogue
    let mut registers = registers.to_vec();
    registers.push(0);
    v.extend(clear_frame(&registers, &locals)?.concat());

    v.extend(native::ret());

    Ok(v)
}

fn wasm2bin(inst: &Instruction) -> Result<Vec<Code>> {
    // for now, we use r0, r1, r2 to general operations
    match inst {
        I32Const(x) => {
            let x = *x;
            let mov_r0 = native::mov_val(0, x)?;
            let push_r0 = native::push(0)?;
            Ok(vec![mov_r0, push_r0])
        }

        I32Add => {
            let pop_1 = native::pop(1)?;
            let pop_2 = native::pop(2)?;
            let add_ = native::add_reg(0, 1, 2)?;
            let push_r0 = native::push(0)?;
            Ok(vec![pop_1, pop_2, add_, push_r0])
        }

        GetLocal(l) => {
            let load_local = native::load(9, reg::FP, native::local_offset(*l))?;
            let push_local = native::push(9)?;
            Ok(vec![load_local, push_local])
        }

        SetLocal(l) => {
            let pop_value = native::pop(9)?;
            let store_value = native::store(9, reg::FP, native::local_offset(*l))?;
            Ok(vec![pop_value, store_value])
        }

        End => Ok(vec![]),

        other => Err(NotImplemented("instruction", Some(format!("{:?}", other)))),
    }
}

fn create_frame(registers: &[u8], locals: &[Local]) -> Result<Vec<Code>> {
    let mut v = Vec::new();
    v.extend(save_registers(registers)?);
    v.extend(setup_locals(locals)?);

    Ok(v)
}

fn clear_frame(registers: &[u8], locals: &[Local]) -> Result<Vec<Code>> {
    let mut v = Vec::new();
    v.extend(load_registers(registers)?);

    let mem_size = native::local_size_aligned(locals.len() as u32);
    let destroy_frame = native::add_imm(reg::SP, reg::SP, mem_size)?;
    v.push(destroy_frame);

    Ok(v)
}

fn save_registers(registers: &[u8]) -> Result<Vec<Code>> {
    let to_push = frame_registers(registers, false);
    to_push.iter().copied().map(native::push).collect()
}

fn load_registers(registers: &[u8]) -> Result<Vec<Code>> {
    let to_pop = frame_registers(registers, true);
    to_pop.iter().copied().map(native::pop).collect()
}

fn frame_registers(registers: &[u8], ascending: bool) -> Vec<u8> {
    let mut v = Vec::new();
    // frame pointer
    v.push(reg::FP);
    // link register; holds return address
    v.push(reg::LR);

    v.dedup();

    // sort by reversed order
    if ascending {
        v.sort();
    } else {
        v.sort_by(|a, b| b.cmp(a));
    }
    v.extend(registers);
    v
}

fn setup_locals(variables: &[Local]) -> Result<Vec<Code>> {
    let aligned_bytes = native::local_size_aligned(variables.len() as u32);
    // reserve 8 bytes for every local variables
    let reserve_memory = native::sub_imm(reg::SP, reg::SP, aligned_bytes as i32)?;
    let init_local: Vec<Code> = variables
        .iter()
        .enumerate()
        .map(|(i, _)| native::store(reg::XZR, reg::FP, native::local_offset(i as u32)))
        .collect::<Result<Vec<Code>>>()?;

    let mut v = vec![reserve_memory];
    v.extend(init_local);
    Ok(v)
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
            let push_lr = 0xf81f8ffdu32.to_le_bytes();
            let push_x10 = 0xf81f8feau32.to_le_bytes();
            let push_x9 = 0xf81f8fe9u32.to_le_bytes();
            let set_frame_base = 0x910003fdu32.to_le_bytes();
            let reserve_local = 0xd10043ffu32.to_le_bytes();
            let mov_10 = 0xd2800149u32.to_le_bytes();
            let push_10 = 0xf81f8fe9u32.to_le_bytes();
            let pop_10 = 0xf84087e9u32.to_le_bytes();
            let set_10 = 0xf90003a9u32.to_le_bytes();
            let mov_20 = 0xd2800289u32.to_le_bytes();
            let push_20 = 0xf81f8fe9u32.to_le_bytes();
            let pop_20 = 0xf84087e9u32.to_le_bytes();
            let set_20 = 0xf90003a9u32.to_le_bytes();
            let get_l0 = 0xf94003au32.to_le_bytes();
            let push_l0 = 0xf81f8feu32.to_le_bytes();
            let get_l1 = 0xf94007au32.to_le_bytes();
            let push_l1 = 0xf81f8feu32.to_le_bytes();

            let pop_l0 = 0xf84087e9u32.to_le_bytes();
            let pop_l1 = 0xf84087eau32.to_le_bytes();
            let add_l0_l1 = 0x8b0a0129u32.to_le_bytes();
            let push_res = 0xf81f8fe0u32.to_le_bytes();
            let clear_local = 0x910043ffu32.to_le_bytes();
            let pop_res = 0xf84087e0u32.to_le_bytes();
            let pop_x9 = 0xf84087e9u32.to_le_bytes();
            let pop_xa = 0xf84087eau32.to_le_bytes();
            let pop_lr = 0xf84087fdu32.to_le_bytes();
            let pop_fp = 0xf84087feu32.to_le_bytes();
            let ret = 0xd65f03c0u32.to_le_bytes();

            vec![
                push_fp,
                push_lr,
                push_x10,
                push_x9,
                set_frame_base,
                reserve_local,
                mov_10,
                push_10,
                pop_10,
                set_10,
                mov_20,
                push_20,
                pop_20,
                set_20,
                get_l0,
                push_l0,
                get_l1,
                push_l1,
                pop_l0,
                pop_l1,
                add_l0_l1,
                push_res,
                clear_local,
                pop_res,
                pop_x9,
                pop_xa,
                pop_lr,
                pop_fp,
                ret,
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
