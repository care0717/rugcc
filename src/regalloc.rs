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
    error("register exhausted".to_string());
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
        // r0 is a reserved register that is always mapped to rbp.
        reg_map[0] = 0;
        used[0] = true;
        visit(&mut f.irs, &mut reg_map, &mut used);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    # [test]
    fn can_alloc() {
        let mut input = [
            Function {
                name: "main".to_string(),
                irs: [
                    IR { op: IRType::IMN, lhs: 1, rhs: 2, ..Default::default()},
                    IR { op: IRType::IMN, lhs: 2, rhs: 2, ..Default::default() },
                    IR { op: IRType::IMN, lhs: 3, rhs: 3, ..Default::default() },
                    IR { op: IRType::Ope('*'), lhs: 2, rhs: 3, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 3, ..Default::default() },
                    IR { op: IRType::Ope('+'), lhs: 1, rhs: 2, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 2, ..Default::default() },
                    IR { op: IRType::IMN, lhs: 4, rhs: 2, ..Default::default() },
                    IR { op: IRType::Ope('/'), lhs: 1, rhs: 4, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 4, ..Default::default() },
                    IR { op: IRType::IMN, lhs: 5, rhs: 1, ..Default::default() },
                    IR { op: IRType::Ope('-'), lhs: 1, rhs: 5, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 5, ..Default::default() },
                    IR { op: IRType::RETURN, lhs: 1, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 1, ..Default::default() }].to_vec(),
                stack_size: 0 }].to_vec();

        alloc_regs(&mut input);

        let expect =  [
            Function {
                name: "main".to_string(),
                irs: [
                    IR { op: IRType::IMN, lhs: 1, rhs: 2, ..Default::default()},
                    IR { op: IRType::IMN, lhs: 2, rhs: 2, ..Default::default() },
                    IR { op: IRType::IMN, lhs: 3, rhs: 3, ..Default::default() },
                    IR { op: IRType::Ope('*'), lhs: 2, rhs: 3, ..Default::default() },
                    IR { op: IRType::NOP, lhs: 3, ..Default::default() },
                    IR { op: IRType::Ope('+'), lhs: 1, rhs: 2, ..Default::default() },
                    IR { op: IRType::NOP, lhs: 2, ..Default::default() },
                    IR { op: IRType::IMN, lhs: 2, rhs: 2, ..Default::default() },
                    IR { op: IRType::Ope('/'), lhs: 1, rhs: 2, ..Default::default() },
                    IR { op: IRType::NOP, lhs: 4, ..Default::default() },
                    IR { op: IRType::IMN, lhs: 2, rhs: 1, ..Default::default() },
                    IR { op: IRType::Ope('-'), lhs: 1, rhs: 2, ..Default::default() },
                    IR { op: IRType::NOP, lhs: 5, ..Default::default() },
                    IR { op: IRType::RETURN, lhs: 1, ..Default::default() },
                    IR { op: IRType::NOP, lhs: 1, ..Default::default() }].to_vec(),
                stack_size: 0 }];

        assert_eq!(input.len(), expect.len());
        for i in 0..input.len() {
            assert_eq!(input[i], expect[i]);
        }
    }
}
