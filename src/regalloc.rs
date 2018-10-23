extern crate rugcc;
use self::rugcc::common::{IR, error, IRType, IRInfoType, Function};
use REGS;

fn alloc(ir_reg: usize, reg_map: &mut Vec<i32>, used: &mut Vec<bool>) -> usize{
    if reg_map[ir_reg] != -1 {
        let r = reg_map[ir_reg] as usize;
        assert!(used[r]);
        return r;
    }

    for i in 0..REGS.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        reg_map[ir_reg] = i as i32;
        return i;
    }
    error("register exhausted", None);
    return 0
}
fn kill(r: usize, used: &mut Vec<bool>)  {
    assert!(used[r]);
    used[r] = false;
}

fn visit(irs: &mut Vec<IR>, reg_map: &mut Vec<i32>, used: &mut Vec<bool>) {
    for i in 0..irs.len() {
        let ir = irs[i].clone();
        let info = ir.get_irinfo();
        //eprintln!("{:?}", info);
        match info.ty {
            IRInfoType::REG | IRInfoType::REG_IMN | IRInfoType::REG_LABEL => {
                irs[i].lhs = alloc(ir.lhs,  reg_map,  used);
            },
            IRInfoType::REG_REG  => {
                irs[i].lhs = alloc(ir.lhs,  reg_map,  used);
                irs[i].rhs = alloc(ir.rhs,  reg_map,  used);
            },
            IRInfoType::CALL => {
                irs[i].lhs = alloc(ir.lhs,  reg_map,  used);
                for j in 0..ir.args.clone().len() {
                    irs[i].args[j] = alloc(ir.args[j],  reg_map,  used);
                }
            },
            _ => {}
        }
        if ir.op == IRType::KILL {
            kill(reg_map[ir.lhs] as usize,  used);
            irs[i].op = IRType::NOP;
        }

    }
}

pub fn alloc_regs(fns: &mut Vec<Function>) {
    for f in fns {
        let mut reg_map = Vec::new();
        let mut used = Vec::new();
        for _i in 0..1000 {
            reg_map.push(-1);
        }
        for _i in 0..REGS.len() {
            used.push(false);
        }
        visit(&mut f.irs, &mut reg_map, &mut used);
    }
}
