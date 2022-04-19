/*
 * Copyright © 2022 Quantii Foundation
 * mailto:TODO!("add email")
 *
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License or LICENSE.txt for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

#![no_std]

extern crate novusk;
extern crate novuskinc;
// Missing intrinsics patch.
extern crate compiler_builtins_patch;

// extern crate sha2;

use novuskinc::kernel::syscalls;
// use sha2::Digest;
// use rand::{thread_rng, rngs::ThreadRng, Rng};
// use sha2::digest::Update;

const APP_EXE: &[u8] = include_bytes!("hello_world.wasm");

/*
// pub struct Hasher {
//     hasher: sha2::Sha256,
//     update: bool,
// }
//
// impl Hasher {
//     pub const fn get_next_hash(&mut self) -> TokenStream {
//         if update {
//             self.next_hash();
//             update = false
//         } else {
//             update = true
//         }
//         self.hasher.finalize()
//     }
//
//     const fn next_hash(&mut self) {
//         let rngval: u64 = thread_rng().gen();
//         self.hasher.update(rngval as &[u8]);
//     }
//
//     pub const fn hash(value: &[u8]) {
//         return sha2::Sha256::new().update(value)
//     }
// }
//
// pub struct TermHash {
//     pub level1: Hasher,
//     pub level2: Hasher,
//     pub level3: Hasher
// }
//
// pub const TERM_HASH: TermHash = TermHash {
//     level1: Hasher { hasher: sha2::Sha256::new(), update: false },
//     level2: Hasher { hasher: sha2::Sha256::new(), update: false },
//     level3: Hasher { hasher: sha2::Sha256::new(), update: false },
// }; */
// unimportant

pub struct System;

impl ardaku::System for System {
    fn sleep(&self) -> (ardaku::Event, u32) {
        let byte = novuskinc::kernel::io::safe_sys_read();
        (ardaku::Event::Read, byte.into())
    }

    fn write(&self, line: &[u8]) {
        for c in line {
            novuskinc::kernel::io::safe_sys_write(*c);
        }
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

    fn reboot(&self) {}
}

pub fn setup() {
    use ardaku::System;

    System.write(b"\n\n=== ARDAKU STARTED ===\n");

    ardaku::start(System, APP_EXE).unwrap();

    System.write(b"\n=== ARDAKU STOPPED ===\n");

    // hasher.update(b"");
}
