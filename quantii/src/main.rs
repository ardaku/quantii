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
    setup();
    loop {}
}

#[no_mangle]
pub extern "C" fn initramfs_main() {}
