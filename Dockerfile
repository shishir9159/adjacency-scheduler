FROM rust:bookworm AS builder
LABEL authors="carmack"

WORKDIR /app
COPY . .

RUN set -x && apt-get update && apt-get install -y \
    bpftool bpfcc-tools libbpfcc clang pkg-config linux-headers-6.1.0-28-amd64  && \
    rm -rf /var/lib/apt/lists/*

RUN rustup install stable && \
    rustup toolchain install nightly --component rust-src

# run as root user

RUN cargo install bpf-linker bindgen-cli cargo-generate
RUN cargo build --target=x86_64-unknown-linux-gnu --profile=release-with-debug
RUN RUST_LOG=info cargo run --bin xtask --verbose codegen cgroup-skb-egress-ebpf/src/bindings.rs
#CMD ["RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target.\"cfg(all())\".runner=\"sudo -E\"'\""]
CMD ["/app/target/x86_64-unknown-linux-gnu/release/cgroup-skb-egress"]

FROM debian:bookworm-slim
RUN set -x && apt-get update && apt-get install -y \
    ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/cgroup-skb-egress /app/server
CMD ["/app/server"]