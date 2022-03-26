use modular_bitfield::prelude::*;



#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mie {
    #[skip] __: B48,
    #[skip] __: B4,
    meie: B1,
    #[skip] __: B1,
    seie: B1,
    #[skip] __: B1,
    mtie: B1,
    #[skip] __: B1,
    stie: B1,
    #[skip] __: B1,
    msie: B1,
    #[skip] __: B1,
    ssie: B1,
    #[skip] __: B1,
}
