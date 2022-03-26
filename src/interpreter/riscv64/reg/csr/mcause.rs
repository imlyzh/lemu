use modular_bitfield::prelude::*;



#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MCause {
    pub is_interrupt: B1,
    pub exception_code: B63,
}

