
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

    #[cfg(feature = "rpi")]
    {
        let mut fb = A64Fb::new();
        fb.init();
        fb.clear_screen(FbColor::new(0xFFFFFF, 0, 0));
        fb.pixel(20, 20, FbColor::new(0, 0, 0));
        fb.pixel(20, 21, FbColor::new(0, 0, 0));
        fb.pixel(20, 22, FbColor::new(0, 0, 0));
        fb.pixel(20, 23, FbColor::new(0, 0, 0));
        fb.pixel(20, 24, FbColor::new(0, 0, 0));
        fb.pixel(20, 25, FbColor::new(0, 0, 0));
        fb.pixel(20, 26, FbColor::new(0, 0, 0));
        fb.pixel(20, 27, FbColor::new(0, 0, 0));
        fb.pixel(20, 28, FbColor::new(0, 0, 0));
        fb.pixel(20, 29, FbColor::new(0, 0, 0));
        fb.pixel(20, 30, FbColor::new(0, 0, 0));
        fb.pixel(21, 20, FbColor::new(0, 0, 0));
        fb.pixel(21, 21, FbColor::new(0, 0, 0));
        fb.pixel(21, 22, FbColor::new(0, 0, 0));
        fb.pixel(21, 23, FbColor::new(0, 0, 0));
        fb.pixel(21, 24, FbColor::new(0, 0, 0));
        fb.pixel(21, 25, FbColor::new(0, 0, 0));
        fb.pixel(21, 26, FbColor::new(0, 0, 0));
        fb.pixel(21, 27, FbColor::new(0, 0, 0));
        fb.pixel(21, 28, FbColor::new(0, 0, 0));
        fb.pixel(21, 29, FbColor::new(0, 0, 0));
        fb.pixel(21, 30, FbColor::new(0, 0, 0));
        fb.pixel(22, 20, FbColor::new(0, 0, 0));
        fb.pixel(22, 21, FbColor::new(0, 0, 0));
        fb.pixel(22, 22, FbColor::new(0, 0, 0));
        fb.pixel(22, 23, FbColor::new(0, 0, 0));
        fb.pixel(22, 24, FbColor::new(0, 0, 0));
        fb.pixel(22, 25, FbColor::new(0, 0, 0));
        fb.pixel(22, 26, FbColor::new(0, 0, 0));
        fb.pixel(22, 27, FbColor::new(0, 0, 0));
        fb.pixel(22, 28, FbColor::new(0, 0, 0));
        fb.pixel(22, 29, FbColor::new(0, 0, 0));
        fb.pixel(22, 30, FbColor::new(0, 0, 0));
        fb.pixel(23, 20, FbColor::new(0, 0, 0));
        fb.pixel(23, 21, FbColor::new(0, 0, 0));
        fb.pixel(23, 22, FbColor::new(0, 0, 0));
        fb.pixel(23, 23, FbColor::new(0, 0, 0));
        fb.pixel(23, 24, FbColor::new(0, 0, 0));
        fb.pixel(23, 25, FbColor::new(0, 0, 0));
        fb.pixel(23, 26, FbColor::new(0, 0, 0));
        fb.pixel(23, 27, FbColor::new(0, 0, 0));
        fb.pixel(23, 28, FbColor::new(0, 0, 0));
        fb.pixel(23, 29, FbColor::new(0, 0, 0));
        fb.pixel(23, 30, FbColor::new(0, 0, 0));
        fb.pixel(24, 20, FbColor::new(0, 0, 0));
        fb.pixel(24, 21, FbColor::new(0, 0, 0));
        fb.pixel(24, 22, FbColor::new(0, 0, 0));
        fb.pixel(24, 23, FbColor::new(0, 0, 0));
        fb.pixel(24, 24, FbColor::new(0, 0, 0));
        fb.pixel(24, 25, FbColor::new(0, 0, 0));
        fb.pixel(24, 26, FbColor::new(0, 0, 0));
        fb.pixel(24, 27, FbColor::new(0, 0, 0));
        fb.pixel(24, 28, FbColor::new(0, 0, 0));
        fb.pixel(24, 29, FbColor::new(0, 0, 0));
        fb.pixel(24, 30, FbColor::new(0, 0, 0));
        fb.pixel(25, 20, FbColor::new(0, 0, 0));
        fb.pixel(25, 21, FbColor::new(0, 0, 0));
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn initramfs_main() {}
