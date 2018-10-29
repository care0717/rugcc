extern crate rugcc;
use self::rugcc::common::{IR, ND, Node, IRType, error, Function};
use std::collections::HashMap;

pub struct IrGenerator {
    code: Vec<IR>,
    regno: usize,
    vars: HashMap<String, usize>,
    stack_size: usize,
    label: usize,
}

impl IrGenerator {
    pub fn new() -> IrGenerator {
        return IrGenerator{code: Vec::new(), regno: 1, vars: HashMap::new(), label: 0, stack_size: 0}
    }

    fn add(&mut self, op: IRType, lhs: usize, rhs: usize) {
        self.code.push(IR { op, lhs, rhs, ..Default::default()});
    }

    fn gen_lval(&mut self, node: Node) -> usize {
        if node.ty != ND::IDENT {
            error("not an lvalue", None);
        }
        let off = if self.vars.get(&node.val.to_string()).is_none() {
            self.stack_size += 8;
            self.vars.insert(node.val, self.stack_size);
            self.stack_size
        } else {
            *self.vars.get(&node.val.to_string()).unwrap()
        };
        let r = self.regno;
        self.regno += 1;
        self.add(IRType::MOV, r, 0);
        self.code.push(IR { op: IRType::SUB_IMN, lhs: r, rhs: off, ..Default::default()});
        return r;
    }

    fn gen_binop(&mut self, ty: IRType, lhs: Node, rhs: Node) -> usize {
        let r1 = self.gen_expr(lhs);
        let r2 = self.gen_expr(rhs);
        self.add(ty, r1, r2);
        self.add(IRType::KILL, r2, 0);
        return r1;
    }


    fn gen_expr(&mut self, node: Node) -> usize {
        match node.ty {
            ND::NUM => {
                let r = self.regno;
                self.regno += 1;
                self.add(IRType::IMN, r, node.val.parse().unwrap());
                return r
            },
            ND::IDENT => {
                let r = self.gen_lval(node);
                self.add(IRType::LOAD, r, r);
                return r
            },
            ND::LOGAND => {
                let x = self.label;
                self.label += 1;
                let r1 = self.gen_expr(*node.lhs.unwrap());
                self.add(IRType::UNLESS, r1, x);
                let r2 = self.gen_expr(*node.rhs.unwrap());
                self.add(IRType::MOV, r1, r2);
                self.add(IRType::KILL, r2, 0);
                self.add(IRType::UNLESS, r1, x);
                self.add(IRType::IMN, r1, 1);
                self.add(IRType::LABEL, x, 0);
                return r1
            },
            ND::LOGOR => {
                let x = self.label;
                self.label += 1;
                let y = self.label;
                self.label += 1;

                let r1 = self.gen_expr(*node.lhs.unwrap());
                self.add(IRType::UNLESS, r1, x);
                self.add(IRType::IMN, r1, 1);
                self.add(IRType::JMP, y, 0);
                self.add(IRType::LABEL, x, 0);

                let r2 = self.gen_expr(*node.rhs.unwrap());
                self.add(IRType::MOV, r1, r2);
                self.add(IRType::KILL, r2, 0);
                self.add(IRType::UNLESS, r1, y);
                self.add(IRType::IMN, r1, 1);
                self.add(IRType::LABEL, y, 0);
                return r1;
            },
            ND::CALL => {
                let mut args = Vec::new();
                for n in node.args {
                    args.push(self.gen_expr(n));
                }
                let r = self.regno;
                self.regno += 1;

                let ir = IR { op: IRType::CALL, lhs: r, rhs: 0, name: node.val, args, ..Default::default() };
                self.code.push(ir.clone());
                for i in ir.args {
                    self.add(IRType::KILL, i, 0);
                }
                return r
            },
            ND::OPE('=') => {
                let rhs = self.gen_expr(*node.rhs.unwrap());
                let lhs = self.gen_lval(*node.lhs.unwrap());
                self.add(IRType::STORE, lhs, rhs);
                self.add(IRType::KILL, rhs, 0);
                return lhs
            },
            ND::OPE('<') => {
                return self.gen_binop(IRType::LT, *node.lhs.unwrap(), *node.rhs.unwrap())
            },
            ND::OPE(o) =>{
                return self.gen_binop(IRType::Ope(o), *node.lhs.unwrap(), *node.rhs.unwrap())
            },
            _ => {
                assert!(false);
                return 0
            }
        }
    }

    fn gen_stmt(&mut self, node: Node) {
        match node.ty {
            ND::IF => {
                let r = self.gen_expr(*node.cond.unwrap());
                let x = self.label;
                self.label += 1;
                self.add(IRType::UNLESS, r, x);
                self.add(IRType::KILL, r, 0);
                self.gen_stmt(*node.then.unwrap());
                if node.els.is_none() {
                    self.add(IRType::LABEL, x, 0);
                    return;
                }
                let y = self.label;
                self.label += 1;
                self.add(IRType::JMP, y, 0);
                self.add(IRType::LABEL, x, 0);
                self.gen_stmt(*node.els.unwrap());
                self.add(IRType::LABEL, y, 0);
            },
            ND::RETURN => {
                let r = self.gen_expr(*node.expr.unwrap());
                self.add(IRType::RETURN, r, 0);
                self.add(IRType::KILL, r, 0);
            },
            ND::EXPR_STMT => {
                let r = self.gen_expr(*node.expr.unwrap());
                self.add(IRType::KILL, r, 0);
            },
            ND::COMP_STMT => {
                for n in node.stmts {
                    self.gen_stmt(n);
                }
            },
            _ => error("unknown node: ", Some(&"aa".to_string())),
        }
    }
    fn gen_args(&mut self, nodes: Vec<Node>) {
        if nodes.len() == 0 {
            return
        }
        self.add(IRType::SAVE_ARGS, nodes.len(), 0);
        for node in nodes {
            if node.ty != ND::IDENT {  error("bad parameter", None); }
            self.stack_size += 8;
            self.vars.insert(node.val, self.stack_size);
        }
    }

    pub fn gen_ir(&mut self, nodes: Vec<Node>) -> Vec<Function> {
        let mut funcs = Vec::new();
        for node in nodes {
            assert!(node.ty==ND::FUNC);
            self.code= Vec::new();
            self.regno = 1;
            self.vars = HashMap::new();
            self.label = 0;
            self.stack_size = 0;
            let name = node.val.clone();
            self.gen_args(node.args);
            self.gen_stmt(*node.body.unwrap());
            funcs.push(Function{name, irs: self.code.clone(), stack_size: self.stack_size, ..Default::default()})
        }
        return funcs
    }
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

        let result = IrGenerator::new().gen_ir(input.to_vec());
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
