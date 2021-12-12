#![no_std]

#[macro_use] extern crate novuskinc;

use novuskinc::kernel::syscalls;

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
        unsafe { syscalls::syscall(syscalls::VERSION, 0) }.into()
    }

    fn reboot(&self) {
        unsafe { syscalls::syscall(syscalls::REBOOT, 0) };
    }
}

pub fn arc_setup() {
    ardaku::start(System, APP_EXE).unwrap();
}
