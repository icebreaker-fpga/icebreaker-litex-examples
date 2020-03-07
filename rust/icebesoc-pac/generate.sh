#!/usr/bin/env bash

svd2rust -i iCEBESOC.svd --target riscv
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
sed -i 's/extern crate riscv;/extern crate vexriscv as riscv;/' src/lib.rs
sed -i 's/extern crate riscv_rt;/extern crate fomu_rt as riscv_rt;/' src/lib.rs
