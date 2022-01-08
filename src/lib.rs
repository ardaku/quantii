#![no_std]

extern crate novuskinc;
// Missing intrinsics patch.
extern crate compiler_builtins_patch;

use novuskinc::kernel::syscalls;

pub mod novusk;

const APP_EXE: &[u8] = include_bytes!("hello_world.wasm");

struct System;

impl ardaku::System for System {
    fn sleep(&self) -> (ardaku::Event, u32) {
        let byte = novuskinc::kernel::io::safe_sys_read();
        (ardaku::Event::Read, byte.into())
    }

    fn write(&self, line: &[u8]) {
        for c in line {
            novuskinc::kernel::io::safe_sys_write(*c);
        }
        novuskinc::kernel::io::safe_sys_write(b'\n');
    }

    fn version(&self) -> u32 {
        #[cfg(not(target_arch = "riscv32"))]
        unsafe { syscalls::syscall(syscalls::VERSION, 0) }.into();

        #[cfg(target_arch = "riscv32")]
        return 3;
    }

    fn reboot(&self) {
        #[cfg(not(target_arch = "riscv32"))]
        unsafe { syscalls::syscall(syscalls::REBOOT, 0) };
    }
}

pub fn arc_setup() -> ! {
    use ardaku::System;

    System.write(b"\n\n=== ARDAKU STARTED ===\n");

    ardaku::start(System, APP_EXE).unwrap();

    System.write(b"\n=== ARDAKU STOPPED ===\n");

    loop {
        System.sleep();
    }
}
