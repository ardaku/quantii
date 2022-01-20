ARCH =?
TARGET =?
IMAGE_CMD =?

ifeq ($(ARCH), aarch64)
	TARGET = aarch64-novusk.json
	IMAGE_CMD = aarch64-linux-gnu-objcopy --strip-all -O binary target/aarch64-novusk/release/quantii target/aarch64-novusk/release/kernel8.img
endif

ifeq ($(ARCH), riscv)
	TARGET = riscv32imac-unknown-none-elf
	IMAGE_CMD = mv target/riscv32imac-unknown-none-elf/release/quantii target/riscv32imac-unknown-none-elf/release/quantii.img
endif

all: build image


build:
	@ echo "Compiling Quantii..."
	@ cargo build --release --target $(TARGET)

image:
	@ echo "Creating image..."
	@ $(IMAGE_CMD)
