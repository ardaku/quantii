
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
use ardaku::System;
use novusk::drivers::gpu::armfb::{armfb_end, armfb_init, BLACK, graphics_pixel, graphics_write, LIGHT_GRAY, WHITE};
use novusk::drivers::gpu::armfb::a64::A64Fb;
use novusk::fb::FbColor;
use novusk::fb::framebuffer::FrameBufferGraphics;
use quantii::setup;


// Called from novusk
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    use quantii::System;
    use ardaku::System as ASystem;

    setup();
    loop {}
}

#[no_mangle]
pub extern "C" fn initramfs_main() {}
