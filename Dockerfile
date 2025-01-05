FROM rust:latest
#AS builder
LABEL authors="carmack"

RUN set -x && apt-get update && apt-get install -y \
    ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo install bpf-linker bindgen-cli cargo-generate
RUN cargo build --target=x86_64-unknown-linux-gnu
RUN RUST_LOG=info cargo run --bin xtask codegen cgroup-skb-egress-ebpf/src/bindings.rs
RUN RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target."cfg(all())".runner="sudo -E"'
CMD ["RUN RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"'"]