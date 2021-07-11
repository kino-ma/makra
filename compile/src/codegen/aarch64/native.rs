use crate::err::{Error::*, Result};
use alloc::fmt::Debug;
use core::convert::TryInto;

use super::reg;
use super::Code;

pub fn mov_val(dist: u8, val: i32) -> Result<Code> {
    validate_register(dist)?;
    // 1101_0010_100_[#imm; 16]_[dist; 5]
    if val >= 1i32 << 15 {
        Err(TooLargeI32(val))
    } else {
        Ok((0xd2800000 | (val as u32) << 5 | dist as u32).into())
    }
}

pub fn mov_reg(dist: u8, src: u8) -> Result<Code> {
    validate_register(dist)?;
    validate_register(src)?;
    // 1001_0001_0000_0000_[src; 5]_0000_00_[XZR; 5 = 11111]_[dist; 5]
    Ok((0xaa0003e0 | shl32(src, 16) | dist as u32).into())
}

pub fn mov_reg_sp(dist: u8, src: u8) -> Result<Code> {
    validate_register(dist)?;
    validate_register(src)?;
    // 1010_1010_0000_0000_0000_00_[src; 5]_[dist; 5]
    Ok((0x91000000 | shl32(src, 5) | dist as u32).into())
}

pub fn add_reg(dist: u8, src_n: u8, src_m: u8) -> Result<Code> {
    validate_register(dist)?;
    validate_register(src_n)?;
    validate_register(src_m)?;
    // 1000_1011_[shift; 2]_0_[src_m; 5]_[imm6]_[src_n; 5]_[dist; 5]
    Ok((0x8b000000u32 | shl32(src_m, 16) | shl32(src_n, 5) | dist as u32).into())
}

pub fn add_imm(dist: u8, src: u8, val: u32) -> Result<Code> {
    validate_register(dist)?;
    validate_register(src)?;
    // 1001_0001_00_[imm12]_[src; 5]_[dist; 5]
    Ok((0x91000000 | shl32(val, 10) | shl32(src, 5) | dist as u32).into())
}

pub fn sub_imm(dist: u8, src: u8, val: i32) -> Result<Code> {
    validate_register(dist)?;
    validate_register(src)?;

    // val can be 2^12 at most
    if val >= 2i32.wrapping_shl(12) {
        return Err(TooLargeI32(val));
    }

    // 1101_0001_00_[imm12]_[src; 5]_[dist; 5]
    Ok((0xd1000000 | shl32(val, 10) | shl32(src, 5) | dist as u32).into())
}

pub fn push(src: u8) -> Result<Code> {
    // 1111_1000_000_[#imm9]_11_[SP; 5]_[src; 5]
    validate_register(src)?;
    Ok((0xf81f8fe0 | shl32(reg::SP, 5) | src as u32).into())
}

pub fn pop(dist: u8) -> Result<Code> {
    validate_register(dist)?;
    Ok((0xf84087e0 | shl32(reg::SP, 5) | dist as u32).into())
    //Ok(to_le([0xe4, 0x9d, dist << 4, 0x04]))
}

pub fn store(src: u8, target: u8, offset: u32) -> Result<Code> {
    validate_register(target)?;
    validate_register(src)?;

    // offset must be a multiple of 8
    if offset % 8 != 0 {
        return Err(InvalidOffsetAlignment(offset, 8));
    }

    // 1111_1001_00_[imm12]_[target; 5]_[src; 5]
    Ok((0xf9000000 | shl32(offset / 8, 10) | shl32(target, 5) | src as u32).into())
}

pub fn load(dist: u8, target: u8, offset: u32) -> Result<Code> {
    validate_register(target)?;
    validate_register(dist)?;

    // offset must be a multiple of 8
    if offset % 8 != 0 {
        return Err(InvalidOffsetAlignment(offset, 8));
    }

    // 1111_1001_01_[imm12]_[target; 5]_[dist; 5]
    Ok((0xf9400000 | shl32(offset / 8, 10) | shl32(target, 5) | dist as u32).into())
}

pub fn ret() -> Code {
    0xd65f03c0u32.into()
}

pub fn local_offset(l: u32) -> u32 {
    l * 8
}

pub fn local_size_aligned(num_local: u32) -> u32 {
    16 * (num_local / 16 + 1)
}

fn validate_register(reg: u8) -> Result<()> {
    if reg > 31 {
        Err(InvalidRegister(reg))
    } else {
        Ok(())
    }
}

fn shl32<T: TryInto<u32> + Debug + Copy>(x: T, rhs: u32) -> u32 {
    match x.try_into() {
        Ok(x) => x.wrapping_shl(rhs),
        _ => panic!("failed to convert {:?} to u32", x),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_reg_correct() {
        // add x0, x1, x2
        let expect = 0x8b020020u32.to_le_bytes();
        let result = add_reg(0, 1, 2).expect("failed to generate");
        assert_eq!(result, expect);
    }

    #[test]
    fn add_imm_correct() {
        // add x0, x1, x2
        let expect = 0x91000c41u32.to_le_bytes();
        let result = add_imm(1, 2, 3).expect("failed to generate");
        assert_eq!(result, expect);
    }

    #[test]
    fn sub_imm_correct() {
        // add x0, x1, x2
        let expect = 0xd1000c41u32.to_le_bytes();
        let result = sub_imm(1, 2, 3).expect("failed to generate");
        assert_eq!(result, expect);
    }

    #[test]
    fn push_correct() {
        // push x0
        let expect = 0xf81f8fe1u32.to_le_bytes();
        let result = push(1).expect("failed to generate");
        assert_eq!(result, expect);
    }

    #[test]
    fn pop_correct() {
        // pop x0
        let expect = 0xf84087e0u32.to_le_bytes();
        let result = pop(0).expect("failed to generate");
        assert_eq!(result, expect);
    }

    #[test]
    fn store_correct() {
        // str x1, [x2, #16]
        let expect = 0xf9000841u32.to_le_bytes();
        let result = store(1, 2, 16).expect("failed to generate");
        assert_eq!(result, expect);
    }

    #[test]
    fn load_correct() {
        // load x1, [x2, #16]
        let expect = 0xf9400841u32.to_le_bytes();
        let result = load(1, 2, 16).expect("failed to generate");
        assert_eq!(result, expect);
    }

    #[test]
    fn mov_val_correct() {
        // mov x0, #10
        let expect = 0xd2800140u32.to_le_bytes();
        let result = mov_val(0, 10).expect("failed to generate");
        assert_eq!(result, expect);
    }

    #[test]
    fn mov_reg_correct() {
        // mov x1, x2
        let expect = 0xaa0203e1u32.to_le_bytes();
        let result = mov_reg(1, 2).expect("failed to generate");
        assert_eq!(result, expect);
    }

    #[test]
    fn mov_reg_sp_correct() {
        // mov x1, x2
        let expect = 0x910003e1u32.to_le_bytes();
        let result = mov_reg(1, reg::SP).expect("failed to generate");
        assert_eq!(result, expect);
    }
}
