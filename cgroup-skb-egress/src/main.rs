// mod inspect;
// mod lookuptable;

use aya::{
    maps::{perf::AsyncPerfEventArray, HashMap},
    programs::{CgroupAttachMode, CgroupSkb, CgroupSkbAttachType},
    util::online_cpus,
};
use aya_log::EbpfLogger;
use bytes::BytesMut;
use clap::Parser;
use log::info;
use std::net::Ipv4Addr;
use tokio::{signal, task};

use cgroup_skb_egress_common::PacketCount;

#[derive(Debug, Parser)]
struct Opt {
    //    #[clap(short, long, default_value = "/sys/fs/cgroup/unified")]
    //    targeting the cgroupv2 containing all kubernetes pod
    //     todo: include memory QOS for next k8s update
    #[clap(
        short,
        long,
        default_value = "/sys/fs/cgroup/kubepods.slice/kubepods-burstable.slice/"
    )]
    cgroup_path: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();
    // todo: Ebpf::load_file for runtime updates
    //  to the updated ebpf bytecodes
    let mut bpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/cgroup-skb-egress"
    )))?;
    let program: &mut CgroupSkb = bpf.program_mut("cgroup_skb_egress").unwrap().try_into()?;
    let cgroup = std::fs::File::open(opt.cgroup_path)?;
    program.load()?;
    program.attach(
        cgroup,
        CgroupSkbAttachType::Egress,
        CgroupAttachMode::Single,
    )?;

    let mut blocklist: HashMap<_, u32, u32> = HashMap::try_from(bpf.map_mut("BLOCKLIST").unwrap())?;

    // let block_addr: u32 = Ipv4Addr::new(1, 1, 1, 1).into();
    // blocklist.insert(block_addr, 0, 0)?;

    let mut perf_array = AsyncPerfEventArray::try_from(bpf.take_map("EVENTS").unwrap())?;

    for cpu_id in online_cpus().map_err(|(_, error)| error)? {
        let mut buf = perf_array.open(cpu_id, None)?;

        task::spawn(async move {
            let mut buffers = (0..10)
                .map(|_| BytesMut::with_capacity(1024))
                .collect::<Vec<_>>();

            env_logger::init();
            EbpfLogger::init(&mut bpf).unwrap();
            loop {
                let events = buf.read_events(&mut buffers).await.unwrap();
                for buf in buffers.iter_mut().take(events.read) {
                    let ptr = buf.as_ptr() as *const PacketCount;
                    let data = unsafe { ptr.read_unaligned() };
                    let dst_addr = Ipv4Addr::from(data.ipv4_address);
                    info!(
                        "LOG: DST {}, COUNT {}, ipv4 format {}",
                        dst_addr, data.counter, data.ipv4_address
                    );
                }
            }
        });
    }

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
