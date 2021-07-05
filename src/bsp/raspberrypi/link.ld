__rpi_load_addr = 0x80000;

ENTRY(__rpi_load_addr)

PHDRS
{
    segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
    segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
    segment_rwx PT_LOAD FLAGS(7); /* 7 == RWX */
}

SECTIONS
{
    . =  __rpi_load_addr;
                                        /*   ^             */
                                        /*   | stack       */
                                        /*   | growth      */
                                        /*   | direction   */
   __boot_core_stack_end_exclusive = .; /*   |             */

    .text :
    {
        KEEP(*(.text._start))
        *(.text._start_arguments) /* Constants (or statics in Rust speak) read by _start(). */
        *(.text._start_rust)      /* The Rust entry point */
        *(.text*)                 /* Everything else */
    } :segment_rx

    .rodata : ALIGN(8) { *(.rodata*) } :segment_rx
    .got    : ALIGN(8) { *(.got)     } :segment_rx

    .data : { *(.data*) } :segment_rw

    .bss : ALIGN(8)
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(8);

        . += 8; /* Fill for the bss == 0 case, so that __bss_start <= __bss_end_inclusive holds */
        __bss_end_inclusive = . - 8;
    } :NONE

    /* allocate 10KBytes for heap
    ** ailgned 4096 bytes
    */
    .kernelheap : {
        __kernel_heap_start__ = .;
        . += 0x100000;
    }

    . = ALIGN(4096);
    __kernel_heap_end__ = .;

    .moduletext : ALIGN(8) {
        __module_text_start__ = .;
        . += 0x100000;
    } :segment_rwx

    . = ALIGN(4096);
    __module_text_end__ = .;
}
