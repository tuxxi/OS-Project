# OS Project

Project for CIS31: *Operating System Concepts* at De Anza College, written in Rust.

Simulates an OS by reading an input file containing the CPU and IO operations each process will do,
and allocates / swaps / schedules the processes just like a real OS would. 

Requires the Rust 2018 compiler, `rustc >=1.31`. 
It is recommended to install the Rust toolchain using [rustup](https://rustup.rs/).

To compile and run:
```bash
cd OS-Project 
cargo build --release
cargo run
```