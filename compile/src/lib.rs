#![no_std]
#![feature(alloc_prelude)]

#[macro_use]
extern crate alloc;

pub mod codegen;
pub mod err;
pub mod ir;
pub mod parse;

use alloc::prelude::v1::*;

pub use err::{Error, Result};
use ir::Module;

pub struct Compiler {
    module: Module,
}

impl Compiler {
    pub fn new(module: Module) -> Self {
        Self { module }
    }

    pub fn parse(binary: &[u8]) -> Result<Self> {
        let module = parse::parse(binary)?;

        Ok(Self::new(module))
    }

    pub fn generate(&self) -> Result<Vec<u8>> {
        self.module.generate()
    }
}
