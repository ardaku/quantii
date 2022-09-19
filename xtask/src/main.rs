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

use std::{env, fs, process::Command};

fn help() {
    println!("Commands:");
    println!("  --help         Print this message");
    println!("  dist [arch]    Build release artifacts to ./target/dist");
    println!("  qemu <arch>    Build and launch OS in QEMU");
    println!();
    println!("Arch:");
    println!("  riscv          RISC-V 64 (experimental)");
    println!("  arm            AARCH 64");
}

fn build() {
    let status = Command::new("cargo")
        .args(["install", "--path", "runner", "--root", "target"])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

fn dist_riscv() {
    let _ = fs::remove_dir_all("../target/dist/quantii-riscv");
    let _ = fs::create_dir_all("../target/dist/quantii-riscv");

    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--target",
            "riscv32imac-unknown-none-elf",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());

    fs::copy(
        "../target/riscv32imac-unknown-none-elf/release/quantii",
        "../target/dist/quantii-riscv/quantii.img",
    )
    .unwrap();
}

fn dist_ci() {
    // Build for ARM
    let status = Command::new("cargo")
        .args(["build", "--release", "--target", "aarch64-novusk.json"])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
    // Clippy for ARM
    let status = Command::new("cargo")
        .args([
            "clippy",
            "--target",
            "aarch64-novusk.json",
            "--",
            "-D",
            "warnings",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

fn dist_arm() {
    let _ = fs::remove_dir_all("../target/dist/quantii-arm");
    let _ = fs::create_dir_all("../target/dist/quantii-arm");

    let status = Command::new("cargo")
        .args(["build", "--features", "rpi", "--release", "--target", "aarch64-novusk.json"])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
    let status = Command::new("aarch64-linux-gnu-objcopy")
        .args([
            "--strip-all",
            "-O",
            "binary",
            "../target/aarch64-novusk/release/quantii",
            "../target/dist/quantii-arm/kernel8.img",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

fn dist() {
    match env::args().nth(2).as_deref() {
        None => {
            dist_arm();
            dist_riscv();
        }
        Some("riscv") => dist_riscv(),
        Some("arm") => dist_arm(),
        Some("ci") => dist_ci(),
        Some(arg) => panic!("dist: Invalid argument: {}", arg),
    }
}

fn qemu_riscv() {
    let status = Command::new("cargo")
        .args([
            "run",
            "--release",
            "--target",
            "riscv32imac-unknown-none-elf",
        ])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

fn qemu_arm() {
    let status = Command::new("cargo")
        .args(["run", "--features", "rpi", "--release", "--target", "aarch64-novusk.json"])
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

fn qemu() {
    match env::args().nth(2).as_deref() {
        None => panic!("qemu: need arch"),
        Some("riscv") => qemu_riscv(),
        Some("arm") => qemu_arm(),
        Some(arg) => panic!("qemu: Invalid argument: {}", arg),
    }
}

fn main() {
    // Add runner and linker to the path.
    let curdir = env::current_dir().unwrap();
    let curdir = curdir.as_path().to_str().unwrap();
    let path = env::var("PATH").unwrap();
    env::set_var("PATH", format!("{curdir}/target/bin:{}", path));
    // Install runner
    build();
    // Create config
    const CONFIG_DIR: &str = "quantii/.cargo/";
    const CONFIG_FILE: &str = "quantii/.cargo/config.toml";
    fs::create_dir_all(CONFIG_DIR).unwrap();
    if cfg!(target_arch = "aarch64") && cfg!(target_vendor = "apple") {
        fs::copy("config/macos.toml", CONFIG_FILE).unwrap();
    } else {
        fs::copy("config/linux.toml", CONFIG_FILE).unwrap();
    }
    // Go to Quantii
    env::set_current_dir("quantii").unwrap();
    // Options
    match env::args().nth(1).as_deref() {
        None | Some("--help") => help(),
        Some("dist") => dist(),
        Some("qemu") => qemu(),
        Some(arg) => panic!("Invalid xtask argument: {}", arg),
    }
}
