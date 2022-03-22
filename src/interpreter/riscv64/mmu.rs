use modular_bitfield::prelude::*;

use crate::device::Device;

use super::reg::csr::satp::{Satp, SatpModeValue};


#[bitfield(bits = 32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv32VAddr(pub B10, pub B10, pub B12);
#[bitfield(bits = 34)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv32PAddr(pub B12, pub B10, pub B12);

#[bitfield(bits = 34)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv32PageTable {
    pub ppn1: B12,
    pub ppn0: B12,
    pub rsw: B2,
    pub d: B1,
    pub a: B1,
    pub g: B1,
    pub u: B1,
    pub x: B1,
    pub w: B1,
    pub r: B1,
    pub v: B1,
}


#[bitfield(bits = 39)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv39VAddr(pub B9, pub B9, pub B9, pub B12);

#[bitfield(bits = 56)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv39PAddr(pub B26, pub B9, pub B9, pub B12);

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv39PageTable {
    pub n: B1,
    pub pbmt: B2,
    pub _reserved: B7,
    pub ppn2: B26,
    pub ppn1: B9,
    pub ppn0: B9,
    pub rsw: B2,
    pub d: B1,
    pub a: B1,
    pub g: B1,
    pub u: B1,
    pub x: B1,
    pub w: B1,
    pub r: B1,
    pub v: B1,
}


#[bitfield(bits = 48)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv48VAddr(pub B9, pub B9, pub B9, pub B9, pub B12);

#[bitfield(bits = 56)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv48PAddr(pub B17, pub B9, pub B9, pub B9, pub B12);

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv48PageTable {
    pub n: B1,
    pub pbmt: B2,
    pub _reserved: B7,
    pub ppn3: B17,
    pub ppn2: B9,
    pub ppn1: B9,
    pub ppn0: B9,
    pub rsw: B2,
    pub d: B1,
    pub a: B1,
    pub g: B1,
    pub u: B1,
    pub x: B1,
    pub w: B1,
    pub r: B1,
    pub v: B1,
}


#[bitfield(bits = 57)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv57VAddr(pub B9, pub B9, pub B9, pub B9, pub B9, pub B12);
#[bitfield(bits = 56)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv57PAddr(pub B8, pub B9, pub B9, pub B9, pub B9, pub B12);

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv57PageTable {
    pub n: B1,
    pub pbmt: B2,
    pub _reserved: B7,
    pub ppn4: B8,
    pub ppn3: B9,
    pub ppn2: B9,
    pub ppn1: B9,
    pub ppn0: B9,
    pub rsw: B2,
    pub d: B1,
    pub a: B1,
    pub g: B1,
    pub u: B1,
    pub x: B1,
    pub w: B1,
    pub r: B1,
    pub v: B1,
}


fn mmu_map(satp: Satp, device: &Device, vaddr: u64) -> Option<u64> {
    let r = match satp.mode() {
        SatpModeValue::BARE => vaddr,
        SatpModeValue::SV39 => todo!(),
        SatpModeValue::SV48 => todo!(),
        SatpModeValue::SV57 => todo!(),
        SatpModeValue::SV64 => todo!(),
        _ => return None,
    };
    Some(r)
}