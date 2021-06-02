use crate::runtime_init;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    runtime_init::runtime_init()
}
