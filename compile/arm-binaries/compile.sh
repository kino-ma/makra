#!/bin/sh
name=$1
aarch64-none-elf-gcc -march=armv8-a $name.S -c -o $name.o && aarch64-none-elf-objdump -D $name.o
