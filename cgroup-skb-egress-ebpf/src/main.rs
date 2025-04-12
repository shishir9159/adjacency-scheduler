#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{cgroup_skb, map},
    maps::{HashMap, PerfEventArray},
    programs::SkBuffContext,
};
use aya_ebpf::cty::int64_t;
use memoffset::offset_of;
use ipnet::IpNet;
use cgroup_skb_egress_common::PacketLog;

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
static EVENTS: PerfEventArray<PacketLog> = PerfEventArray::new(0);

#[map]
static BLOCKLIST: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);

#[cgroup_skb]
pub fn cgroup_skb_egress(ctx: SkBuffContext) -> i32 {
    try_cgroup_skb_egress(ctx).unwrap_or(0)
}

fn block_ip(address: u32) -> bool {
    unsafe { BLOCKLIST.get(&address).is_some() }
}

fn increment_counter(address: &u32, counter: &u32) -> bool {

    let mut counter = 1;

    unsafe {
        if BLOCKLIST.get(&address).is_some() {
            counter = counter + BLOCKLIST.get(&address).unwrap_or(&u32::MAX);
        }

        BLOCKLIST.insert(address, &counter, 0)
    }
    return true
}

fn try_cgroup_skb_egress(ctx: SkBuffContext) -> Result<i32, i64> {

    let cidr:  IpNet = "10.244.0.0/16".parse().unwrap();

    let protocol = unsafe { (*ctx.skb.skb).protocol };
    if protocol != ETH_P_IP {
        return Ok(1);
    }

    // TODO:
    //  need this line to be dynamic
    //  via env value about distros
    let destination = u32::from_be(ctx.load(offset_of!(iphdr, daddr))?);
    // let destination = u32::from_be(ctx.load(offset_of!(iphdr__bindgen_ty_1__bindgen_ty_1, daddr))?);

    if cidr.contains(&destination) {
        increment_counter(&destination, &0);
    }

    let action = if block_ip(destination) { 0 } else { 1 };

    let log_entry = PacketLog {
        ipv4_address: destination,
        action,
    };
    EVENTS.output(&ctx, &log_entry, 0);
    Ok(action)
}

const ETH_P_IP: u32 = 8;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
