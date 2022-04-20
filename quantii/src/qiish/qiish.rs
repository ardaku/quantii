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

extern crate neutron_api;
extern crate quantii;

use neutron_api::*;
use quantii::System;

/// `QuantII SHell`
///
/// sh variant. Kinda like bash or zsh
pub fn call_qiish(_entrance_code: u8) {
    use ardaku::System;

    static STANDARD_COLOR: neutron_api::ColorCode =
        ColorCode::new(Color::White, Color::Black);

    System.write(b"computer#username@0 % ");
    let shell = Shell::new((1280, 720), 424 as Font, STANDARD_COLOR);

    shell.write("computer#username@0 %");
    let command = shell.next_line().split_whit
}

// fn qiish_localenv(context: &[u8]) {
//     ardaku::System.write("Entered level 2 term")
// }
