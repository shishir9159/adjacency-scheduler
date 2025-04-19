#![no_std]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PacketLog {
    pub ipv4_address: u32,
    pub action: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PacketCount {
    pub ipv4_address: u32,
    pub counter: u32,
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for PacketLog {}
