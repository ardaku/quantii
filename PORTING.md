# Porting
Porting a program to Quantii can be fairly involved.  If it's a command line
program, it can run on Quantii by being ported to LLVM's wasm32-wasi target.
For GUI programs, your GUI toolkit must either support the Daku API or you can
opt to re-write the graphical portion using the provided API.

Quantii doesn't currently support threading in WASM due to limitations of the
Rust compiler (not being able to spawn a thread).  This means each thread must
become a Daku asynchronous task.  Tasks can only communicate over Daku channels,
so mutexes and non-locking data structures must be replaced.
