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

//! Quantii's real entry point.
//!
//! Enters at [`setup`]

#![no_std]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::restriction)]
#![allow(clippy::blanket_clippy_restriction_lints)]
// Allow: `clippy::implicit_returns`
// coincides with `clippy::return_used`
#![allow(clippy::implicit_return)]
// Allow: type [`Result<(), <unknown>>`]
// cannot be easily unwrapped.
#![allow(unused_results)]
// Allow: Same as `unused_results`
#![allow(unused_must_use)]

extern crate alloc;
extern crate novuskinc;
// Missing intrinsics patch.
extern crate compiler_builtins_patch;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use ardaku::Error as ArdakuError;
use novuskinc::kernel::syscalls;

/// Novusk config
pub mod novusk;

/// The executable WASM for the kernel's initially running App
const APP_EXE: &[u8] = include_bytes!("hello_world.wasm");

/// Wrapper for the kernel's syscall interface.
struct System;

impl ardaku::System for System {
    /// Sleep until an event interrupt occurs.
    fn sleep(&self) -> (ardaku::Event, u32) {
        let byte = novuskinc::kernel::io::safe_sys_read();
        (ardaku::Event::Read, byte.into())
    }

    /// Write to the console.
    ///
    /// Note that the console is not shown to the actual onscreen
    /// OS (QEMU/emulators, VMs, or real hardware), but rather
    /// in a separate console. For QEMU, this would be the
    /// console used to start the QEMU process.
    fn write(&self, line: &[u8]) {
        for c in line {
            novuskinc::kernel::io::safe_sys_write(*c);
        }
        novuskinc::kernel::io::safe_sys_write(b'\n');
    }

    /// Return version of the kernel.
    fn version(&self) -> u32 {
        #[cfg(not(target_arch = "riscv32"))]
        // Safety: Syscalls are generally unsafe.
        // In this case it is okay, as this is an
        // operating system.
        unsafe {
            syscalls::syscall(syscalls::VERSION, 0).into()
        }

        #[cfg(target_arch = "riscv32")]
        {
            // TODO: Implement a better versioning system that works for RISC-V.
            3
        }
    }

    /// Reboot the system.
    ///
    /// This does not work on RISC-V targets.
    fn reboot(&self) {
        // This is a no-op on RISC-V.
        #[cfg(target_arch = "riscv32")]
        Self::write(self, b"Reboot not supported on RISC-V");

        // On all other targets, proceed to reboot.
        #[cfg(not(target_arch = "riscv32"))]
        // Safety: Syscalls are generally unsafe.
        // In this case it is okay, as this is an
        // operating system.
        unsafe {
            syscalls::syscall(syscalls::REBOOT, 0);
        };
    }
}

/// The OS's entry point.
///
/// This function is called by the main function
/// in `main.rs` when it is ready to run.
#[inline]
// Allow: requires extra allocation do use [`push_str`]
#[allow(clippy::string_add)]
pub fn setup() -> ! {
    use ardaku::System;

    System.write(b"Quantii OS v0.0.1");

    System.write(b"\n=== ARDAKU STARTED ===\n");

    let combined: String;
    let msg = match ardaku::start(System, APP_EXE) {
        Ok(_) => "\n=== ARDAKU SUCCESSFULLY EXECUTED ===\n",
        Err(e) => match e {
            ArdakuError::InvalidWasm => "Ardaku: Error: Invalid WASM file",
            ArdakuError::LinkerFailed => {
                "Ardaku: Error: Failed to link WASM file"
            }
            ArdakuError::Crash(c) => {
                combined = "Ardaku: Error: Crash: ".to_owned()
                    + c.to_string().as_str();
                combined.as_str()
            }
            ArdakuError::MissingMemory => "Ardaku: Error: Ran out of memory",
        },
    };

    System.write(msg.as_bytes());

    System.write(b"=== ARDAKU STOPPED ===\n");

    // TODO: Do something more proper after the app has stopped.
    loop {
        System.sleep();
    }
}
