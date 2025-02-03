FROM rust:bookworm AS builder
#AS builder
LABEL authors="carmack"

WORKDIR /app

RUN set -x && rm -f /etc/apt/apt.conf.d/docker-clean && \
    apt-get update && apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

RUN rustup install stable && \
    rustup toolchain install nightly --component rust-src

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
# Build the dependencies without the actual source code to cache dependencies separately
RUN cargo build --release
COPY . .
RUN CARGO_PROFILE_RELEASE_DEBUG=true cargo build --target=x86_64-unknown-linux-gnu --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/cgroup-skb-egress ./cgroupv2-skb-egress


# run as root user

#RUN RUST_LOG=info cargo run --bin xtask --verbose codegen cgroup-skb-egress-ebpf/src/bindings.rs
#CMD ["RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target.\"cfg(all())\".runner=\"sudo -E\"'\""]
CMD ["./cgroupv2-skb-egress"]