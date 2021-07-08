# arm-binaries
To test my compiler (i.e., code generator) implementation.

## usage
```
# To compile
$ aarch64-none-elf-gcc -march=armv8-a <ASM FILE> -c [-o <OUT OBJ FILE>]
# To objdump
$ aarch64-none-elf-objdump <OBJ FILE>
```
