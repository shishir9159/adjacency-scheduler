# adjacency-scheduler

## Prerequisites

## Build & Run

### Ubuntu
```Bash
apt install -y build-essential pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup install stable
rustup toolchain install nightly --component rust-src
cargo install bpf-linker bindgen-cli cargo-generate
cargo build --target=x86_64-unknown-linux-gnu --release
RUST_LOG=info cargo run --bin xtask codegen cgroup-skb-egress-ebpf/src/bindings.rs
RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target."cfg(all())".runner="sudo -E"'
```

unfortunately, rust binding differs from debian to rhel 8
### RHEL 8 

Unfortunately, RHEL 8 isn't configured to support cgroupv2 and it's crucial for our project to work.

we will need to add the following argument in /etc/default/grub file within GRUB_CMDLINE_LINUX:
```
systemd.unified_cgroup_hierarchy=1
```

then we need to run the following commands and reboot each node:
```shell
#grub2-mkconfig -o /boot/grub2/grub.cfg
grub2-mkconfig -o /boot/efi/EFI/redhat/grub.cfg
```

```shell
yum install -y clang gcc openssl openssl-devel pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup install stable
rustup toolchain install nightly --component rust-src
cargo install bpf-linker bindgen-cli cargo-generate
cargo build --target=x86_64-unknown-linux-gnu --release
RUST_LOG=info cargo run --bin xtask codegen cgroup-skb-egress-ebpf/src/bindings.rs
RUST_LOG=info cargo run --bin cgroup-skb-egress --config 'target."cfg(all())".runner="sudo -E"'
#RUST_LOG=info cargo run --config 'target ."cfg(all())".runner="sudo -E"'
```



```shell
wget https://github.com/fullstorydev/grpcurl/releases/download/v1.9.3/grpcurl_1.9.3_linux_x86_64.tar.gz
tar xzvf grpcurl_*_linux_x86_64.tar.gz
sudo cp grpcurl /usr/bin/
sudo chmod +x /bin/grpcurl
```