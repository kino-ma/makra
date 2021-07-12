# compare
./compile.sh test > /dev/null &&  paste  <(pbpaste | fold -w2 | sed 's/^/0x/' | xxd -r -p > a.o && aarch64-none-elf-objdump -D -b binary -maarch64 a.o) <(aarch64-none-elf-objdump -D -b binary -maarch64 test.bin)
# diff
./compile.sh test > /dev/null &&  diff  <(pbpaste | fold -w2 | sed 's/^/0x/' | xxd -r -p > a.o && aarch64-none-elf-objdump -D -b binary -maarch64 a.o) <(aarch64-none-elf-objdump -D -b binary -maarch64 test.bin)
