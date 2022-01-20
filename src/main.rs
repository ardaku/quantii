#![no_std]
#![no_main]

extern crate novusk;
extern crate quantii;

use quantii::setup;

// Called from novusk
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    setup()
}

#[no_mangle]
pub extern "C" fn initramfs_main() {}
