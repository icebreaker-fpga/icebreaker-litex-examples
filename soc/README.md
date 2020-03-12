This is an example Risc-V SOC for the iCEBreaker FPGA.

The goal is to create a simple SOC that can be programmed from C, Rust or
micropython.

## Installation

Run the SOC build script and upload the bitstream to your iCEBreaker:
```
./icebreaker.py --debug --flash
```

NOTE: The script should automatically download all python dependencies. You
only have to provide the FPGA tools and riscv compiler.

TODO: Add information how to get FPGA tools and the riscv compiler.

Go to one of the language example repositories and follow the instructions in
the respective README.md.

NOTE: You can omit the `--debug` parameter for the litex build. This will
result in a smaller SOC but you will loose the wishbone bridge that allows
direct memory inspection and GDB debugging of your SOC. If you choose to omit
the `--debug` option the UART can be used directly without the wishbone-tool.

## Documentation

The generator script also autogenerates Sphinx documentation that describes the
peripheral registers, their addresses and functionality. You can find the
generated documentation
[here](https://icebreaker-fpga.github.io/icebreaker-litex-examples/).
