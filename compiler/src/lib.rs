#![no_std]
#![feature(alloc_prelude)]

#[macro_use]
extern crate alloc;

pub mod parse;
pub mod ir;
pub mod codegen;

use alloc::prelude::v1::*;

use crate::ir::Module;

pub struct Compiler {
    module: Module,
}

impl Compiler {
    pub fn new(module: Module) -> Self {
        Self {
            module,
        }
    }

    pub fn parse(binary: &[u8]) -> Result<Self, ()> {
        let module = parse::parse(binary)?;

        Ok(Self::new(module))
    }

    pub fn generate(&self) -> Vec<u8> {
        Vec::new()
    }
}