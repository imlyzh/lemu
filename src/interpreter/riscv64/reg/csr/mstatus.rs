use modular_bitfield::prelude::*;


#[derive(BitfieldSpecifier)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SUMachineMode {
    User = 0,
    Supervisor = 1,
}

#[derive(BitfieldSpecifier)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MachineMode {
    User = 0,
    Supervisor = 1,
    Hypervisor = 2,
    Machine = 3,
}

#[bitfield(bits = 64)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MStatus {
    sd: B1,
    #[skip] __: B25,
    mbe: B1,
    sbe: B1,
    sxl: B2,
    uxl: B2,
    #[skip] __: B9,
    tsr: B1,
    tw: B1,
    tvm: B1,
    mxr: B1,
    sum: B1,
    mprv: B1,
    xs: B2,
    fs: B2,
    #[bits=2]
    mpp: MachineMode,
    vs: B2,
    #[bits=1]
    spp: SUMachineMode,
    mpie: B1,
    ube: B1,
    spie: B1,
    #[skip] __: B1,
    mie: B1,
    #[skip] __: B1,
    sie: B1,
    #[skip] __: B1,
}
