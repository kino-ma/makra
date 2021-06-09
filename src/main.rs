#![feature(global_asm)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

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

mod compiler;

unsafe fn kernel_init() -> ! {
    panic!()
}

#[cfg(test)]
fn main() {
    ()
}