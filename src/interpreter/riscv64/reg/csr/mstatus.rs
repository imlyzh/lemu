use modular_bitfield::prelude::*;


#[derive(BitfieldSpecifier)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SUMachineMode {
    User = 0,
    Supervisor = 1,
}

#[repr(u8)]
#[derive(BitfieldSpecifier)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MachineMode {
    User = 0,
    Supervisor = 1,
    Hypervisor = 2,
    Machine = 3,
}

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MStatus {
    pub sd: B1,
    #[skip] __: B25,
    pub mbe: B1,
    pub sbe: B1,
    pub sxl: B2,
    pub uxl: B2,
    #[skip] __: B9,
    pub tsr: B1,
    pub tw: B1,
    pub tvm: B1,
    pub mxr: B1,
    pub sum: B1,
    pub mprv: B1,
    pub xs: B2,
    pub fs: B2,
    #[bits=2]
    pub mpp: MachineMode,
    pub vs: B2,
    #[bits=1]
    pub spp: SUMachineMode,
    pub mpie: B1,
    pub ube: B1,
    pub spie: B1,
    #[skip] __: B1,
    pub mie: B1,
    #[skip] __: B1,
    pub sie: B1,
    #[skip] __: B1,
}
