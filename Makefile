ARCH =?
TARGET =?
IMAGE_CMD =?

ifeq ($(ARCH), aarch64)
	TARGET = aarch64-novusk.json
	IMAGE_CMD = aarch64-linux-gnu-objcopy --strip-all -O binary target/aarch64-novusk/release/arc target/aarch64-novusk/release/kernel8.img
endif

ifeq ($(ARCH), riscv)
	TARGET = riscv32imac-unknown-none-elf
	IMAGE_CMD = mv target/riscv32imac-unknown-none-elf/release/arc target/riscv32imac-unknown-none-elf/release/arc_os.img
endif

all: build image


build:
	@ echo "Compiling Arc..."
	@ cargo build --release --target $(TARGET)

image:
	@ echo "Creating image..."
	@ $(IMAGE_CMD)
