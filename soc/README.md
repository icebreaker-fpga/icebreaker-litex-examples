This is an example Risc-V SOC for the iCEBreaker FPGA.

The goal is to create a simple SOC that can be programmed from C, Rust or micropython.

## Installation

Install litex: https://github.com/enjoy-digital/litex#quick-start-guide

Run the SOC build script and upload the bitstream to your iCEBreaker:
```
python ./icebreaker.py --cpu --debug
iceprog soc_basesoc_icebreaker/gateware/top.bin
```

Go to one of the language example repositories and follow the instructions in
the respective README.md.

NOTE: You can omit the `--debug` parameter for the litex build. This will result in a smaller SOC but you will loose the wishbone bridge that allows direct memory inspection and GDB debugging of your SOC. If you choose to omit the `--debug` option the UART can be used directly without the wishbone-tool.