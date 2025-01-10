# adjacency-scheduler

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
Could not find openssl via pkg-config:
  Could not run `PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags openssl`
  The pkg-config command could not be found.

  Most likely, you need to install a pkg-config package for your OS.
  Try `apt install pkg-config`, or `yum install pkg-config`,
  or `pkg install pkg-config`, or `apk add pkgconfig` depending on your distribution.
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

sudo iptables -t nat -A PREROUTING -d 10.0.0.7 -p tcp --dport 6443 -j DNAT --to-destination 10.0.0.9:6443
sudo iptables -t nat -A POSTROUTING -d 10.0.0.9 -p tcp --dport 6443 -j MASQUERADE

```shell
wget https://github.com/fullstorydev/grpcurl/releases/download/v1.9.3/grpcurl_1.9.3_linux_x86_64.tar.gz
tar xzvf grpcurl_*_linux_x86_64.tar.gz
sudo cp grpcurl /usr/bin/
sudo chmod +x /bin/grpcurl
```