FROM rust:bookworm
#AS builder
LABEL authors="carmack"

WORKDIR /app
COPY . .

#RUN #echo deb http://cloudfront.debian.net/debian sid main >> /etc/apt/sources.list)

RUN set -x && apt-get update && apt-get install -y \
    bpftool bpfcc-tools ca-certificates curl libbpfcc libbpfcc-devl ibclang-dev pkg-config linux-headers-6.1.0-28-amd64  && \
    rm -rf /var/lib/apt/lists/*

RUN rustup install stable && \
    rustup toolchain install nightly --component rust-src

RUN cargo install bpf-linker bindgen-cli cargo-generate
RUN cargo build --target=x86_64-unknown-linux-gnu --release
RUN RUST_LOG=info cargo run --bin xtask --verbose codegen cgroup-skb-egress-ebpf/src/bindings.rs
#CMD ["RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target.\"cfg(all())\".runner=\"sudo -E\"'\""]
CMD ["/app/target/x86_64-unknown-linux-gnu/release/cgroup-skb-egress"]