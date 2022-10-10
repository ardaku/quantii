
#![no_std]
#![no_main]
#![feature(ptr_metadata)]
#![feature(slice_take)]

extern crate quantii;
// Enable libm
extern crate externc_libm;
extern crate alloc;

use alloc::string::ToString;
use ardaku::System;
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
