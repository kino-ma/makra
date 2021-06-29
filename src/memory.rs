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

#[derive(Clone, Copy, Default)]
struct FreeMemoryInfo {
    addr: usize,
    size: usize,
}

const MAX_FREES: usize = 4090;

static mut FREES: usize = MAX_FREES;
static mut FREE: [FreeMemoryInfo; MAX_FREES] = [FreeMemoryInfo; MAX_FREES];

extern {
    static __kernel_heap_start__: usize;
    static __kernel_heap_end__: usize;
}

#[inline]
fn kernel_heap_start() -> usize {
    &__kernel_heap_start__ as *const _ as usize
}

#[inline]
fn kernel_heap_end() -> usize{
    &__kernel_heap_end__ as *const _ as usize
}
pub unsafe fn init() {
    FREES = 1;
    FREE[0] = FreeMemoryInfo {
        addr: kernel_heap_start(),
        size: (kernel_heap_end() - kernel_heap_start()) as usize
    };
}

fn aligned_size(layout: &Layout) -> usize {
    let s = layout.size();
    let a = layout.align();
    return a * ((s / a) + if s % a > 0 {1} else {0});
}

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    // FIXME: ensure that return address is multiple of layout.align()
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        for i in 0..FREES {
            let size = aligned_size(&layout);

            if FREE[i].size >= size {
                let addr = FREE[i].addr as *mut u8;
                FREE[i].addr += size as usize;
                FREE[i].size -= size;
                if FREE[i].size == 0 {
                    FREES -= 1;
                    for j in i..FREES {
                        FREE[j] = FREE[j+1];
                    }
                }
                return addr
            }
        }

        0 as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let addr = ptr as usize;
        let size = aligned_size(&layout);

        let mut i = 0usize;
        for j in 0..FREES {
            if FREE[i].addr > addr {
                i = j;
                break;
            }
        }

        if i > 0 {
            if FREE[i-1].addr + FREE[i-1].size as usize == addr {
                FREE[i-1].size += size;
                if i < FREES {
                    if addr + size as usize == FREE[i].addr {
                        FREE[i-1].size += FREE[i].size;
                        FREES -= 1;
                        for j in i..FREES {
                            FREE[j] = FREE[j+1];
                        }
                    }
                }
                return;
            }
        }

        if i < FREES {
            if addr + size as usize == FREE[i].addr {
                FREE[i].addr = addr;
                FREE[i].size += size;
                return;
            }
        }

        if FREES < MAX_FREES {
            for j in (i+1..=FREES).rev() {
                FREE[j] = FREE[j-1];
            }

            FREES -= 1;

            FREE[i].addr = addr;
            FREE[i].size = size;
            return;
        }

        // FIXME: deallocation failed. abort?
        uart::write("dealloc failed\n");
    }
}

#[alloc_error_handler]
fn foo(_: Layout) -> ! {
    uart::write("alloc_error");
    loop {}
}