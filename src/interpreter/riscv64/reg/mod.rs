pub mod gpr;
pub mod csr;
pub mod pc;

use std::collections::HashMap;

use once_cell::sync::Lazy;

pub type Xlen = u64;

pub type Reg = Xlen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegType {
    Gpr,
    Fpr,
    Csr,
}

unsafe impl Sync for RegType {}
unsafe impl Send for RegType {}

pub static REG_MAP: Lazy<HashMap<&str, (RegType, usize)>> = Lazy::new(|| {
    let mut map = HashMap::new();

    let gpr_def = include_str!("./gpr_def");
    let gpr_def = gpr_def.trim().split('\n').map(|x| {
        let mut r = x.trim().split(' ');
        (
            r.next().unwrap(),
            (RegType::Gpr, r.next().unwrap().parse::<usize>().unwrap()),
        )
    });
    map.extend(gpr_def);

    let csr_def = include_str!("./csr_def");
    let csr_def = csr_def.trim().split('\n').map(|x| {
        let mut r = x.trim().split(' ');
        (
            r.next().unwrap(),
            (RegType::Csr, r.next().unwrap().parse::<usize>().unwrap()),
        )
    });
    map.extend(csr_def);

    /*
    let fpr_def = include_str!("./reg_def/fpr_def");
    let fpr_def = fpr_def.trim().split("\n").map(
        |x| {
            let mut r = x.trim().split(" ");
            (r.next().unwrap(), (RegType::FPR, r.next().unwrap().parse::<usize>().unwrap()))
        }
    );
    map.extend(fpr_def);
    //  */

    map
});

pub mod csrmap {
    pub const CYCLE: usize = 0x0c00;
    pub const CYCLEH: usize = 0x0c80;
    pub const DCSR: usize = 0x07b0;
    pub const DPC: usize = 0x07b1;
    pub const DSCRATCH0: usize = 0x07b2;
    pub const DSCRATCH1: usize = 0x07b3;
    pub const FCSR: usize = 0x0003;
    pub const FFLAGS: usize = 0x0001;
    pub const FRM: usize = 0x0002;
    pub const HCOUNTEREN: usize = 0x0606;
    pub const HEDELEG: usize = 0x0602;
    pub const HGATP: usize = 0x0680;
    pub const HGEIE: usize = 0x0607;
    pub const HGEIP: usize = 0x0e07;
    pub const HIDELEG: usize = 0x0603;
    pub const HIE: usize = 0x0604;
    pub const HIP: usize = 0x0644;
    pub const HPMCOUNTER3: usize = 0x0c03;
    pub const HPMCOUNTER31: usize = 0x0c1f;
    pub const HPMCOUNTER31H: usize = 0x0c9f;
    pub const HPMCOUNTER3H: usize = 0x0c83;
    pub const HPMCOUNTER4: usize = 0x0c04;
    pub const HPMCOUNTER4H: usize = 0x0c84;
    pub const HSTATUS: usize = 0x0600;
    pub const HTIMEDELTA: usize = 0x0605;
    pub const HTIMEDELTAH: usize = 0x0615;
    pub const HTINST: usize = 0x064a;
    pub const HTVAL: usize = 0x0643;
    pub const INSTRET: usize = 0x0c02;
    pub const INSTRETH: usize = 0x0c82;
    pub const MARCHID: usize = 0x0f12;
    pub const MBASE: usize = 0x0380;
    pub const MBOUND: usize = 0x0381;
    pub const MCAUSE: usize = 0x0342;
    pub const MCOUNTEREN: usize = 0x0306;
    pub const MCOUNTINHIBIT: usize = 0x0320;
    pub const MCYCLE: usize = 0x0b00;
    pub const MCYCLEH: usize = 0x0b80;
    pub const MDBASE: usize = 0x0384;
    pub const MDBOUND: usize = 0x0385;
    pub const MEDELEG: usize = 0x0302;
    pub const MEPC: usize = 0x0341;
    pub const MHARTID: usize = 0x0f14;
    pub const MHPMCOUNTER3: usize = 0x0b03;
    pub const MHPMCOUNTER31: usize = 0x0b1f;
    pub const MHPMCOUNTER31H: usize = 0x0b9f;
    pub const MHPMCOUNTER3H: usize = 0x0b83;
    pub const MHPMCOUNTER4: usize = 0x0b04;
    pub const MHPMCOUNTER4H: usize = 0x0b84;
    pub const MHPMEVENT3: usize = 0x0323;
    pub const MHPMEVENT31: usize = 0x033f;
    pub const MHPMEVENT4: usize = 0x0324;
    pub const MIBASE: usize = 0x0382;
    pub const MIBOUND: usize = 0x0383;
    pub const MIDELEG: usize = 0x0303;
    pub const MIE: usize = 0x0304;
    pub const MIMPID: usize = 0x0f13;
    pub const MINSTRET: usize = 0x0b02;
    pub const MINSTRETH: usize = 0x0b82;
    pub const MIP: usize = 0x0344;
    pub const MISA: usize = 0x0301;
    pub const MSCRATCH: usize = 0x0340;
    pub const MSTATUS: usize = 0x0300;
    pub const MSTATUSH: usize = 0x0310;
    pub const MTINST: usize = 0x034a;
    pub const MTVAL: usize = 0x0343;
    pub const MTVAL2: usize = 0x034b;
    pub const MTVEC: usize = 0x0305;
    pub const MVENDORID: usize = 0x0f11;
    pub const PMPADDR0: usize = 0x03b0;
    pub const PMPADDR1: usize = 0x03b1;
    pub const PMPADDR15: usize = 0x03bf;
    pub const PMPCFG0: usize = 0x03a0;
    pub const PMPCFG1: usize = 0x03a1;
    pub const PMPCFG2: usize = 0x03a2;
    pub const PMPCFG3: usize = 0x03a3;
    pub const SATP: usize = 0x0180;
    pub const SCAUSE: usize = 0x0142;
    pub const SCOUNTEREN: usize = 0x0106;
    pub const SEDELEG: usize = 0x0102;
    pub const SEPC: usize = 0x0141;
    pub const SIDELEG: usize = 0x0103;
    pub const SIE: usize = 0x0104;
    pub const SIP: usize = 0x0144;
    pub const SSCRATCH: usize = 0x0140;
    pub const SSTATUS: usize = 0x0100;
    pub const STXAL: usize = 0x0143;
    pub const STVEC: usize = 0x0105;
    pub const TDATA1: usize = 0x07a1;
    pub const TDATA2: usize = 0x07a2;
    pub const TDATA3: usize = 0x07a3;
    pub const TIME: usize = 0x0c01;
    pub const TIMEH: usize = 0x0c81;
    pub const TSELECT: usize = 0x07a0;
    pub const UCAUSE: usize = 0x0042;
    pub const UEPC: usize = 0x0041;
    pub const UIE: usize = 0x0004;
    pub const UIP: usize = 0x0044;
    pub const USCRATCH: usize = 0x0040;
    pub const USTATUS: usize = 0x0000;
    pub const UTVAL: usize = 0x0043;
    pub const UTVEC: usize = 0x0005;
    pub const VSATP: usize = 0x0280;
    pub const VSCAUSE: usize = 0x0242;
    pub const VSEPC: usize = 0x0241;
    pub const VSIE: usize = 0x0204;
    pub const VSIP: usize = 0x0244;
    pub const VSSCRATCH: usize = 0x0240;
    pub const VSSTATUS: usize = 0x0200;
    pub const VSTVAL: usize = 0x0243;
    pub const VSTVEC: usize = 0x0205;
}
