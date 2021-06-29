use crate::runtime_init;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    println!("hoge");
    asm!(include_str!("test-binary.s"));
    runtime_init::runtime_init()
}
