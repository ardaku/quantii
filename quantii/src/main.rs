/*
 * Copyright © 2022 Quantii Foundation
 * mailto:TODO!("add email")
 *
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License or LICENSE.txt for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

#![no_std]
#![no_main]

extern crate novusk;
extern crate novuskinc;
extern crate quantii;

use quantii::setup;

#[path = "qiish/qiish.rs"]
mod qiish;

// Called from novusk
#[no_mangle]
pub extern "C" fn kernel_main() {
    setup();
    qiish::call_qiish(0);
}

#[no_mangle]
pub extern "C" fn initramfs_main() {}
