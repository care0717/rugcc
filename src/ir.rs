extern crate rugcc;
use self::rugcc::common::{IR, TY, Node, IRType};

fn new_ir(op: IRType, lhs: usize, rhs: usize) -> IR {
    return IR{op, lhs, rhs};
}

fn gen_ir_sub(node: Node, mut regs: &mut Vec<&str>, mut ins: &mut Vec<IR>, mut regno: usize) -> usize {
    if node.ty == TY::Num{
        let r = regno;
        ins.push(new_ir(IRType::IMN, regno, node.val.parse().unwrap()));
        return r
    }
    let ope = node.get_ope();
    regno += 1;
    let lhs = gen_ir_sub(*node.lhs.unwrap(), &mut regs, &mut ins, regno);
    regno += lhs;
    let rhs = gen_ir_sub(*node.rhs.unwrap(), &mut regs, &mut ins, regno);
    ins.push(new_ir(IRType::Ope(ope), lhs, rhs));
    ins.push(new_ir(IRType::KILL, rhs, 0));
    return lhs
}

pub fn gen_ir(node: Node, regs: &mut Vec<&str>, ins: &mut Vec<IR>, regno: usize) -> usize{
    let r = gen_ir_sub(node, regs, ins, regno);
    ins.push(new_ir(IRType::RETURN, r, 0));
    return 0
}