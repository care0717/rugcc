extern crate rugcc;
use self::rugcc::common::{TY, Token, error};

#[derive(Debug)]
pub struct Node {
    pub ty: TY,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: String,
}

impl Node {
    fn get_ope(&self) -> char {
        match self.ty {
            TY::Ope(c) => return c,
            _ => {
                assert!(true);
                return 'a'
            },
        }
    }
}

#[derive(Clone, Debug)]
enum IRType {
    IMN,
    MOV,
    RETURN,
    KILL,
    NOP,
    Ope(char),
}

#[derive(Clone, Debug)]
pub struct IR {
    op: IRType,
    lhs: usize,
    rhs: usize,
}


fn new_node(op: char, lhs: Node, rhs: Node) -> Node {
    return Node{ty: TY::Ope(op), lhs: Some(Box::new(lhs)), rhs: Some(Box::new(rhs)), val: op.to_string()};
}

fn new_node_num(val: String) -> Node {
    return Node{ty: TY::Num, lhs: None, rhs: None, val};
}

fn number(tokens: &mut Vec<Token>) -> Node {
    let token = tokens.pop().unwrap();
    if token.ty != TY::Num { error("number expected, but got ", Some(&token.val))}
    return new_node_num(token.val);
}

fn new_ir(op: IRType, lhs: usize, rhs: usize) -> IR {
    return IR{op, lhs, rhs};
}

pub fn expr(mut tokens: &mut Vec<Token>) -> Node {
    let mut lhs = number(tokens);

    loop {
        let token = tokens.pop();
        match token {
            Some(t) => {
                let op = t.ty;
                match op {
                    TY::Ope(o) => lhs = new_node(o, lhs, number(&mut tokens)),
                    _ => break,
                }
            },
            None => break,
        }
    }

    return lhs;
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
            IRType::IMN | IRType::RETURN => {
                ins[i].lhs = alloc(ir.lhs, &mut reg_map, regs, &mut used);
            },
            IRType::MOV | IRType::Ope(_) => {
                ins[i].lhs = alloc(ir.lhs, &mut reg_map, regs, &mut used);
                ins[i].rhs = alloc(ir.rhs, &mut reg_map, regs, &mut used);
            },
            IRType::KILL => {
                kill(reg_map[ir.lhs] as usize, &mut used);
                ins[i].op = IRType::NOP;
            },
            _ =>  assert!(true),
        }
    }
}

pub fn gen_x86(regs: Vec<&str>, ins: Vec<IR>) {
    for ir in ins {
        match ir.op {
            IRType::IMN => {
                print!("\tmov {}, {}\n", regs[ir.lhs], ir.rhs);
            },
            IRType::RETURN => {
                print!("\tmov rax, {}\n", regs[ir.lhs]);
                print!("\tret\n");
            },
            IRType::Ope(o) => {
                match o {
                    '+' => print!("\tadd {}, {}\n", regs[ir.lhs], regs[ir.rhs]),
                    '-' => print!("\tsub {}, {}\n", regs[ir.lhs], regs[ir.rhs]),
                    '*' => {
                        print!("\tmov rax, {}\n", regs[ir.rhs]);
                        print!("\tmul {}\n", regs[ir.lhs]);
                        print!("\tmov {}, rax\n", regs[ir.lhs]);
                    }
                    _ => assert!(true),
                }
            }
            IRType::NOP => {},
            _ => assert!(true),
        }
    }
}
