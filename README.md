# adjacency-scheduler

## Prerequisites

1. Install a rust stable toolchain: `rustup install stable`
2. Install a rust nightly toolchain: `rustup install nightly`
3. Install bpf-linker: `cargo install bpf-linker`

## Build & Run

Use `cargo build`, `cargo check`, etc. as normal. Run your program with:

```shell
RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"'
```
