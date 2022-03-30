pub mod sdb;
pub mod parser;


use std::{
    process::exit,
    collections::VecDeque,
    // convert::identity
};

use crate::{
    abstract_machine::{RegInfo, Readable, Execable, ExceptionProcessable},
    memory::Memory, device::MMIODevice
};

use self::sdb::{SDB, Expr};


impl SDB {
    pub fn eval_sdb<E>(&self, breakpoint_list: &mut VecDeque<()>, machine: impl RegInfo + Execable<E> + ExceptionProcessable<E>, memory: &dyn MMIODevice) {
        // machine.get_reg_value(i)
        match self {
            SDB::H => todo!(),
            SDB::C => {
                machine.logged_process_exception(memory, machine.exec_once(memory));
            },
            SDB::Q => exit(0),
            SDB::Si(num) => {
                machine.logged_process_exception(memory, machine.setp_num(memory, *num));
            },
            SDB::Info(_) => todo!(),
            SDB::X(_, _) => todo!(),
            SDB::P(expr) => {
                let r = expr.eval(&machine, memory);
                println!("{:?}", r);
            },
            SDB::W(_) => todo!(),
            SDB::D(offset) => {
                breakpoint_list.remove(*offset);
            }
        }
    }
}

impl Expr {
    pub fn eval(&self, machine: &impl RegInfo, memory: &dyn MMIODevice) -> Option<u64> {
        // machine.get_reg_value(i)
        let r = match self {
            Expr::Reg(r) => return machine.get_reg_value(r),
            Expr::Num(num) => *num,
            Expr::Deref(addr) => return memory.read_u64(addr.eval(machine, memory)? as usize),
            Expr::Leq   (e1, e2) =>
                (e1.eval(machine, memory)? <= e2.eval(machine, memory)?) as u64,
            Expr::Lt    (e1, e2) =>
                (e1.eval(machine, memory)? < e2.eval(machine, memory)?) as u64,
            Expr::Geq   (e1, e2) =>
                (e1.eval(machine, memory)? >= e2.eval(machine, memory)?) as u64,
            Expr::Gt    (e1, e2) =>
                (e1.eval(machine, memory)? > e2.eval(machine, memory)?) as u64,
            Expr::Eq    (e1, e2) =>
                (e1.eval(machine, memory)? == e2.eval(machine, memory)?) as u64,
            Expr::Ne    (e1, e2) =>
                (e1.eval(machine, memory)? != e2.eval(machine, memory)?) as u64,
            Expr::And   (e1, e2) => e1.eval(machine, memory)? & e2.eval(machine, memory)?,
                Expr::Or    (e1, e2) => e1.eval(machine, memory)? | e2.eval(machine, memory)?,
            Expr::Add   (e1, e2) => e1.eval(machine, memory)? + e2.eval(machine, memory)?,
            Expr::Sub   (e1, e2) => e1.eval(machine, memory)? - e2.eval(machine, memory)?,
            Expr::Mul   (e1, e2) => e1.eval(machine, memory)? * e2.eval(machine, memory)?,
            Expr::Div   (e1, e2) => e1.eval(machine, memory)? / e2.eval(machine, memory)?,
            Expr::Mod   (e1, e2) => e1.eval(machine, memory)? % e2.eval(machine, memory)?,
            Expr::Pow   (e1, e2) => e1.eval(machine, memory)?.pow(e2.eval(machine, memory)? as u32),
        };
        Some(r)
    }
}