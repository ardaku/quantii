
#![no_std]
#![no_main]
#![feature(ptr_metadata)]
#![feature(slice_take)]

extern crate novusk;
extern crate quantii;
// Enable libm
extern crate externc_libm;
extern crate alloc;

use alloc::string::ToString;
use core::arch::{asm, global_asm};
use core::borrow::BorrowMut;
use core::ptr;
use quantii::framebuffer::Pixel;
#[cfg(feature = "rpi")]
use quantii::framebuffer::mailbox::Mailbox;
use quantii::{framebuffer, setup};


// Called from novusk
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    use quantii::System;
    use ardaku::System as ASystem;

    setup();

    let mb: Mailbox<0x2000B880> = Mailbox::new();
    System.write("Mailbox base: {}".replace("{}", mb.get_base().to_string().as_str()).as_bytes());
    mb.write(0x40000000);
    System.write("Mailbox read: {}".replace("{}", mb.read(8).to_string().as_str()).as_bytes());

    loop {}
}

#[no_mangle]
pub extern "C" fn initramfs_main() {}
