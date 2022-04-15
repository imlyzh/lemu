use crate::{abstract_machine::{ExceptionProcessable, ExceptionAttr}, memory::Memory, device::MMIODevice, disassembly::riscv::disassembly};

use super::{machine::MachineModel, reg::{csrmap::{MSTATUS, MIE, MIP, MEPC, MTVEC, MTVAL}, csr::{mstatus::{MStatus, MachineMode}, mie_mip::{Mie, Mip}, mtvec::Tvec}}};


#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawTrapType {
    Exception = 0,
    Interrupt = 1,
}

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawInstrrupt {
    SupervisorSoftwareInterrupt = 1,
    MachineSoftwareInterrupt = 3,
    SupervisorTimerInterrupt = 5,
    MachineTimerInterrupt = 7,
    SupervisorExternalInterrupt = 9,
    MachineExternalInterrupt = 11,
}


#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawException {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAddressMisaligned = 6,
    StoreAccessFault = 7,
    EnvironmentCallFromUMode = 8,
    EnvironmentCallFromSMode = 9,
    EnvironmentCallFromMMode = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exception {
    InstructionAccessFault,
    IllegalInstruction,
    LoadAccessFault(u64),
    StoreAccessFault(u64),
    LoadAddressMisaligned(u64),
    StoreAddressMisaligned(u64),
    InstructionPageFault(u64),
    LoadPageFault(u64),
    StorePageFault(u64),
    UserEcall,
    SupervisorEcall,
    MachineEcall,
    Breakpoint,
}

impl ExceptionAttr for Exception {
    fn is_debugger_trap(&self) -> bool {
        self == &Exception::Breakpoint
    }
}

impl Exception {
    #[inline]
    pub fn into_cause_tval(&self) -> (RawException, u64) {
        match self {
            Exception::InstructionAccessFault => (RawException::InstructionAccessFault, 0),
            Exception::IllegalInstruction => (RawException::IllegalInstruction, 0),
            Exception::LoadAccessFault(u) => (RawException::LoadAccessFault, *u),
            Exception::StoreAccessFault(u) => (RawException::StoreAccessFault, *u),
            Exception::UserEcall => (RawException::EnvironmentCallFromUMode, 0),
            Exception::SupervisorEcall => (RawException::EnvironmentCallFromSMode, 0),
            Exception::MachineEcall => (RawException::EnvironmentCallFromMMode, 0),
            Exception::Breakpoint => (RawException::Breakpoint, 0),
            // Exception::LoadAddressMisaligned(_) => todo!(),
            // Exception::StoreAddressMisaligned(_) => todo!(),
            // Exception::InstructionPageFault(_) => todo!(),
            // Exception::LoadPageFault(_) => todo!(),
            // Exception::StorePageFault(_) => todo!(),
            _ => todo!(),
        }
    }
}


impl MachineModel {
    #[inline]
    pub fn exception_request(&self, e: Exception) -> Option<()> {

        // todo: checks
        let mut mstatus = MStatus::from_bytes(self.csr.read(MSTATUS).to_le_bytes());
        let mie = Mie::from_bytes(self.csr.read(MIE).to_le_bytes());
        let mip = Mip::from_bytes(self.csr.read(MIP).to_le_bytes());
        let mie = mie.msie();
        let mip = mip.msie();

        let tvec = Tvec::from_bytes(self.csr.read(MTVEC).to_le_bytes());

        let (cause, tval) = e.into_cause_tval();
        let cause = cause as u64;

        if mstatus.mie() == 1 && mie == 1 && mip == 1 {
            self.csr.store(MEPC, self.pc.read());
            self.pc.store(tvec.get_pc(RawTrapType::Exception, cause));
            mstatus.set_mpie(mstatus.mie());
            mstatus.set_mie(0);
            mstatus.set_mpp(self.mode.get());
            self.csr.store(MTVAL, tval);
            self.csr.store(MSTATUS, u64::from_le_bytes(mstatus.into_bytes()));
        }

        Some(())
    }

    #[inline]
    pub fn mret(&self) {
        let mut mstatus = MStatus::from_bytes(self.csr.read(MSTATUS).to_le_bytes());
        mstatus.set_mie(mstatus.mpie());
        self.mode.set(mstatus.mpp());
        self.csr.store(MSTATUS, u64::from_le_bytes(mstatus.into_bytes()));
        self.pc.store(self.csr.read(MEPC));
    }

    #[inline]
    pub fn check_inst_access(&self, mode: MachineMode) {
        if self.mode.get() < mode {
            self.exception_request(Exception::InstructionAccessFault);
        }
    }

    #[inline]
    pub fn ecall(&self) {
        let e = match self.mode.get() {
            MachineMode::User => Exception::UserEcall,
            MachineMode::Supervisor => Exception::SupervisorEcall,
            MachineMode::Hypervisor => todo!(),
            MachineMode::Machine => Exception::MachineEcall,
        };
        self.exception_request(e);
    }

    #[inline]
    pub fn ebreak(&self) {
        self.exception_request(Exception::Breakpoint);
    }
}

impl ExceptionProcessable<Exception> for MachineModel {
    #[inline]
    fn process_exception(&self, e: Result<(), Exception>) {
        if let Err(e) = e {
            self.exception_request(e);
        }
    }

    fn exception_log(&self, memory: &dyn MMIODevice, e: Result<(), Exception>) -> Result<(), Exception> {
        if let Err(e) = e {
            match e {
                Exception::InstructionAccessFault => eprintln!("[lemu] InstructionAccessFault, pc at {:8x}", self.pc.read()),
                Exception::IllegalInstruction => {
                    let inst = memory.read_u32(self.pc.read() as usize).unwrap();
                    eprintln!("[lemu] IllegalInstruction 0x{:8x} ({:?}), pc at 0x{:8x}", inst, disassembly(inst).map(|x| x.to_string()), self.pc.read());
                },
                Exception::LoadAccessFault(tval) => {
                    let inst = memory.read_u32(self.pc.read() as usize).map(disassembly).flatten().map(|x| x.to_string());
                    eprintln!("[lemu] LoadAccessFault at {:8x} ({:?}), pc at 0x{:8x}", tval, inst, self.pc.read());
                }
                Exception::StoreAccessFault(tval) => eprintln!("[lemu] StoreAccessFault {:8x}, pc at 0x{:8x}", tval, self.pc.read()),
                Exception::LoadAddressMisaligned(tval) => eprintln!("[lemu] LoadAddressMisaligned at {:8x} with tval {:8x}", self.pc.read(), tval),
                Exception::StoreAddressMisaligned(tval) => eprintln!("[lemu] StoreAddressMisaligned at {:8x} with tval {:8x}", self.pc.read(), tval),
                Exception::InstructionPageFault(tval) => eprintln!("[lemu] InstructionPageFault at {:8x} with tval {:8x}", self.pc.read(), tval),
                Exception::LoadPageFault(tval) => eprintln!("[lemu] LoadPageFault at {:8x} with tval {:8x}", self.pc.read(), tval),
                Exception::StorePageFault(tval) => eprintln!("[lemu] StorePageFault at {:8x} with tval {:8x}", self.pc.read(), tval),
                Exception::UserEcall => eprintln!("[lemu] UserEcall at {:8x}", self.pc.read()),
                Exception::SupervisorEcall => eprintln!("[lemu] SupervisorEcall at {:8x}", self.pc.read()),
                Exception::MachineEcall => eprintln!("[lemu] MachineEcall at {:8x}", self.pc.read()),
                Exception::Breakpoint => eprintln!("[lemu] Breakpoint at {:8x}", self.pc.read()),
            }
        }
        e
    }
}