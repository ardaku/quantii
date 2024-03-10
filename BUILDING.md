## Build Environment
You will need the rustup, the nightly toolchain and sources as well as
cargo-binutils and the binutils-aarch64-linux-gnu, qemu-system-riscv packages
installed.

```shell
rustup component add rust-src --toolchain nightly-2024-03-01
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

## Building Instructions
Quantii uses cargo-xtask, so building and testing the OS is easy:

```shell
cargo dist arm # Build OS for AARCH64
cargo qemu arm # Emulate OS in AARCH64 QEMU
cargo dist riscv # Build OS for RISC-V (doesn't work yet)
cargo qemu riscv # Emulate OS in RISC-V QEMU (doesn't work yet)
```

Distribution artifacts go into `/target/dist/`

# Try On Real Hardware

## Test on Raspberry Pi

[TODO](link_to_novusk_docs)

Install the Raspberry Pi boot files from [here](https://github.com/raspberrypi/firmware/tree/master/boot) and replace 
``kernel8.img`` with ``target/aarch64-novusk/release/kernel8.img``

## Test on VisionFive
TODO
