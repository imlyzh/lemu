use super::{machine::MachineModel, reg::{CSRMap::{MSTATUS, MIE, MIP, MEPC, MTVEC}, csr::{mstatus::MStatus, mie_mip::{Mie, Mip}, mtvec::Tvec}}};


#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawInstrruptType {
    Interrupt = 0,
    Exception = 1,
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
  IllegalInstruction,
  LoadAccessFault(u64),
  StoreAccessFault(u64),
//   LoadAddressMisaligned(u64),
//   StoreAddressMisaligned(u64),
//   InstructionPageFault(u64),
//   LoadPageFault(u64),
//   StorePageFault(u64),
  UserEcall,
  SupervisorEcall,
  MachineEcall,
  Breakpoint,
}

impl Exception {
    pub fn into_cause_tval(&self) -> (RawException, u64) {
        match self {
            Exception::IllegalInstruction => (RawException::IllegalInstruction, 0),
            Exception::LoadAccessFault(u) => (RawException::LoadAccessFault, *u),
            Exception::StoreAccessFault(u) => (RawException::StoreAccessFault, *u),
            Exception::UserEcall => (RawException::EnvironmentCallFromUMode, 0),
            Exception::SupervisorEcall => (RawException::EnvironmentCallFromSMode, 0),
            Exception::MachineEcall => (RawException::EnvironmentCallFromMMode, 0),
            Exception::Breakpoint => (RawException::Breakpoint, 0),
        }
    }
}


impl MachineModel {
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
            self.pc.store(tvec.get_pc(cause));
            mstatus.set_mpie(mstatus.mie());
            mstatus.set_mie(0);
            mstatus.set_mpp(self.mode.get());
            self.csr.store(MSTATUS, u64::from_le_bytes(mstatus.into_bytes()));
        }

        Some(())
    }

    pub fn mret(&self) {
        let mut mstatus = MStatus::from_bytes(self.csr.read(MSTATUS).to_le_bytes());
        mstatus.set_mie(mstatus.mpie());
        self.mode.set(mstatus.mpp());
        self.csr.store(MSTATUS, u64::from_le_bytes(mstatus.into_bytes()));
        self.pc.store(self.csr.read(MEPC));
    }

    pub fn invalid_inst(&self) {
        self.exception_request(Exception::IllegalInstruction);
    }
}