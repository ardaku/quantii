/**
Copyright (c) 2022 Quantii Foundation

This file is part of Quantii.

Quantii is free software: you can redistribute
it and/or modify it under the terms of the GNU
Lesser General Public License as published by
the Free Software Foundation, either version 3
of the License, or (at your option) any later
version.

Quantii is distributed in the hope that it
will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR
PURPOSE. See the GNU Lesser General Public
License for more details.

You should have received a copy of the GNU
Lesser General Public License along with
Quantii. If not, see <https://www.gnu.org/licenses/>.
*/

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
