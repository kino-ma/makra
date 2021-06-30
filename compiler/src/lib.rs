#![no_std]

extern crate alloc;

pub mod parse;
pub mod ir;
pub mod compile;

pub use compile::Compiler;