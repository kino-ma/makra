use core::ops::RangeInclusive;

pub unsafe fn zero_volatile<T>(range: RangeInclusive<*mut T>)
where
    T: From<u8>,
{
    let mut ptr = *range.start();
    let end_inclusive = *range.end();

    while ptr <= end_inclusive {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}

use core::alloc::{GlobalAlloc, Layout};

#[derive(Clone, Copy)]
struct FreeMemoryInfo {
    addr: usize,
    size: usize,
}

const MAX_FREES: usize = 4090;

static mut FREES: usize = MAX_FREES;
static mut FREE: [FreeMemoryInfo; MAX_FREES] = [FreeMemoryInfo { addr: 0, size: 0 }; MAX_FREES];

extern "C" {
    static __kernel_heap_start__: usize;
    static __kernel_heap_end__: usize;
    static __module_text_start__: usize;
    static __module_text_end__: usize;
    static _binary_compile_wasm_binaries_test_wasm_start: usize;
    static _binary_compile_wasm_binaries_test_wasm_end: usize;
    static _binary_compile_wasm_binaries_test_wasm_size: usize;
}

#[inline]
fn kernel_heap_start() -> usize {
    unsafe { &__kernel_heap_start__ as *const _ as usize }
}

#[inline]
fn kernel_heap_end() -> usize {
    unsafe { &__kernel_heap_end__ as *const _ as usize }
}

pub unsafe fn init() {
    FREES = 1;
    FREE[0] = FreeMemoryInfo {
        addr: kernel_heap_start(),
        size: (kernel_heap_end() - kernel_heap_start()) as usize,
    };
}

fn aligned_size(layout: &Layout) -> usize {
    let s = layout.size();
    let a = layout.align();
    return a * ((s / a) + if s % a > 0 { 1 } else { 0 });
}

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    // FIXME: ensure that return address is multiple of layout.align()
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = aligned_size(&layout);

        for i in 0..FREES {
            if FREE[i].size >= size {
                let addr = FREE[i].addr as *mut u8;
                FREE[i].addr += size as usize;
                FREE[i].size -= size;
                if FREE[i].size == 0 {
                    FREES -= 1;
                    for j in i..FREES {
                        FREE[j] = FREE[j + 1];
                    }
                }
                return addr;
            }
        }

        0 as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // For easiness, just mark "unused" currently.
        // We may concatenate free space or more efficent management may be used in the future.
        if FREES >= MAX_FREES {
            panic!("reached max allocation count");
        }

        FREE[FREES] = FreeMemoryInfo {
            size: layout.size(),
            addr: ptr as usize,
        };
    }
}

#[alloc_error_handler]
fn foo(layout: Layout) -> ! {
    println!("alloc_error: {:?}", layout);
    unsafe {
        for f in &FREE[..10] {
            println!("addr: {:?}, size: {:?}", f.addr, f.size);
        }
    }
    loop {}
}

pub fn module_text_start() -> usize {
    unsafe { &__module_text_start__ as *const _ as usize }
}

pub fn module_text_end() -> usize {
    unsafe { &__module_text_end__ as *const _ as usize }
}

pub fn wasm_binary() -> &'static [u8] {
    let s = unsafe {
        let start = &_binary_compile_wasm_binaries_test_wasm_start as *const _ as _;
        let size = _binary_compile_wasm_binaries_test_wasm_size;
        let size = if size <= 0 { 40 } else { size };
        core::slice::from_raw_parts(start, size)
    };
    println!("s[0] {}", s[0]);
    s
}
