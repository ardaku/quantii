# Quantii

[Novusk](https://github.com/NathanMcMillan54/novusk/) based OS running the
[Ardaku](https://github.com/ardaku/ardaku/) engine.

## About

An OS where all userspace programs are compiled to WebAssembly. Rather than
relying on context switching to protect memory, Quantii relies on WebAssembly
sandboxing guarantees. Each userspace program is contained so it can't
interfere with other programs (such as by tampering with their files). Apps
interact with the hardware via either the Daku (for gui, multi-media, semantic
filesystem, async networking, etc.) or WASI (for hierarchical filesystem,
blocking networking, etc.) APIs. Each WebAssembly program will use a
[custom data section](https://www.w3.org/TR/2019/REC-wasm-core-1-20191205/#binary-customsec)
to store metadata about the program. For porting to this OS, crates that work
on Linux and Windows will need to add support using the daku/wasi crates as
opposed to the windows/winapi or libc/nix crates. Luckily a lot of crates
can already work on WASI. Multi-threading is supported via an async task API in
Daku (similar to Dart/Flutter Isolates), since there is no native threading
support in WebAssembly 1.0. Quantii is a modular OS that can be compiled with
different kernels, WebAssembly runtimes, and / or WASI implementations depending
on the requirements for a specific target. Rust programs made for the OS target
either wasm32-wasi or wasm32-unknown-unknown. Each program will have access to
select portals, which are similar to app permissions on Android, and each portal
is a message channel for interacting with hardware.

## Project Overview

Quantii is an OS designed for the future. Powered by WebAssembly and the Rust
programming language, Quantii provides a novel take on OS security. The main
architecture targets for Quantii are ARM64 and RISCV64. Once the project gets
to a completed and successful state, we will add X86\_64 support as well.

### Graphics

Quantii targets desktop, server and mobile platforms by using a minimal and
responsive convergent GUI. The Quantii GUI makes a trade-off, where it is
not designed to be familiar, but to be as easy to learn and use as possible.
Main points are that it must be self-documenting/discoverable, and accessible
to more people than conventional GUIs, while also looking clean and serving
itself well to power users.

### Security

OS security is broken, and has always been tacked on. Quantii is designed with
security in mind from the start. Quantii runs on an app model, similar to a
mobile operating system. Each app is sandboxed within its own WebAssembly
task. This means apps by themselves can't affect any other app. They can't
even access the same files. Files have to explicitly shared from one app to
another by the user.

### Performance

WebAssembly has the capability for "near-native" performance. Due to the
sandboxing guarantees of WebAssembly, Quantii does not require context
switching. This means Quantii has the potential to be faster than conventional
operating systems.

Additionally, Quantii uses the asynchronous
[Daku](https://github.com/ardaku/daku) API which means syscalls are queued up,
then sent together in a single syscall. This reduces the number of calls to
functions outside the sandbox, which should allow a significant performance
improvement for I/O bound applications over conventional operating systems.

### WebAssembly

The WebAssembly runtime used by Quantii can be switched out, as well as even
the kernel due to its modular design. This should allow Quantii to get support
for more target architectures quickly.
