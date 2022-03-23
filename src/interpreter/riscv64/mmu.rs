use modular_bitfield::prelude::*;

use crate::{
    abstract_machine::Readable,
    device::Device,
};

use super::reg::csr::satp::{Satp, SatpMode};


#[bitfield(bits = 32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv32VAddr {
    pub vpn1: B10,
    pub vpn0: B10,
    pub offset: B12,
}
#[bitfield(bits = 34)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv32PAddr{
    pub ppn1: B12,
    pub ppn0: B10,
    pub offset: B12
}

#[bitfield(bits = 24)]
#[derive(BitfieldSpecifier)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv32PPN {
    pub ppn1: B12,
    pub ppn0: B12,
}

#[bitfield(bits = 34)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv32PageTableEntry {
    pub ppn: B24,
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


#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv39VAddr {
    #[skip] __: B25,
    pub vpn2: B9,
    pub vpn1: B9,
    pub vpn0: B9,
    pub offset: B12,
}

#[bitfield(bits = 44)]
// #[derive(BitfieldSpecifier)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv39PPN {
    pub ppn2: B26,
    pub ppn1: B9,
    pub ppn0: B9,
}

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv39PAddr {
    #[skip] __: B8,
    pub ppn: B44,
    pub offset: B12
}

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv39PageTableEntry {
    pub n: B1,
    pub pbmt: B2,
    #[skip] __: B7,
    pub ppn: B44,
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


#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv48VAddr {
    #[skip] __: B16,
    pub vpn3: B9,
    pub vpn2: B9,
    pub vpn1: B9,
    pub vpn0: B9,
    pub offset: B12
}

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv48PAddr {
    #[skip] __: B8,
    // pub ppn3: B17,
    // pub ppn2: B9,
    // pub ppn1: B9,
    // pub ppn0: B9,
    pub ppn: B44,
    pub offset: B12
}

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv48PageTableEntry {
    pub n: B1,
    pub pbmt: B2,
    #[skip] __: B7,
    // pub ppn3: B17,
    // pub ppn2: B9,
    // pub ppn1: B9,
    // pub ppn0: B9,
    pub ppn: B44,
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


#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv57VAddr {
    #[skip] __: B7,
    pub vpn4: B9,
    pub vpn3: B9,
    pub vpn2: B9,
    pub vpn1: B9,
    pub vpn0: B9,
    pub offset: B12
}
#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv57PAddr {
    #[skip] __: B8,
    // pub vpn4: B8,
    // pub vpn3: B9,
    // pub vpn2: B9,
    // pub vpn1: B9,
    // pub vpn0: B9,
    pub ppn: B44,
    pub offset: B12
}

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sv57PageTableEntry {
    pub n: B1,
    pub pbmt: B2,
    #[skip] __: B7,
    // pub ppn4: B8,
    // pub ppn3: B9,
    // pub ppn2: B9,
    // pub ppn1: B9,
    // pub ppn0: B9,
    pub ppn: B44,
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

type R = bool;
type W = bool;
type X = bool;


macro_rules! sv39_lookup_page_table {
    ($device:expr, $ptaddr:expr, $vaddr:expr) => {
        let pt = $device.read_u64($ptaddr as usize)?;
        let pt = Sv39PageTableEntry::from_bytes(pt.to_le_bytes());
        let ptaddr = pt.ppn() as u64;
        if pt.r() | pt.w() | pt.x() != 0 {
            let addr = ((ptaddr as u64) << 12) | ($vaddr.offset() as u64);
            return Some((addr, pt.r() != 0, pt.w() != 0, pt.x() != 0));
        }
    };
}

fn mmu_map(satp: Satp, device: &Device, vaddr: u64) -> Option<(u64, R, W, X)> {
    match satp.mode() {
        SatpMode::Bare => Some((vaddr, true, true, true)),
        SatpMode::Sv39 => {
            let vaddr = Sv39VAddr::from_bytes(vaddr.to_le_bytes());

            let ptaddr = satp.root_addr() + (vaddr.vpn2() as u64);
            sv39_lookup_page_table!(device, ptaddr, vaddr);

            let ptaddr = ptaddr + (vaddr.vpn1() as u64);
            sv39_lookup_page_table!(device, ptaddr, vaddr);

            let ptaddr = ptaddr + (vaddr.vpn0() as u64);
            sv39_lookup_page_table!(device, ptaddr, vaddr);

            None
        },
        SatpMode::Sv48 => todo!(),
        // SatpMode::Sv57 => todo!(),
        // SatpMode::Sv64 => todo!(),
        _ => None, // not implmented sv32
    }
}