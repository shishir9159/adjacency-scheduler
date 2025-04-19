#![no_std]
#![no_main]

use aya_ebpf::cty::int64_t;
use aya_ebpf::{
    macros::{cgroup_skb, map},
    maps::{HashMap, PerfEventArray},
    programs::SkBuffContext,
};
use cgroup_skb_egress_common::PacketCount;
// use ipnet::IpNet;
use memoffset::offset_of;

#[allow(clippy::all)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[rustfmt::skip]
mod bindings;

// TODO:
//  need this line to be dynamic
//  via env value about distros
use bindings::iphdr;
// use bindings::iphdr__bindgen_ty_1__bindgen_ty_1;

#[map]
static EVENTS: PerfEventArray<PacketCount> = PerfEventArray::new(0);
// static EVENTS: PerfEventArray<PacketLog> = PerfEventArray::new(0);

#[map]
static BLOCKLIST: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);

#[cgroup_skb]
pub fn cgroup_skb_egress(ctx: SkBuffContext) -> i32 {
    try_cgroup_skb_egress(ctx).unwrap_or(0)
}

fn block_ip(address: u32) -> bool {
    unsafe { BLOCKLIST.get(&address).is_some() }
}

fn ip_in_cidr(ip: u32) -> bool {
    // TODO:
    //  from configmap

    let base: u32 = 0x0af40000;
    let prefix: u8 = 16;

    let mask = if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    };

    // (ip & mask) == (base & mask)
    true
}

fn increment_counter(address: u32) -> bool {
    let mut counter = 1;

    unsafe {
        if BLOCKLIST.get(&address).is_some() {
            counter = counter + BLOCKLIST.get(&address).unwrap_or(&u32::MAX);
        }

        BLOCKLIST.insert(&address, &counter, 0);
    }

    return true;
}

fn try_cgroup_skb_egress(ctx: SkBuffContext) -> Result<i32, i64> {
    // let cidr: IpNet = "10.244.0.0/16".parse().unwrap();

    let protocol = unsafe { (*ctx.skb.skb).protocol };
    if protocol != ETH_P_IP {
        return Ok(1);
    }

    // TODO:
    //  need this line to be dynamic
    //  via env value about distros
    let destination = u32::from_be(ctx.load(offset_of!(iphdr, daddr))?);
    // let destination = u32::from_be(ctx.load(offset_of!(iphdr__bindgen_ty_1__bindgen_ty_1, daddr))?);

    // converted to_be_bytes()
    if ip_in_cidr(destination) {
        increment_counter(destination);
    }

    let action = if block_ip(destination) { 0 } else { 1 };

    // TODO:

    // let log_entry = PacketLog {
    //     ipv4_address: destination,
    //     action,
    // };

    let packet_counter = PacketCount {
        ipv4_address: destination,
        counter: 0,
    };

    // TODO:

    EVENTS.output(&ctx, &packet_counter, 0);
    // EVENTS.output(&ctx, &log_entry, 0);
    Ok(action)
}

const ETH_P_IP: u32 = 8;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
