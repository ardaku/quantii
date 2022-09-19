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

use core::arch::asm;

pub struct Mailbox<const MAILBOX_BASE: usize>;

impl<const MAILBOX_BASE: usize> Mailbox<MAILBOX_BASE> {
    pub const MAILBOX_READ: usize = MAILBOX_BASE;
    pub const MAILBOX_POLL: usize = MAILBOX_BASE + 0x10;
    pub const MAILBOX_SENDER: usize = MAILBOX_BASE + 0x14;
    pub const MAILBOX_STATUS: usize = MAILBOX_BASE + 0x18;
    pub const MAILBOX_CONFIG: usize = MAILBOX_BASE + 0x1C;
    pub const MAILBOX_WRITE: usize = MAILBOX_BASE + 0x20;

    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    pub fn get_base(&self) -> usize {
        MAILBOX_BASE
    }

    pub fn write(&self, data: usize) {
        unsafe {
            while ({
                let x: usize;
                asm!("ldr {}, [{}]", out(reg) x, in(reg) Self::MAILBOX_STATUS);
                x
            } & 0x80000000) != 0 {}
            asm!("str {}, [{}]", in(reg) data, in(reg) Self::MAILBOX_WRITE);
        }
    }

    /// Read the top 28 bits of the mailbox
    pub fn read(&self, channel: usize,) -> usize {
        unsafe {
            let mut data: usize;
            while {
                loop {
                    asm!("ldr {}, [{}]", out(reg) data, in(reg) Self::MAILBOX_READ);
                    if data & 0xF == channel {
                        break;
                    }
                }
                data & 0x80000000 != 0
            } {}
            data & 0xFFFFFFF0
        }
    }

    pub fn read_poll(&self, channel: usize) -> usize {
        unsafe {
            let mut data: usize;
            loop {
                asm!("ldr {}, [{}]", out(reg) data, in(reg) Self::MAILBOX_POLL);
                if (data & 0xF) == channel {
                    break;
                }
            }
            data
        }
    }

    pub fn get_sender(&self) -> usize {
        unsafe {
            let x: usize;
            asm!("ldr {}, [{}]", out(reg) x, in(reg) Self::MAILBOX_SENDER);
            x
        }
    }

    pub fn get_status(&self) -> usize {
        unsafe {
            let x: usize;
            asm!("ldr {}, [{}]", out(reg) x, in(reg) Self::MAILBOX_STATUS);
            x
        }
    }

    pub fn get_config(&self) -> usize {
        unsafe {
            let x: usize;
            asm!("ldr {}, [{}]", out(reg) x, in(reg) Self::MAILBOX_CONFIG);
            x
        }
    }

    pub fn set_config(&self, config: usize) {
        unsafe {
            asm!("str {}, [{}]", in(reg) config, in(reg) Self::MAILBOX_CONFIG);
        }
    }
}

/// The info to be written to instantiate the framebuffer
#[derive(Debug, Clone, Copy)]
pub struct FramebufferInfo {
    pub width: u32,
    pub height: u32,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub pitch: u32,
    pub depth: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub pointer: u32,
    pub size: u32,
}

impl FramebufferInfo {

    pub fn send<const MAILBOX_BASE: usize>(&self, mailbox: &Mailbox<MAILBOX_BASE>) {
        let message = self.to_message();
    }

    fn to_message(&self) -> Message {

    }
}

/// Message must be 36 bytes long, and must be 4-byte aligned
#[derive(Debug, Clone, Copy)]
#[repr(C, align(4))]
pub struct Message {
    pub data: [usize; 9],
}