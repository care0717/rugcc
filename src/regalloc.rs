extern crate rugcc;
use self::rugcc::common::{IR, error, IRType};

fn alloc(ir_reg: usize, reg_map: &mut Vec<i32>, regs: &Vec<&str>, used: &mut Vec<bool>) -> usize{
    if reg_map[ir_reg] != -1 {
        let r = reg_map[ir_reg] as usize;
        assert!(used[r]);
        return r;
    }

    for i in 0..regs.len() {
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

pub fn alloc_regs(regs: &mut Vec<&str>, ins: &mut Vec<IR>) {
    let mut reg_map = Vec::new();
    let mut used = Vec::new();
    for _i in 0..1000 {
        reg_map.push(-1);
    }
    for _i in 0..8 {
        used.push(false);
    }
    for i in 0..ins.len() {
        let ir = ins[i].clone();
        match ir.op {
            IRType::IMN | IRType::RETURN | IRType::ALLOCA => {
                ins[i].lhs = alloc(ir.lhs, &mut reg_map, regs, &mut used);
            },
            IRType::MOV | IRType::Ope(_) | IRType::LOAD | IRType::STORE => {
                ins[i].lhs = alloc(ir.lhs, &mut reg_map, regs, &mut used);
                if !ir.has_imm {ins[i].rhs = alloc(ir.rhs, &mut reg_map, regs, &mut used);}
            },
            IRType::KILL => {
                kill(reg_map[ir.lhs] as usize, &mut used);
                ins[i].op = IRType::NOP;
            },
            _ =>  assert!(true),
        }
    }
}
