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

extern crate neutron_api;
extern crate quantii;

use neutron_api::*;

use quantii::System;

/// `QuantII SHell`
///
/// sh variant. Kinda like bash or zsh
pub fn call_qiish(_entrance_code: u8) {
    static STANDARD_COLOR: neutron_api::ColorCode =
        ColorCode::new(Color::White, Color::Black);

    use ardaku::System;

    // if entrance_code == 0 {
    // qiish_localenv(context)
    // } else if entrance_code == 1 && Hasher::hash(key) == quantii::TERM_HASH.level1.get_next_hash() {}

    System.write(b"computer#username@0 % ");
    let shell = Shell::new();
}

// fn qiish_localenv(context: &[u8]) {
//     ardaku::System.write("Entered level 0 term")
// }
