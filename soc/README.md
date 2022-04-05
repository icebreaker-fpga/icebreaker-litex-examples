This is an example Risc-V SOC for the iCEBreaker FPGA.

The goal is to create a simple SOC that can be programmed from C, Rust or
micropython.

## Prerequisites

1. Install the latest [YosysHQ toolchain](https://github.com/YosysHQ/oss-cad-suite-build#installation). Older versions (fomu-toolchain and open-tool-forge) will not optimize the design enough to fit the fpga.

2. Install the [riscv toolchiain](https://github.com/sifive/freedom-tools/releases)

Both are installed by unzipping and adding the \*/bin directory to your path

## Installation

Run the SOC build script and upload the bitstream to your iCEBreaker:
```
./icebreaker.py --debug --flash
```

NOTE: The script should automatically download all python dependencies. You
only have to provide the FPGA tools and riscv compiler.

Go to one of the language example repositories and follow the instructions in
the respective README.md.

NOTE: You can omit the `--debug` parameter for the litex build. This will
result in a smaller SOC but you will loose the wishbone bridge that allows
direct memory inspection and GDB debugging of your SOC. If you choose to omit
the `--debug` option the UART can be used directly without the wishbone-tool.

## Customization

Because the SOC is a softcore for an FPGA we strive to provide a few examples
of customization the SOC configuration. Here is a non exhaustive list of a few
interesting parameters you can play with. Try the `--help` parameter to see the
full list of parameters that our litex script accepts.

### `--sys-clk-freq` System Clock Frequency

You can choose a system clock frequency. The default external clock connected
to the iCE40UP5k on the iCEBreaker is 12MHz (aka. 12e6). The default clock we
select in our script is 21MHz (aka. 21e6). This causes our script to use a
special PLL block in the FPGA to generate a 21MHz clock out of the 12MHz input
clock. You can try selecting other clocks too and the script will try to
generate a clock frequency that is as close as possible to the one you
requested.

## Documentation

The generator script also autogenerates Sphinx documentation that describes the
peripheral registers, their addresses and functionality. You can find the
generated documentation
[here](https://icebreaker-fpga.github.io/icebreaker-litex-examples/).
