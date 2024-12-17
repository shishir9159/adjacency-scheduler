#![no_std]
#![no_main]

use aya_bpf::{
    macros::{map, tracepoint},
    maps::HashMap,
    programs::TracePointContext,
};
use aya_log_ebpf::info;
use core::mem;

#[repr(C)]
struct PodCommStats {
    packets: u64,
    bytes: u64,
}

#[map(name = "POD_COMM_STATS")]
static mut POD_COMM_STATS: HashMap<(u32, u32), PodCommStats> = HashMap::with_max_entries(1024, 0);

#[tracepoint(name = "tracepoint_net_dev_xmit")]
pub fn tracepoint_net_dev_xmit(ctx: TracePointContext) -> u32 {
    match try_tracepoint_net_dev_xmit(ctx) {
        Ok(ret) => ret,
        Err(_) => 1,
    }
}

fn try_tracepoint_net_dev_xmit(ctx: TracePointContext) -> Result<u32, u32> {
    let skb: *const u8 = unsafe { ctx.read_at(0) }?;
    let len: u32 = unsafe { ctx.read_at(mem::size_of::<*const u8>()) }?;

    // Extract source and destination IPs (simplified, assuming IPv4)
    let src_ip = 0; // Replace with actual extraction logic
    let dst_ip = 0; // Replace with actual extraction logic

    let key = (src_ip, dst_ip);
    let stats = POD_COMM_STATS.get_mut(&key).unwrap_or_default();
    stats.packets += 1;
    stats.bytes += len as u64;
    POD_COMM_STATS.insert(&key, &stats, 0);

    info!(&ctx, "Packet from {} to {}: {} bytes", src_ip, dst_ip, len);

    Ok(0)
}