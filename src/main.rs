#![feature(global_asm)]
#![feature(asm)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[cfg(not(test))]
#[macro_use]
mod print;
#[cfg(not(test))]
mod bsp;
#[cfg(not(test))]
mod cpu;
#[cfg(not(test))]
mod memory;
#[cfg(not(test))]
mod panic_wait;
#[cfg(not(test))]
mod runtime_init;
#[cfg(not(test))]
mod console;

#[global_allocator]
static GLOBAL_ALLOC: memory::KernelAllocator = memory::KernelAllocator;

extern crate compiler;

unsafe fn kernel_init() -> ! {
    println!("Hello QEMU!");
    panic!("Stopping")
}

#[cfg(test)]
fn main() {
    ()
}