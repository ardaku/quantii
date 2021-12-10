#![no_std]
#![no_main]

#[macro_use] extern crate libarc_os;
#[macro_use] extern crate novusk;

use libarc_os::arc_setup;

// Called from novusk
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    arc_setup();

    panic!("Arc OS has nothing to run");
}
