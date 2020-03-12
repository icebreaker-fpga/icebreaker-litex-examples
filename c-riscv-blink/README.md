Welcome to the C example code for LiteX RISC-V SOC on iCEBreaker.

## Install

Make sure you have the soc installed as described in the `soc` directory of this repository.

Build and flash the firmware for your soc by running:
```
make
make prog
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
