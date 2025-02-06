# adjacency-scheduler

## Assumptions
not more than four containers per pod

[//]: # (maybe a max config value wouldn't be so bad)

## Prerequisites

## Build & Run

```Bash
apt install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup install stable
rustup toolchain install nightly --component rust-src
cargo install bpf-linker
cargo install bindgen-cli
cargo install cargo-generate
apt install -y pkg-config
cargo build --target=x86_64-unknown-linux-gnu --release
RUST_LOG=info cargo run --bin xtask codegen cgroup-skb-egress-ebpf/src/bindings.rs
RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target."cfg(all())".runner="sudo -E"'
RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"'
```

```shell
cargo install bpf-linker
cargo install bindgen-cli
cargo install cargo-generate
cargo build --target=x86_64-unknown-linux-gnu
cargo build --release --target=x86_64-unknown-linux-musl
RUST_LOG=info cargo run --bin xtask codegen cgroup-skb-egress-ebpf/src/bindings.rs
RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target."cfg(all())".runner="sudo -E"'
RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"'
```



```shell
wget https://github.com/fullstorydev/grpcurl/releases/download/v1.9.3/grpcurl_1.9.3_linux_x86_64.tar.gz
tar xzvf grpcurl_*_linux_x86_64.tar.gz
sudo cp grpcurl /usr/bin/
sudo chmod +x /bin/grpcurl
```