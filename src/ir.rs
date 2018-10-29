extern crate rugcc;
use self::rugcc::common::{IR, ND, Node, IRType, error, Function};
use std::collections::HashMap;


fn add(op: IRType, lhs: usize, rhs: usize, code: &mut Vec<IR>) {
    code.push(IR { op, lhs, rhs, ..Default::default()});
}

fn gen_lval(node: Node, regno: &mut usize, code: &mut Vec<IR>, vars: &mut HashMap<String, usize>, stack_size: &mut usize) -> usize {
    if node.ty != ND::IDENT {
        error("not an lvalue", None);
    }
    let off = if vars.get(&node.val.to_string()).is_none() {
        *stack_size += 8;
        vars.insert(node.val, *stack_size);
        *stack_size
    } else {
        *vars.get(&node.val.to_string()).unwrap()
    };
    let r = *regno;
    *regno += 1;
    add(IRType::MOV, r, 0, code);
    code.push(IR { op: IRType::SUB_IMN, lhs: r, rhs: off, ..Default::default()});
    return r;
}

fn gen_expr(node: Node, regno: &mut usize, code: &mut Vec<IR>, vars: &mut HashMap<String, usize>, stack_size: &mut usize, label: &mut usize) -> usize {
    match node.ty {
        ND::NUM => {
            let r = *regno;
            *regno += 1;
            add(IRType::IMN, r, node.val.parse().unwrap(), code);
            return r
        },
        ND::IDENT => {
            let r = gen_lval(node, regno, code, vars, stack_size);
            add(IRType::LOAD, r, r, code);
            return r
        },
        ND::LOGAND => {
            let x = *label;
            *label += 1;
            let r1 = gen_expr(*node.lhs.unwrap(), regno, code, vars, stack_size, label);
            add(IRType::UNLESS, r1, x, code);
            let r2 = gen_expr(*node.rhs.unwrap(), regno, code, vars, stack_size, label);
            add(IRType::MOV, r1, r2, code);
            add(IRType::KILL, r2, 0, code);
            add(IRType::UNLESS, r1, x, code);
            add(IRType::IMN, r1, 1, code);
            add(IRType::LABEL, x, 0, code);
            return r1
        },
        ND::LOGOR => {
            let x = *label;
            *label += 1;
            let y = *label;
            *label += 1;

            let r1 = gen_expr(*node.lhs.unwrap(), regno, code, vars, stack_size, label);
            add(IRType::UNLESS, r1, x, code);
            add(IRType::IMN, r1, 1, code);
            add(IRType::JMP, y, 0, code);
            add(IRType::LABEL, x, 0, code);

            let r2 = gen_expr(*node.rhs.unwrap(), regno, code, vars, stack_size, label);
            add(IRType::MOV, r1, r2, code);
            add(IRType::KILL, r2, 0, code);
            add(IRType::UNLESS, r1, y, code);
            add(IRType::IMN, r1, 1, code);
            add(IRType::LABEL, y, 0, code);
            return r1;

        },
        ND::CALL => {
            let mut args = Vec::new();
            for n in node.args {
                args.push(gen_expr(n, regno, code, vars, stack_size, label));
            }
            let r = *regno;
            *regno += 1;

            let ir = IR { op: IRType::CALL, lhs: r, rhs: 0, name: node.val, args, ..Default::default() };
            code.push(ir.clone());
            for i in ir.args {
                add(IRType::KILL, i, 0, code);
            }
            return r
        },
        ND::OPE('=') => {
            let rhs = gen_expr(*node.rhs.unwrap(), regno, code, vars, stack_size, label);
            let lhs = gen_lval(*node.lhs.unwrap(), regno, code, vars, stack_size);
            add(IRType::STORE, lhs, rhs, code);
            add(IRType::KILL, rhs, 0, code);
            return lhs
        },
        _ => {
            let ope = node.get_ope();
            *regno += 1;
            let lhs = gen_expr(*node.lhs.unwrap(), regno, code, vars, stack_size, label);
            *regno += lhs;
            let rhs = gen_expr(*node.rhs.unwrap(), regno, code, vars, stack_size, label);
            add(IRType::Ope(ope), lhs, rhs, code);
            add(IRType::KILL, rhs, 0, code);
            return lhs
        }
    }
}

