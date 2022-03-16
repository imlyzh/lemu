use std::{collections::HashMap, io::BufRead};

use once_cell::unsync::Lazy;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegType {
    GPR,
    FPR,
    CSR,
}

pub const REG_MAP: Lazy<HashMap<&str, (RegType, usize)>> = Lazy::new(|| {
    let mut map = HashMap::new();

    let gpr_def = include_str!("./reg_def/gpr_def");
    let gpr_def = gpr_def.trim().split("\n").map(
        |x| {
            let mut r = x.trim().split(" ");
            (r.next().unwrap(), (RegType::GPR, r.next().unwrap().parse::<usize>().unwrap()))
        }
    );
    map.extend(gpr_def);

    let csr_def = include_str!("./reg_def/csr_def");
    let csr_def = csr_def.trim().split("\n").map(
        |x| {
            let mut r = x.trim().split(" ");
            (r.next().unwrap(), (RegType::CSR, r.next().unwrap().parse::<usize>().unwrap()))
        }
    );
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
