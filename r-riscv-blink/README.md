Welcome to the Rust example code for LiteX RISC-V SOC on iCEBreaker.

## Prerequisites

Make sure you have the soc installed as described in the `soc` directory of this repository.

You will also need to install the Rust compiler. Install rustup by following the instructions at https://rustup.rs.

NOTE Make sure you have a compiler version equal to or newer than 1.31. rustc -V should return a date newer than the one shown below.

```
$ rustc -V
rustc 1.31.1 (b6c32da9b 2018-12-18)
```

For bandwidth and disk usage concerns the default installation only supports native compilation. To add cross compilation support for the RISC-V architecture install the `riscv32i-unknown-none-elf` target.

```
rustup target add riscv32i-unknown-none-elf
```

To learn more about Rust embedded, take a look at the Rust [embedded book](https://rust-embedded.github.io/book/).

## Build and Flash

Build and flash the firmware for your soc by running:
```
cargo build --release
cargo run --release
```

The two LED Red and Green should blink alternating.

You should be able to access the console output by running the wishbone-tool.

```
wishbone-tool --uart /dev/ttyUSB1 -s terminal
```

You can find the wishbone-tool here: https://github.com/litex-hub/wishbone-utils

NOTE: If you decided to build your SOC without the `--debug` parameter you can access the console output directly through the iCEBreaker UART port. For example using screen:

```
screen /dev/ttyUSB1 115200
```

To exit screen you can type `Ctrl-a k` or `Ctrl-a Ctrl-k`
