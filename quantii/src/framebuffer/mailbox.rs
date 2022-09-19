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

    pub fn new() -> Self {
        Self{}
    }

    pub fn get_base() -> usize {
        MAILBOX_BASE
    }
}

#[repr(C, align(16))]
pub struct MailboxBuffer<const TAG_COUNT: usize> {
    size: u32,
    code: u32,
    tags: [MailboxTag; TAG_COUNT],
    end_tag: MailboxTag,
}

impl<const TAG_COUNT: usize> MailboxBuffer<TAG_COUNT> {
    pub fn new(tags: [MailboxTag; TAG_COUNT]) -> Self {
        Self {
            size: 0,
            code: 0,
            tags,
            end_tag: MailboxTag::End,
        }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }

    pub fn get_tags(&self) -> &[MailboxTag] {
        &self.tags
    }

    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    pub fn set_code(&mut self, code: u32) {
        self.code = code;
    }

    pub fn set_tags(&mut self, tags: [MailboxTag; TAG_COUNT]) {
        self.tags = tags;
    }

    pub unsafe fn read(&mut self, channel: u32) {
        let mut buffer = self as *mut Self as usize;
        buffer += 0x40000000;
        asm!("
            mov x0, {buffer}
            mov x1, {mailbox}
            ldr x2, =0x80000000
            orr x0, x0, x2
            str x0, [x1]
            ",
            buffer = in(reg) buffer,
            mailbox = in(reg) Mailbox::<0x3F00B880>::MAILBOX_WRITE,
            options(nostack, preserves_flags));

    }

    pub unsafe fn write(&mut self) {
        let mut buffer = self as *mut Self as usize;
        buffer += 0x40000000;
        asm!("
            mov x0, {buffer}
            mov x1, 0x80000000
            str x1, [{mailbox}, #{write}]
            dmb sy
        ",
            buffer = in(reg) buffer,
            mailbox = in(reg) Mailbox::<0x3F00B880>::get_base(),
            write = const Mailbox::<0x3F00B880>::MAILBOX_WRITE,
            options(nomem, nostack, preserves_flags)
        );
    }
}
pub enum MailboxTag {
    End = 0x0,
    VCGetFirmwareRevision = 0x00000001,
    HWGetBoardModel = 0x00010001,
    HWGetBoardRevision = 0x00010002,
    HWGetBoardMACAddress = 0x00010003,
    HWGetBoardSerial = 0x00010004,
    HWGetARMMemory = 0x00010005,
    HWGetVCMemory = 0x00010006,
    HWGetClocks = 0x00010007,
    HWGetCommandline = 0x00050001,
    HWGetDMAChannels = 0x00060001,
    HWGetPowerState = 0x00020001,
    HWGetTiming = 0x00020002,
    HWSetPowerState = 0x00028001,
    HWSetClockState = 0x00038001,
}