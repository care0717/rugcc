extern crate rugcc;
use self::rugcc::common::{IR, ND, Node, IRType, error};

use std::collections::HashMap;


fn add(op: IRType, lhs: usize, rhs: usize, code: &mut Vec<IR>) {
    code.push(IR { op, lhs, rhs});
}

fn gen_lval(node: Node, regno: &mut usize, code: &mut Vec<IR>, vars: &mut HashMap<String, usize>, bpoff: &mut usize) -> usize {
    if node.ty != ND::IDENT {
        error("not an lvalue", None);
    }
    let off = if vars.get(&node.val.to_string()).is_none() {
        vars.insert(node.val, *bpoff);
        *bpoff += 8;
        *bpoff-8
    } else {
        *vars.get(&node.val.to_string()).unwrap()
    };
    let r = *regno;
    *regno += 1;
    add(IRType::MOV, r, 0, code);
    add(IRType::ADD_IMN, r, off, code);
    return r;
}

fn gen_expr(node: Node, regno: &mut usize, code: &mut Vec<IR>, vars: &mut HashMap<String, usize>, bpoff: &mut usize) -> usize {
    if node.ty == ND::NUM {
        let r = *regno;
        *regno += 1;
        add(IRType::IMN, r, node.val.parse().unwrap(), code);
        return r
    } else if node.ty == ND::IDENT {
        let r = gen_lval(node, regno, code, vars, bpoff);
        add(IRType::LOAD, r, r, code);
        return r
    } else if node.ty == ND::OPE('=') {
        let rhs = gen_expr(*node.rhs.unwrap(), regno, code, vars, bpoff);
        let lhs = gen_lval(*node.lhs.unwrap(), regno, code, vars, bpoff);
        add(IRType::STORE, lhs, rhs, code);
        add(IRType::KILL, rhs, 0, code);
        return lhs;
    }
    let ope = node.get_ope();
    *regno += 1;
    let lhs = gen_expr(*node.lhs.unwrap(), regno, code, vars, bpoff);
    *regno += lhs;
    let rhs = gen_expr(*node.rhs.unwrap(), regno, code, vars, bpoff);
    add(IRType::Ope(ope), lhs, rhs, code);
    add(IRType::KILL, rhs, 0, code);
    return lhs
}

fn gen_stmt(node: Node, regno: &mut usize, code: &mut Vec<IR>, vars: &mut HashMap<String, usize>, bpoff: &mut usize, label: &mut usize) {
    match node.ty {
        ND::IF => {
            let r = gen_expr(*node.cond.unwrap(), regno, code, vars, bpoff);
            let x = *label;
            *label += 1;
            add(IRType::UNLESS, r, x, code);
            add(IRType::KILL, r, 0, code);
            gen_stmt(*node.then.unwrap(), regno, code, vars, bpoff, label);
            add(IRType::LABEL, x, 0, code);
        },
        ND::RETURN => {
            let r = gen_expr(*node.expr.unwrap(), regno, code, vars, bpoff);
            add(IRType::RETURN, r, 0, code);
            add(IRType::KILL, r, 0, code);
        },
        ND::EXPR_STMT => {
            let r = gen_expr(*node.expr.unwrap(), regno, code, vars, bpoff);
            add(IRType::KILL, r, 0, code);
        },
        ND::COMP_STMT => {
            for n in node.stmts {
                gen_stmt(n, regno, code, vars, bpoff, label);
            }
        },
        _ => error("unknown node: ", Some(&"aa".to_string())),
    }
}

pub fn gen_ir(node: Node) -> Vec<IR> {
    assert!(node.ty==ND::COMP_STMT);
    let mut code= Vec::new();
    let mut regno = 1;
    let mut vars = HashMap::new();
    let mut bpoff = 0;
    let mut label = 0;
    gen_stmt(node, &mut regno, &mut code, &mut vars, &mut bpoff, &mut label);
    code.insert(0, IR{op: IRType::ALLOCA,lhs: 0, rhs: bpoff});
    add(IRType::KILL, 0, 0, &mut code);
    return code
}

