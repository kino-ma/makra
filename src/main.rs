#![feature(global_asm)]
#![feature(asm)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(alloc_prelude)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;

use core::num::NonZeroUsize;

use alloc::prelude::*;

#[cfg(not(test))]
#[macro_use]
mod print;
#[cfg(not(test))]
mod bsp;
#[cfg(not(test))]
mod console;
#[cfg(not(test))]
mod cpu;
#[cfg(not(test))]
mod memory;
#[cfg(not(test))]
mod panic_wait;
#[cfg(not(test))]
mod runtime_init;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOC: memory::KernelAllocator = memory::KernelAllocator;

extern crate compile;

extern "C" {
    static _binary_compile_wasm_binaries_test_wasm_start: [u8; 32];
}

unsafe fn kernel_init() -> ! {
    unsafe {
        memory::init();
    }
    println!("Hello QEMU!");
    alloc::vec::Vec::new().push(1);
    alloc::vec::Vec::new().push(1);
    alloc::vec::Vec::new().push(1);
    alloc::vec::Vec::new().push(1);
    println!("vec");
    use core::alloc::GlobalAlloc;
    GLOBAL_ALLOC.alloc(core::alloc::Layout::from_size_align(384usize, 8).unwrap());
    println!("me");
    compile::Compiler::parse(&_binary_compile_wasm_binaries_test_wasm_start);
    //compile::Compiler::parse(b"");
    //println!("Hello QEMU 2!");
    println!(
        "bytes: {:?}",
        &_binary_compile_wasm_binaries_test_wasm_start[..]
    );
    panic!("Stopping...")
}

#[cfg(test)]
fn main() {
    ()
}
