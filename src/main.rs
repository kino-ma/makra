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
mod driver;
#[cfg(not(test))]
mod memory;
#[cfg(not(test))]
mod panic_wait;
#[cfg(not(test))]
mod runtime_init;
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
    panic!("Finish")
}

fn kernel_main() {
    use crate::time::interface::TimeManager;
    use time::time_manager;

    let tm = time_manager();
    let boot_time = tm.uptime();

    let wasm_binary = memory::wasm_binary();

    let compile_start = tm.uptime();
    let module = Compiler::parse(wasm_binary).expect("failed to parse wasm binary");

    let func_bin = module
        .generate()
        .expect("failed to generate native code from wasm binary");
    let compile_end = tm.uptime();

    let call_start = tm.uptime();
    let call_res = is_prime(32749);
    let call_end = tm.uptime();

    let compile_spent = compile_end - compile_start;
    let call_spent = call_end - call_start;
    let total_spent = tm.uptime();

    println!("function result: is_prime(32749) = {}", call_res);
    println!();

    println!(
        "boot process took: {}.{} ms",
        boot_time.as_millis(),
        boot_time.subsec_micros()
    );
    println!();

    println!("compile process took: -");
    println!();

    println!(
        "function call took: {}.{} ms",
        call_spent.as_millis(),
        call_spent.subsec_micros()
    );

    println!();
    println!(
        "total: {}.{} ms",
        total_spent.as_millis(),
        total_spent.subsec_micros()
    );
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
