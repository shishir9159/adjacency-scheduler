#![no_std]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PacketLog {
    pub ipv4_address: u32,
    pub action: i32,
}

pub struct Log {
    pub packet_counts: u64,
    pub packets: PacketLog,
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for PacketLog {}
