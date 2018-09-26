extern crate rugcc;
use self::rugcc::common::{IR, ND, Node, IRType, error};

fn add(op: IRType, lhs: usize, rhs: usize, code: &mut Vec<IR>) {
    code.push(IR{op, lhs, rhs});
}

fn gen_expr(node: Node, mut regno: usize, code: &mut Vec<IR>) -> usize {
    if node.ty == ND::NUM {
        let r = regno;
        add(IRType::IMN, regno, node.val.parse().unwrap(), code);
        return r
    }
    let ope = node.get_ope();
    regno += 1;
    let lhs = gen_expr(*node.lhs.unwrap(), regno, code);
    regno += lhs;
    let rhs = gen_expr(*node.rhs.unwrap(), regno, code);
    add(IRType::Ope(ope), lhs, rhs, code);
    add(IRType::KILL, rhs, 0, code);
    return lhs
}

fn gen_stmt(node: Node, regno: usize, code: &mut Vec<IR>) {
    match node.ty {
        ND::RETURN => {
            let r = gen_expr(*node.expr.unwrap(), regno, code);
            add(IRType::RETURN, r, 0, code);
            add(IRType::KILL, r, 0, code);
        },
        ND::EXPR_STMT => {
            let r = gen_expr(*node.expr.unwrap(), regno, code);
            add(IRType::KILL, r, 0, code);
        },
        ND::COMP_STMT => {
            for n in node.stmts {
                gen_stmt(n, regno, code);
            }
        },
        _ => error("unknown node: ", Some(&"aa".to_string())),
    }
}

pub fn gen_ir(node: Node) -> Vec<IR> {
    assert!(node.ty==ND::COMP_STMT);
    let mut code= Vec::new();
    let regno = 0;
    gen_stmt(node, regno, &mut code);
    return code
}

