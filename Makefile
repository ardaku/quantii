ARCH =?
TARGET =?

ifeq ($(ARCH), aarch64)
	TARGET = aarch64-novusk.json
endif

ifeq ($(ARCH), riscv)
	TARGET = riscv32imac-unknown-none-elf
endif

all:
	@ echo "Compiling Arc..."
	@ cargo +nightly build --release --target $(TARGET)
