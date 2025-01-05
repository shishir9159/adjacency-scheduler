# adjacency-scheduler

## Prerequisites

## Build & Run

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