use crate::kernel_init;
use crate::runtime_init;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    kernel_init();
}
