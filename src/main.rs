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
    println!(
        "wasm bytes: {:?}",
        &_binary_compile_wasm_binaries_test_wasm_start[..]
    );
    panic!("Stopping...")
}

#[cfg(test)]
fn main() {
    ()
}
