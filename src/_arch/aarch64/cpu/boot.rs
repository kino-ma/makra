use crate::runtime_init;
use crate::kernel_init;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    println!("hoge");
    kernel_init();
    asm!(include_str!("test-binary.s"));
    runtime_init::runtime_init()
}
