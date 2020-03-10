#!/usr/bin/env bash

set -e

# Create bin file
riscv64-unknown-elf-objcopy $1 -O binary $1.bin

# Program iCEBreaker
iceprog -o 0x00040000 $1.bin
