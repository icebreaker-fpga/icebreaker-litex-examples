# PAC for the iCEBreaker litex Risc-V Example SOC peripherals

SVD is generated from the soc generator script located in `../../soc`.

To generate the SVD file run the SOC generation script with the
`--csr-svd iCEBESOC.svd` parameter.

All you need to do to regenerate the files is replace `iCEBESOC.svd`.
The `build.rs` will see this file has changed and then regenerate `src/pac.rs`.
