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

#![warn(
clippy::all,
clippy::restriction,
clippy::pedantic,
clippy::nursery,
clippy::cargo,
)]

#![allow(clippy::implicit_return)]
#![allow(clippy::missing_inline_in_public_items)]


extern crate quantii;
extern crate std;

use alloc::{string::{
    String,
    ToString
}, vec, vec::Vec};
use std::{
    collections::HashMap,
    fs,
    io::{
        stdin,
        stdout
    },
    path::Path,
    print,
    println};

// #[path="option_parse.rs"]
// mod option_parse;

pub const QIISHENV: &Path = Path::new("/home/.qiishenv");

static mut CWD: &Path = Path::new("/home");

/// `QuantII SHell`
///
/// sh variant. Kinda like bash or zsh
pub fn call_qiish(_entrance_code: u8) {

    let mut exit: bool = false;

    let env = get_env();



    let computer_name: &String = &match env.get("computername") {
        Ok(_env) => _env,
        Err(..) => "cpu_name_unknown".to_string()
    };

    let username: &String = &match env.get("") {
        Ok(_env) => _env,
        Err(..) => "username_unknown".to_string(),
    };

    let _ = stdout().flush();

    unsafe {
        while !exit {
            // The @0 represents terminal mode.
            // Since right now only 0 is supported,
            // it is stated directly as such.
            print!(computer_name + "#" + username + "@" + "0" + "%");

            let mut line: String;
            stdin().read_line(&mut line);

            let command: (String, String) = line.split_once(" ").unwrap().trim().into();
            let full_command: (String, Vec<String>) = (command.0, command.1.split_whitespace().collect());
            let exit_code: (
                i16, // Exit code itself
                bool // Whether or not the shell should exit
            ) = call_command(full_command, &env);

            if exit_code.0 > 0 {
                println!("\nProgram exited with error code {}", exit_code.0)
            } else if exit_code.0 < -1 {
                println!("\nProgram exited with irregular error code {}", exit_code.0)
            }
            exit = exit_code.1;
        }
    }
}

unsafe fn call_command(command: (String, Vec<String>), _environment: &HashMap<String, String>) -> (i16, bool) {
    match command.0.as_str() {
        "exit" => (0, true),
        "cd" => cd(command),
        "ls" => ls(command),

        _ => {
            println!("Unrecognized command: {}", command.0);
            (-1, false)
        }
    }
}

unsafe fn ls(command: (String, Vec<String>)) -> (i16, bool) {
    if command.1.is_empty() {
        return call_command(("ls".to_string(), vec!["~".to_string()]), environment)
    }

    const PATH: &Path =
        Path::new(
            command.1
                .get(0)
                .unwrap())
            .canonicalize()
            .unwrap()
            .as_path();

    if !PATH.is_dir() {
        return if PATH.is_file() {
            println!("ls: Cannot change directory into a file: {}",
                     PATH.file_name()
                         .unwrap()
                         .to_str()
                         .unwrap());
            (-1, false)
        } else {
            println!("ls: No such file or directory: {}",
                     PATH.file_name()
                         .unwrap()
                         .to_str()
                         .unwrap());
            (-1, false)
        }
    };

    const PATHS: ReadDir = fs::read_dir(PATH).unwrap();

    for path in PATHS {
        print!("{} ", path.unwrap().path().display())
    }
    return (0, false)
}

fn cd(command: (String, Vec<String>)) -> (i16, bool) {
    unsafe {
    free(command.1);
    if command.1.is_empty() {
        unsafe { return call_command(("cd".to_string(), vec!["~".to_string()]), environment) }
    }

    const PATH: &Path =
        Path::new(
            command.1
                .get(0)
                .unwrap())
            .canonicalize()
            .unwrap()
            .as_path();

    if !PATH.is_dir() {
        return if PATH.is_file() {
            println!("cd: Cannot change directory into a file: {}",
                     PATH.file_name()
                         .unwrap()
                         .to_str()
                         .unwrap());
            (-1, false)
        } else {
            println!("cd: No such file or directory: {}",
                     PATH.file_name()
                         .unwrap()
                         .to_str()
                         .unwrap());
            (-1, false)
        }
    }

    CWD = PATH;
    (0, false)
}
}

fn get_env() -> HashMap<String, String> {
    let mut variables: HashMap<String, String> = HashMap::new();

    const QIISHENV_CONTENTS: Vec<&str> =
        fs::read_to_string(QIISHENV)
            .expect("Could not load /home/.qiishenv")
            .lines().collect();

    for line in QIISHENV_CONTENTS.into_iter() {
        let key_val: (String, String) = match line.split_once("=") {
            Ok(sides) => sides,
            Err(..) => continue,
        };
        variables.insert(key_val.0, key_val.1);
    }

    return variables;
}
