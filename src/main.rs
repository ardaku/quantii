#![no_std]
#![no_main]

extern crate libarc_os;
extern crate novusk;

use libarc_os::arc_setup;

// Called from novusk
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    arc_setup()
}

#[no_mangle]
pub extern "C" fn initramfs_main() {  }

