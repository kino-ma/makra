__rpi_load_addr = 0x80000;

ENTRY(__rpi_load_addr)

SECTIONS
{
    . = __rpi_load_addr;
    __boot_core_stack_end_exclusive = .;
    .text :
    {
        *(.text*)
    }
}
