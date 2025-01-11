FROM rust:bookworm
#AS builder
LABEL authors="carmack"

WORKDIR /app
COPY . .

#RUN #echo deb http://cloudfront.debian.net/debian sid main >> /etc/apt/sources.list)

RUN set -x && apt-get update && apt-get install -y \
    bpftool ca-certificates curl pkg-config bpfcc-tools libclang-dev libbpfcc libbpfcc-dev linux-headers-6.1.0-28-amd64  && \
    rm -rf /var/lib/apt/lists/*

RUN rustup install stable && \
    rustup toolchain install nightly --component rust-src

RUN cargo install bpf-linker bindgen-cli cargo-generate
RUN cargo build --target=x86_64-unknown-linux-gnu --release
RUN RUST_LOG=info cargo run --bin xtask --verbose codegen cgroup-skb-egress-ebpf/src/bindings.rs
RUN #RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target."cfg(all())".runner="sudo -E"'"

CMD ["RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target.\"cfg(all())\".runner=\"sudo -E\"'\""]
#RUN mv target/release/cgroup-skb-egress .
#CMD ["/app/cgroup-skb-egress"]