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

use std::process::Command;

fn main() {
    let path = std::env::args().skip(1).next().unwrap();
    let path = path.as_str();
    match path {
        _path
            if _path
                .ends_with("riscv32imac-unknown-none-elf/release/quantii") =>
        {
            Command::new("qemu-system-riscv32")
                .args([
                    "-nographic",
                    "-machine",
                    "sifive_e",
                    "-m",
                    "128M",
                    "-kernel",
                    path,
                    "-serial",
                    "mon:stdio",
                    "-bios",
                    "none",
                ])
                .status()
                .expect("failed to execute process");
        }
        _path if _path.ends_with("aarch64-novusk/release/quantii") => {
            Command::new("qemu-system-aarch64")
                .args([
                    "-M", "raspi3b", "-kernel", path, "-serial", "null",
                    "-serial", "stdio",
                ])
                .status()
                .expect("failed to execute process");
        }
        bad => panic!("bad path {bad}"),
    }
}
