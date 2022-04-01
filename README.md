# Quantii
[Novusk](https://github.com/NathanMcMillan54/novusk/) based OS running the
[Ardaku](https://github.com/ardaku/ardaku/) engine.

## Project Overview
Quantii is an OS designed for the future.  Powered by WebAssembly and the Rust
programming language, Quantii provides a novel take on OS security.  The main
architecuture targets for Quantii are ARM64 and RISCV64.  Once the project gets
to a completed and successful state, we will add X86\_64 support as well.

### Graphics
Quantii targets desktop, server and mobile platforms by using a minimal and
responsive convergent GUI.  The Quantii GUI makes a trade-off, where it is
not designed to be familiar, but to be as easy to learn and use as possible.
Main points are that it must be self-documenting/discoverable, and accessible
to more people than conventional GUIs, while also looking clean and serving
itself well to power users.

### Security
OS security is broken, and has always been tacked on.  Quantii is designed with
security in mind from the start.  Quantii runs on an app model, similar to a
mobile operating system.  Each app is sandboxed within it's own WebAssembly
task.  This means apps by themselves can't affect any other app.  They can't
even access the same files.  Files have to explicitly shared from one app to
another by the user.

### Performance
WebAssembly has the capability for "near-native" performance.  Due to the
sandboxing guarantees of WebAssembly, Quantii does not require context
switching.  This means Quantii has the potential to be faster than conventional
operating systems.

Additionally, Quantii uses the asynchronous
[Daku](https://github.com/ardaku/daku) API which means syscalls are queued up,
then sent together in a single syscall.  This reduces the number of calls to
functions outside of the sandbox, which should allow a significant performance
improvement for I/O bound applications over conventional operating systems.

### WebAssembly
The WebAssembly runtime used by Quantii can be switched out, as well as even
the kernel due to its modular design.  This should allow Quantii to get support
for more target architectures quickly.

## Porting
See the [Porting Guide](PORTING.md).

## Building
See the [Instructions](BUILDING.md).
