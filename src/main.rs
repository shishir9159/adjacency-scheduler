// src/main.rs
use aya::{Bpf, maps::HashMap, util::online_cpus};
use aya::programs::TracePoint;
use std::convert::TryInto;
use std::net::Ipv4Addr;
use std::time::Duration;
use tokio::signal;
use tokio::time::interval;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bpf = Bpf::load_file("path/to/ebpf_program.o")?;
    let program: &mut TracePoint = bpf.program_mut("tracepoint_net_dev_xmit")?.try_into()?;
    program.load()?;
    program.attach("net", "net_dev_xmit")?;

    let mut map = HashMap::<_, (u32, u32), PodCommStats>::try_from(bpf.map_mut("POD_COMM_STATS")?)?;

    let mut interval = interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        for cpu_id in online_cpus()? {
            for (key, stats) in map.iter(cpu_id) {
                let (src_ip, dst_ip) = key;
                println!(
                    "Pod communication from {} to {}: {} packets, {} bytes",
                    Ipv4Addr::from(src_ip),
                    Ipv4Addr::from(dst_ip),
                    stats.packets,
                    stats.bytes
                );
            }
        }
    }

    signal::ctrl_c().await?;
    Ok(())
}