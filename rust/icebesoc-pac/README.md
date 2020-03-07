# PAC for the iCEBreaker litex Risc-V Example SOC peripherals

SVD is generated from the soc generator script located in `../../soc`.

All you need to do to regenerate the files is replace `iCEBESOC.svd`.
The `build.rs` will see this file has changed and then regenerate `src/pac.rs`.
