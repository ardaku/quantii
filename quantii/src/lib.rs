// Copyright (c) 2022 The Quantii Contributors
//
// This file is part of Quantii.
//
// Quantii is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Quantii is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU
// Lesser General Public License along with
// Quantii. If not, see <https://www.gnu.org/licenses/>.

#![no_std]

extern crate novuskinc;
// Missing intrinsics patch.
extern crate compiler_builtins_patch;
extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use ardaku::Error as ArdakuError;
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
        unsafe {
            syscalls::syscall(syscalls::VERSION, 0).into()
        }

        #[cfg(target_arch = "riscv32")]
        {
            3
        }
    }

    fn reboot(&self) {
        #[cfg(not(target_arch = "riscv32"))]
        unsafe {
            syscalls::syscall(syscalls::REBOOT, 0)
        };
    }
}

pub fn setup() -> ! {
    use ardaku::System;

    System.write(b"Quantii OS v0.0.1");

    System.write(b"\n=== ARDAKU STARTED ===\n");

    let mut msg: String;
    match ardaku::start(System, APP_EXE) {
        Ok(_) => msg = "\n=== ARDAKU SUCCESSFULLY EXECUTED ===\n".to_owned(),
        Err(e) => {
            match e {
                ArdakuError::InvalidWasm => msg = "Ardaku: Error: Invalid WASM file".to_owned(),
                ArdakuError::LinkerFailed => msg = "Ardaku: Error: Failed to link WASM file".to_owned(),
                ArdakuError::Crash(c) => {
                    msg = "Ardaku: Error: Crash:".to_owned();
                    msg += &c.to_string();
                },
                ArdakuError::MissingMemory => msg = "Ardaku: Error: Ran out of memory".to_owned(),
            }
        }
    }

    System.write(msg.as_bytes());

    System.write(b"\n=== ARDAKU STOPPED ===\n");

    loop {
        System.sleep();
    }
}
