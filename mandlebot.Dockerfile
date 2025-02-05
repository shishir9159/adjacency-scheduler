#syntax=docker/dockerfile:1.7-labs
FROM rust:bookworm AS builder
LABEL authors="carmack"

WORKDIR /app

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt/,type=cache,sharing=locked \
    set -x && rm -f /etc/apt/apt.conf.d/docker-clean && apt-get update && apt-get install -y \
    bpftool bpfcc-tools ca-certificates curl libbpfcc clang pkg-config linux-headers-6.1.0-28-amd64  && \
    rm -rf /var/lib/apt/lists/*

RUN rustup install stable && \
    rustup toolchain install nightly --component rust-src

# run as root user
RUN cargo install bpf-linker bindgen-cli cargo-generate
# cgroup-skb-egress*/src/main.rs
COPY --parents Cargo.toml Cargo.lock cgroup-skb-egress*/Cargo.* xtask ./

RUN mkdir -p cgroup-skb-egress*/src && main=$"#[panic_handler]\nfn main() {}" && echo "$main" | tee cgroup-skb-egress*/src/main.rs
RUN echo "tcp" > cgroup-skb-egress-common/src/lib.rs

# Build the dependencies without the actual source code to cache dependencies separately
# RUN cargo fetch
RUN cargo build --bin fetch-docker-build
COPY . .

RUN CARGO_PROFILE_RELEASE_DEBUG=true cargo build --target=x86_64-unknown-linux-gnu --release
RUN RUST_LOG=info cargo run --bin xtask --verbose codegen cgroup-skb-egress-ebpf/src/bindings.rs
#CMD ["RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target.\"cfg(all())\".runner=\"sudo -E\"'\""]

FROM debian:bookworm-slim
RUN set -x && apt-get update && apt-get install -y \
    ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/debug/cgroup-skb-egress /app/server
CMD ["/app/server"]