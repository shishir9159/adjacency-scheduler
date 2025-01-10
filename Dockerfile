FROM ubuntu:latest
#AS builder
LABEL authors="carmack"

WORKDIR /app
COPY . .

#RUN echo deb http://cloudfront.debian.net/debian sid main >> /etc/apt/sources.list
RUN set -x && apt-get update && apt-get install -y \
    build-essential ca-certificates curl pkg-config bpfcc-tools linux-headers-$(uname -r)  && \
    rm -rf /var/lib/apt/lists/*
#    apt install linux-tools-5.8.0-63-generic

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
#RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="$PATH:~/.cargo/bin"

#RUN source "$HOME/.cargo/env"
RUN . "$HOME/.cargo/env"
RUN rustup install stable && \
    rustup toolchain install nightly --component rust-src

#RUN #export PATH=/usr/lib/linux-tools/5.8.0-63-generic:$PATH
#RUN #rustup target add wasm32-unknown-unknown

RUN cargo install bpf-linker bindgen-cli cargo-generate
RUN cargo build --target=x86_64-unknown-linux-gnu
#RUN cargo build --target=x86_64-unknown-linux-gnu --release
RUN file="$(ls -la .)" && echo $file
RUN RUST_LOG=info cargo run --bin xtask --verbose codegen cgroup-skb-egress-ebpf/src/bindings.rs
CMD ["RUN RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target."cfg(all())".runner="sudo -E"'"]
#CMD ["RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"'"]