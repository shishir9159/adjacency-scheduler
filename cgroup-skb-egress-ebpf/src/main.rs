#![no_std]
#![no_main]

use aya_ebpf::cty::int64_t;
use aya_ebpf::{
    macros::{cgroup_skb, map},
    maps::{HashMap, PerfEventArray},
    programs::SkBuffContext,
};
use aya_log_ebpf::info;
use cgroup_skb_egress_common::PacketCount;
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

    // let prefix: u8 = 16;
    // let base: u32 = 0x0AF40000; // 10.244.0.0
    let prefix: u8 = 12;
    let base: u32 = 0x0A600000; // service - 10.96.0.0

    let mask: u32 = if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    };

    (ip & mask) == (base & mask)
}

fn try_cgroup_skb_egress(ctx: SkBuffContext) -> Result<i32, i64> {
    let protocol: u32 = unsafe { (*ctx.skb.skb).protocol };
    if protocol != ETH_P_IP {
        return Ok(1);
    }

    let destination = u32::from_be(ctx.load(offset_of!(iphdr, daddr))?);
    // let destination = u32::from_be(ctx.load(offset_of!(iphdr__bindgen_ty_1__bindgen_ty_1, daddr))?);

    let mut counter: u32 = 1;
    if ip_in_cidr(destination) {
        unsafe {
            match BLOCKLIST.get(&destination) {
                Some(packet_counter) => {
                    counter = counter + packet_counter;
                }
                None => {}
            }
        }

        BLOCKLIST.insert(&destination, &counter, 0)?
    } else {
        return Ok(1);
    }

    let source: u32 = u32::from_be((ctx.load(offset_of!(iphdr, saddr))?));
    info!(&ctx, "SOURCE {} DST {}", source, destination);

    let packet_counter = PacketCount {
        ipv4_address: destination,
        counter,
    };

    // TODO:
    //  attach PID to track the origin?
    EVENTS.output(&ctx, &packet_counter, 0);
    Ok(0)
}

const ETH_P_IP: u32 = 8;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
