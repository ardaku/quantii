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
                    "-M", "raspi3", "-kernel", path, "-serial", "null",
                    "-serial", "stdio",
                ])
                .status()
                .expect("failed to execute process");
        }
        bad => panic!("bad path {bad}"),
    }
}
