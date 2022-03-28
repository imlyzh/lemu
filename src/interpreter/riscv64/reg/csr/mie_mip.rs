use modular_bitfield::prelude::*;



#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mie {
    #[skip] __: B48,
    #[skip] __: B4,
    pub meie: B1,
    #[skip] __: B1,
    pub seie: B1,
    #[skip] __: B1,
    pub mtie: B1,
    #[skip] __: B1,
    pub stie: B1,
    #[skip] __: B1,
    pub msie: B1,
    #[skip] __: B1,
    pub ssie: B1,
    #[skip] __: B1,
}


pub type Mip = Mie;