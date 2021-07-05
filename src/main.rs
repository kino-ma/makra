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
use compile::Compiler;

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
    static _binary_compile_wasm_binaries_test_wasm_start: [u8; 30];
}

unsafe fn kernel_init() -> ! {
    unsafe {
        memory::init();
    }
    println!("Hello QEMU!");
    println!(
        "wasm bytes: {:?}",
        &_binary_compile_wasm_binaries_test_wasm_start[..]
    );

    println!(
        "module: {:?} ~ {:?} ({:?})",
        memory::module_text_start(),
        memory::module_text_end(),
        memory::module_text_end() - memory::module_text_start()
    );

    let module = Compiler::parse(&_binary_compile_wasm_binaries_test_wasm_start[..])
        .expect("failed to parse");
    let func_bin = module.generate().expect("failed to generate");
    println!("{:?}", func_bin);
    let mut func_mem = memory::module_text_start() as *mut u8;
    let res = unsafe {
        core::ptr::copy(func_bin.as_ptr(), func_mem, func_bin.len());
        let func_ptr: extern "C" fn() -> u64 = core::mem::transmute(func_mem);
        func_ptr()
    };
    println!("res: 10 + 20 = {:?}", res);
    panic!("Stopping...")
}

#[cfg(test)]
fn main() {
    ()
}
