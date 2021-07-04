use crate::err::{Error::*, Result};

use super::reg;
use super::Code;

pub fn mov(dist: u8, val: i32) -> Result<Code> {
    validate_register(dist)?;
    // 1101_0010_100_[#imm; 16]_[dist; 5]
    if val >= 1i32 << 15 {
        Err(TooLargeI32(val))
    } else {
        Ok((0xd2800000 | (val as u32) << 5 | dist as u32).to_le_bytes())
    }
}

pub fn add(dist: u8, src_n: u8, src_m: u8) -> Result<Code> {
    validate_register(dist)?;
    validate_register(src_n)?;
    validate_register(src_m)?;
    // 1000_1011_[shift; 2]_0_[src_m; 5]_[imm6]_[src_n; 5]_[dist; 5]
    Ok((0x8b000000u32 | shl32(src_m, 16) | shl32(src_n, 5) | dist as u32).to_le_bytes())
}

pub fn push(src: u8) -> Result<Code> {
    validate_register(src)?;
    Ok((0xf8008c00 | shl32(reg::SP, 5) | src as u32).to_le_bytes())
}

pub fn pop(dist: u8) -> Result<Code> {
    validate_register(dist)?;
    Ok((0xf84087e0 | shl32(reg::SP, 5) | dist as u32).to_le_bytes())
    //Ok(to_le([0xe4, 0x9d, dist << 4, 0x04]))
}

fn validate_register(reg: u8) -> Result<()> {
    if reg > 31 {
        Err(InvalidRegister(reg))
    } else {
        Ok(())
    }
}

fn shl32(x: u8, rhs: u32) -> u32 {
    (x as u32).wrapping_shl(rhs)
}
