mod native;
mod reg;

use alloc::prelude::v1::*;

use parity_wasm::elements::{
    FuncBody,
    Instruction::{self, *},
    Local,
};

use crate::err::{Error::*, Result};

#[cfg_attr(test, derive(PartialEq))]
pub struct Code {
    raw: [u8; 4],
}

use core::fmt::{Debug, Formatter};
impl Debug for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        let hex: u32 = unsafe { core::mem::transmute(self.raw) };
        f.write_fmt(format_args!("{:x}", hex))
    }
}

use core::borrow::Borrow;
impl Borrow<[u8]> for Code {
    fn borrow(&self) -> &[u8] {
        &self.raw[..]
    }
}

use core::iter::IntoIterator;
impl<'a> IntoIterator for &'a Code {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter()
    }
}

impl<T: Borrow<u32>> From<T> for Code {
    fn from(x: T) -> Self {
        Self {
            raw: x.borrow().to_le_bytes(),
        }
    }
}

impl PartialEq<[u8; 4]> for Code {
    fn eq(&self, other: &[u8; 4]) -> bool {
        &self.raw == other
    }
}

pub struct Generator {
    registers: Vec<u8>,
    locals: Vec<Local>,
    body: FuncBody,
    block_stack: Vec<usize>,
}

impl Generator {
    pub fn new(body: &FuncBody) -> Self {
        let registers = vec![9, 10];
        let locals = body.locals().to_vec();
        let body = body.clone();
        let block_stack = Vec::new();

        Self {
            registers,
            locals,
            body,
            block_stack,
        }
    }

    pub fn generate(&self) -> Result<Vec<u8>> {
        let mut v: Vec<u8> = Vec::new();

        // prologue
        // we use r0 to return result
        v.extend(create_frame(&self.registers, &self.locals)?.concat());

        for i in self.body.code().elements().iter() {
            let code = wasm2bin(i)?;
            debug(&format!("{:?}", code));
            v.extend(code.concat());
        }

        // epilogue
        // pop result
        v.extend(native::pop(0)?.into_iter());
        v.extend(clear_frame(&self.registers, &self.locals)?.concat());

        v.extend(native::ret().into_iter());

        Ok(v)
    }

    fn update_stack(&mut self, inst: &Instruction) -> Result<()> {
        let count = match inst {
            I32Add => -2,
            SetLocal(_) => -1,
            End => 0,
            I32Const(_) | GetLocal(_) => 1,
            other => return Err(NotImplemented("update_stack", None)),
        };
        self.increment_stack(count)
    }

    fn increment_stack(&mut self, count: isize) -> Result<()> {
        match self.block_stack.last_mut() {
            Some(p) => {
                if count < 0 {
                    *p += count as usize;
                } else if (*p as isize) < -count {
                    return Err(TooLittleI32(*p, count as i32));
                } else {
                    *p -= count as usize;
                }
                Ok(())
            }
            None => Err(StackEmpty),
        }
    }
}

pub fn debug(_s: &str) {
    #[cfg(test)]
    {
        extern crate std;
        std::println!("{}", _s);
    }
}

fn wasm2bin(inst: &Instruction) -> Result<Vec<Code>> {
    // for now, we use r0, r1, r2 to general operations
    match inst {
        I32Const(x) => {
            let x = *x;
            let mov_r0 = native::mov_val(9, x)?;
            let push_r0 = native::push(9)?;
            Ok(vec![mov_r0, push_r0])
        }

        I32Add => {
            let pop_1 = native::pop(9)?;
            let pop_2 = native::pop(10)?;
            let add_ = native::add_reg(9, 9, 10)?;
            let push_r0 = native::push(9)?;
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

    let count = locals_count(locals);
    let mem_size = native::local_size_aligned(count);
    let destroy_frame = native::add_imm(reg::SP, reg::SP, mem_size)?;
    v.push(destroy_frame);

    v.extend(load_registers(registers)?);

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

    v.extend(registers);
    v.dedup();

    // sort by reversed order
    if ascending {
        v.sort_by(|a, b| a.cmp(b));
    } else {
        v.sort_by(|a, b| b.cmp(a));
    }
    v
}

fn setup_locals(variables: &[Local]) -> Result<Vec<Code>> {
    debug(&format!("locals: {:?}", variables));

    let count = locals_count(variables);
    let aligned_bytes = native::local_size_aligned(variables.len() as u32);
    // reserve 8 bytes for every local variables
    let reserve_memory = native::sub_imm(reg::SP, reg::SP, aligned_bytes as i32)?;

    let make_frame = native::mov_reg_sp(reg::FP, reg::SP)?;

    let init_local: Vec<Code> = (0..count)
        .map(|i| native::store(reg::XZR, reg::FP, native::local_offset(i as u32)))
        .collect::<Result<Vec<Code>>>()?;

    let mut v = vec![reserve_memory, make_frame];
    v.extend(init_local);
    Ok(v)
}

fn locals_count(l: &[Local]) -> u32 {
    l.iter().fold(0, |a, b| a + b.count())
}

#[cfg(test)]
mod test {
    extern crate std;
    use super::*;

    #[test]
    fn func2code() {
        // wasm function to machine code
        let module = get_wasm_module();
        let bodies = module.code_section().expect("no code section").bodies();
        let body = &bodies[0];

        let expect = {
            use std::fs;
            use std::io::Read;
            let mut f =
                fs::File::open("arm-binaries/test.bin").expect("failed to open test binary file");
            let mut buf = Vec::new();
            f.read_to_end(&mut buf);
            buf
        };

        let generator = Generator::new(body);
        let result = generator.generate().expect("failed to generate");

        assert_eq!(result, expect);
    }

    #[test]
    fn check_create_frame() {
        let expect_bytes = [
            0xf81f8ffeu32,
            0xf81f8ffd,
            0xf81f8fea,
            0xf81f8fe9,
            0xd10043ff,
            0x910003fd,
            0xf90003bf,
            0xf90007bf,
        ];
        let expect = to_le_code(&expect_bytes);
        let registers = &[9, 10];
        let module = get_wasm_module();
        let locals = module.code_section().expect("no code section").bodies()[0].locals();
        let result = create_frame(registers, locals).expect("failed to generate");

        assert_eq!(result, expect);
    }

    #[test]
    fn check_clear_frame() {
        let expect_bytes = [
            0x910043ffu32,
            0xf84087e9,
            0xf84087ea,
            0xf84087fd,
            0xf84087fe,
        ];
        let expect = to_le_code(&expect_bytes);

        let registers = &[9, 10];
        let module = get_wasm_module();
        let locals = module.code_section().expect("no code section").bodies()[0].locals();
        let result = clear_frame(registers, locals).expect("failed to generate");

        assert_eq!(result, expect);
    }

    #[test]
    fn i32_const() {
        // i32.const 10
        // push(r0)
        let inst = I32Const(10);
        let expect = {
            let mov10 = 0xd2800149u32.to_le_bytes();
            let push10 = 0xf81f8fe9u32.to_le_bytes();
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
            let pop_n = 0xf84087e9u32.to_le_bytes();
            let pop_m = 0xf84087eau32.to_le_bytes();
            let add10_20 = 0x8b0a0129u32.to_le_bytes();
            let push_res = 0xf81f8fe9u32.to_le_bytes();

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

    fn get_wasm_module() -> parity_wasm::elements::Module {
        let buf = get_wasm_binary();
        parity_wasm::deserialize_buffer(&buf).expect("failed to parse")
    }

    fn to_le_code(bytes: &[u32]) -> Vec<Code> {
        bytes.iter().map(|x| x.into()).collect()
    }
}