fn gen_stmt(node: Node, regno: &mut usize, code: &mut Vec<IR>, vars: &mut HashMap<String, usize>, stack_size: &mut usize, label: &mut usize) {
    match node.ty {
        ND::IF => {
            let r = gen_expr(*node.cond.unwrap(), regno, code, vars, stack_size, label);
            let x = *label;
            *label += 1;
            add(IRType::UNLESS, r, x, code);
            add(IRType::KILL, r, 0, code);
            gen_stmt(*node.then.unwrap(), regno, code, vars, stack_size, label);
            if node.els.is_none() {
                add(IRType::LABEL, x, 0, code);
                return;
            }
            let y = *label;
            *label += 1;
            add(IRType::JMP, y, 0, code);
            add(IRType::LABEL, x, 0, code);
            gen_stmt(*node.els.unwrap(), regno, code, vars, stack_size, label);
            add(IRType::LABEL, y, 0, code);
        },
        ND::RETURN => {
            let r = gen_expr(*node.expr.unwrap(), regno, code, vars, stack_size, label);
            add(IRType::RETURN, r, 0, code);
            add(IRType::KILL, r, 0, code);
        },
        ND::EXPR_STMT => {
            let r = gen_expr(*node.expr.unwrap(), regno, code, vars, stack_size, label);
            add(IRType::KILL, r, 0, code);
        },
        ND::COMP_STMT => {
            for n in node.stmts {
                gen_stmt(n, regno, code, vars, stack_size, label);
            }
        },
        _ => error("unknown node: ", Some(&"aa".to_string())),
    }
}
fn gen_args(nodes: Vec<Node>, code: &mut Vec<IR>, vars: &mut HashMap<String, usize>, stack_size: &mut usize) {
    if nodes.len() == 0 {
        return
    }
    add(IRType::SAVE_ARGS, nodes.len(), 0, code);
    for node in nodes {
        if node.ty != ND::IDENT {  error("bad parameter", None); }
        *stack_size += 8;
        vars.insert(node.val, *stack_size);
    }
}

pub fn gen_ir(nodes: Vec<Node>) -> Vec<Function> {
    let mut funcs = Vec::new();
    for node in nodes {
        assert!(node.ty==ND::FUNC);
        let mut code= Vec::new();
        let mut regno = 1;
        let mut vars = HashMap::new();
        let mut label = 0;
        let mut stack_size = 0;
        let name = node.val.clone();
        gen_args(node.args, &mut code, &mut vars, &mut stack_size);
        gen_stmt(*node.body.unwrap(), &mut regno, &mut code, &mut vars, &mut stack_size, &mut label);
        funcs.push(Function{name, irs: code, stack_size, ..Default::default()})
    }
    return funcs
}

#[cfg(test)]
mod tests {
    use super::*;
    # [test]
    fn can_gen_ir() {
        let input = [
            Node {
                ty: ND::FUNC,
                val: "main".to_string(),
                body: Some(Box::new(Node {
                    ty: ND::COMP_STMT,
                    stmts: [
                        Node {
                            ty: ND::RETURN,
                            expr: Some(Box::new(Node {
                                ty: ND::NUM,
                                val: "51".to_string(), ..Default::default()})),
                            ..Default::default()
                        }].to_vec(),
                    ..Default::default() })),
                ..Default::default()
            }
        ];

        let result = gen_ir(input.to_vec());
        let expect = [
            Function {
                name: "main".to_string(),
                irs: [
                    IR { op: IRType::IMN, lhs: 1, rhs: 51, ..Default::default() },
                    IR { op: IRType::RETURN, lhs: 1, rhs: 0, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 1, rhs: 0, ..Default::default() }
                ].to_vec(),
                stack_size: 0
            }
        ];

        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }
}
