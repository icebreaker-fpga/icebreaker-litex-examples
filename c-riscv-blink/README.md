Welcome to the example code for litex riscv soc on iCEBreaker.

== Install ==

Make sure you have the soc installed as described in the root directory of this repository.

Build and flash the firmware for your soc by running:
```
make
iceprog -o 0x00020000 riscv-blink.bin
```

The two LED Red and Green should blink alternating.

You should be able to access the console output by running the wishbone-tool.

```
wishbone-tool --uart /dev/ttyUSB1 -s terminal
```

You can find the wishbone-tool here: https://github.com/litex-hub/wishbone-utils
