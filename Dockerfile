FROM rust:latest
LABEL authors="carmack"

RUN cargo install bpf-linker bindgen-cli cargo-generate
RUN cargo build --target=x86_64-unknown-linux-gnu
cargo build --release --target=x86_64-unknown-linux-musl
RUST_LOG=info cargo run --bin xtask codegen cgroup-skb-egress-ebpf/src/bindings.rs
RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target."cfg(all())".runner="sudo -E"'
RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"'