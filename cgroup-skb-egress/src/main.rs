use std::net::Ipv4Addr;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ApiResource, NotUsed, Object, ResourceExt},
    Client,
};
use serde::Deserialize;
use tracing::*;

use aya::{
    maps::{perf::AsyncPerfEventArray, HashMap},
    programs::{CgroupAttachMode, CgroupSkb, CgroupSkbAttachType},
    util::online_cpus,
};
use bytes::BytesMut;
use clap::Parser;
use log::info;
use tokio::{signal, task};

use cgroup_skb_egress_common::PacketLog;

#[derive(Debug, Parser)]
struct Opt {
//    #[clap(short, long, default_value = "/sys/fs/cgroup/unified")]
// targeting the cgroupv2 containing all kubernetes pod
//     todo: include memory QOS for next k8s update
   #[clap(short, long, default_value = "/sys/fs/cgroup/kubepods.slice/kubepods-burstable.slice/")]
    cgroup_path: String,
}

#[tokio::main]
async fn listing() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await?;

    // Here we replace heavy type k8s_openapi::api::core::v1::PodSpec with
    #[derive(Clone, Deserialize, Debug)]
    struct PodSpecSimple {
        containers: Vec<ContainerSimple>,
    }
    #[derive(Clone, Deserialize, Debug)]
    struct ContainerSimple {
        #[allow(dead_code)]
        image: String,
    }
    type PodSimple = Object<PodSpecSimple, NotUsed>;

    // Here we simply steal the type info from k8s_openapi, but we could create this from scratch.
    let ar = ApiResource::erase::<k8s_openapi::api::core::v1::Pod>(&());

    let pods: Api<PodSimple> = Api::default_namespaced_with(client, &ar);
    for p in pods.list(&Default::default()).await? {
        info!("Pod {} runs: {:?}", p.name_any(), p.spec.containers);
    }

    Ok(())
}

fn containersID() -> anyhow::Result<()> {

    let pods: Api<Pod> = Api::default_namespaced(client);

    let pod = pods.get("blog").await?;
    println!("Got pod: {pod:?}");

    let patch = json!({"spec": {
    "activeDeadlineSeconds": 5
}});
    let pp = PatchParams::apply("kube");
    let patched = pods.patch("blog", &pp, &Patch::Apply(patch)).await?;
    assert_eq!(patched.spec.active_deadline_seconds, Some(5));

    pods.delete("blog", &DeleteParams::default()).await?;

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
    let program: &mut CgroupSkb =
        bpf.program_mut("cgroup_skb_egress").unwrap().try_into()?;
    let cgroup = std::fs::File::open(opt.cgroup_path)?;
    // (1)
    program.load()?;
    // (2)
    program.attach(
        cgroup,
        CgroupSkbAttachType::Egress,
        CgroupAttachMode::Single,
    )?;

    let mut blocklist: HashMap<_, u32, u32> =
        HashMap::try_from(bpf.map_mut("BLOCKLIST").unwrap())?;

    let block_addr: u32 = Ipv4Addr::new(1, 1, 1, 1).into();

    // (3)
    blocklist.insert(block_addr, 0, 0)?;

    let mut perf_array =
        AsyncPerfEventArray::try_from(bpf.take_map("EVENTS").unwrap())?;

    for cpu_id in online_cpus().map_err(|(_, error)| error)? {
        let mut buf = perf_array.open(cpu_id, None)?;

        task::spawn(async move {
            let mut buffers = (0..10)
                .map(|_| BytesMut::with_capacity(1024))
                .collect::<Vec<_>>();

            loop {
                let events = buf.read_events(&mut buffers).await.unwrap();
                for buf in buffers.iter_mut().take(events.read) {
                    let ptr = buf.as_ptr() as *const PacketLog;
                    let data = unsafe { ptr.read_unaligned() };
                    let src_addr = Ipv4Addr::from(data.ipv4_address);
                    info!("LOG: DST {}, ACTION {}", src_addr, data.action);
                }
            }
        });
    }

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
