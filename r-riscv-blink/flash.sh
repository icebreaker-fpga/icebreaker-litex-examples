#!/usr/bin/env bash

NAME=r-riscv-blink

# Create bin file
riscv64-unknown-elf-objcopy target/riscv32i-unknown-none-elf/release/$NAME -O binary $NAME.bin

# Program iCEBreaker
iceprog -o 0x00040000 $NAME.bin
