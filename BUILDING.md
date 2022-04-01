## Build Environment
You will need the nightly toolchain and sources as well as cargo-binutils and
the binutils-aarch64-linux-gnu, qemu-system-riscv packages installed.

```commandline
rustup toolchain install nightly-2021-12-14
rustup component add rust-src --toolchain nightly-2021-12-14
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

## Build:

Use ``aarch64`` for arm (64 bit), and riscv (32 bit for now)

```commandline
make all ARCH=<architecture>
```

## Test ARM in QEMU
The OS executable will be located at ``./target/aarch64-novusk/release/kernel8.img``
Make sure you have at least Qemu v4.2.1+

```commandline
qemu-system-aarch64 -M raspi3 -kernel target/aarch64-novusk/release/kernel8.img -serial null -serial stdio
```

## Test RISC-V in QEMU
The OS executable will be located at ``./target/riscv32imac-unknown-none-elf/release/quantii.img``
Make sure you have at least Qemu v4.2.1+

```commandline
qemu-system-riscv32 -nographic -machine sifive_e -m 128M -kernel target/riscv32imac-unknown-none-elf/release/quantii.img -serial 'mon:stdio' -bios none
```

## Test on Raspberry Pi

[TODO](link_to_novusk_docs)

Install the Raspberry Pi boot files from [here](https://github.com/raspberrypi/firmware/tree/master/boot) and replace 
``kernel8.img`` with ``target/aarch64-novusk/release/kernel8.img``
