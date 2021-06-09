#![feature(global_asm)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod memory;
mod panic_wait;
mod runtime_init;

mod compiler;

unsafe fn kernel_init() -> ! {
    panic!()
}
