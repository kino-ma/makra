#![feature(global_asm)]
#![feature(asm)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(alloc_prelude)]
#![feature(alloc_error_handler)]
#![feature(trait_alias)]
#![feature(const_fn_fn_ptr_basics)]

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
mod driver;
#[cfg(not(test))]
mod synchronization;
#[cfg(not(test))]
mod time;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOC: memory::KernelAllocator = memory::KernelAllocator;

extern crate compile;

unsafe fn kernel_init() -> ! {
    memory::init();
    kernel_main();
    panic!("Stopping...")
}

fn kernel_main() {
    println!("Hello QEMU!");

    let wasm_binary = memory::wasm_binary();
    println!("read wasm binary: {:x?}", &wasm_binary[..]);

    println!(
        "module address: {:x?} ~ {:x?} ({:x?})",
        memory::module_text_start(),
        memory::module_text_end(),
        memory::module_text_end() - memory::module_text_start()
    );

    let module = Compiler::parse(wasm_binary).expect("failed to parse");

    let func_bin = module.generate().expect("failed to generate");
    let call_res: usize = unsafe { call_binary(&func_bin) };

    println!("function result: 10 + 20 = {:?}", call_res);
}

unsafe extern "C" fn call_binary<T>(bin: &[u8]) -> T {
    asm!("nop");
    let mut func_mem = memory::module_text_start() as *mut u8;

    core::ptr::copy(bin.as_ptr(), func_mem, bin.len());
    let func_ptr = core::mem::transmute::<_, fn() -> T>(func_mem);

    func_ptr()
}

#[cfg(test)]
fn main() {
    ()
}
