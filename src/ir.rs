extern crate rugcc;
use self::rugcc::common::{IR, ND, Node, IRType, Function};

pub struct IrGenerator {
    code: Vec<IR>,
    regno: usize,
    label: usize,
}

impl IrGenerator {
    pub fn new() -> IrGenerator {
        return IrGenerator{code: Vec::new(), regno: 1, label: 0}
    }

    fn add(&mut self, op: IRType, lhs: usize, rhs: usize) {
        self.code.push(IR { op, lhs, rhs, ..Default::default()});
    }

    fn kill(&mut self, r: usize) {  self.add(IRType::KILL, r, 0); }

    fn label(&mut self, x: usize) { self.add(IRType::LABEL, x, 0); }

    fn gen_lval(&mut self, node: Node) -> usize {
        if node.op != ND::LVAR {
            unreachable!("not an lvalue: {:?} ({})", node.op, node.val);
        }
        let r = self.regno;
        self.regno += 1;

        self.add(IRType::MOV, r, 0);
        self.add(IRType::SUB_IMM, r, node.offset);
        return r;
    }

    fn gen_binop(&mut self, ty: IRType, lhs: Node, rhs: Node) -> usize {
        let r1 = self.gen_expr(lhs);
        let r2 = self.gen_expr(rhs);
        self.add(ty, r1, r2);
        self.kill(r2);
        return r1;
    }


    fn gen_expr(&mut self, node: Node) -> usize {
        match node.op {
            ND::NUM => {
                let r = self.regno;
                self.regno += 1;
                self.add(IRType::IMM, r, node.val.parse::<usize>().unwrap());
                return r
            },
            ND::LVAR => {
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
                self.kill(r2);
                self.add(IRType::UNLESS, r1, x);
                self.add(IRType::IMM, r1, 1);
                self.label(x);
                return r1
            },
            ND::LOGOR => {
                let x = self.label;
                self.label += 1;
                let y = self.label;
                self.label += 1;

                let r1 = self.gen_expr(*node.lhs.unwrap());
                self.add(IRType::UNLESS, r1, x);
                self.add(IRType::IMM, r1, 1);
                self.add(IRType::JMP, y, 0);
                self.label(x);

                let r2 = self.gen_expr(*node.rhs.unwrap());
                self.add(IRType::MOV, r1, r2);
                self.kill(r2);
                self.add(IRType::UNLESS, r1, y);
                self.add(IRType::IMM, r1, 1);
                self.label(y);
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
                    self.kill(i);
                }
                return r
            },
            ND::DEREF => {
                let r = self.gen_expr(*node.expr.unwrap());
                self.add(IRType::LOAD, r, r);
                return r
            },
            ND::OPE('=') => {
                let rhs = self.gen_expr(*node.rhs.unwrap());
                let lhs = self.gen_lval(*node.lhs.unwrap());
                self.add(IRType::STORE, lhs, rhs);
                self.kill(rhs);
                return lhs
            },
            ND::OPE('<') => {
                return self.gen_binop(IRType::LT, *node.lhs.unwrap(), *node.rhs.unwrap())
            },
            ND::OPE('+') =>{
                return self.gen_binop(IRType::ADD, *node.lhs.unwrap(), *node.rhs.unwrap())
            },
            ND::OPE('-') =>{
                return self.gen_binop(IRType::SUB, *node.lhs.unwrap(), *node.rhs.unwrap())
            },
            ND::OPE('*') =>{
                return self.gen_binop(IRType::MUL, *node.lhs.unwrap(), *node.rhs.unwrap())
            },
            ND::OPE('/') =>{
                return self.gen_binop(IRType::DIV, *node.lhs.unwrap(), *node.rhs.unwrap())
            },
            _ => { unreachable!("unexpected node type:{:?}", node.op)}
        }
    }

    fn gen_stmt(&mut self, node: Node) {
        match node.op {
            ND::VARDEF => {
                if node.init.is_none() { return }

                let rhs = self.gen_expr(*node.init.unwrap());
                let lhs = self.regno;
                self.regno += 1;
                self.add(IRType::MOV, lhs, 0);
                self.add(IRType::SUB_IMM, lhs, node.offset);
                self.add(IRType::STORE, lhs, rhs);
                self.kill(lhs);
                self.kill(rhs);
            },
            ND::IF => {
                let x = self.label;
                self.label += 1;
                if node.els.is_some() {
                    let y = self.label;
                    self.label += 1;
                    let r = self.gen_expr(*node.cond.unwrap());
                    self.add(IRType::UNLESS, r, x);
                    self.kill(r);
                    self.gen_stmt(*node.then.unwrap());
                    self.add(IRType::JMP, y, 0);
                    self.label(x);
                    self.gen_stmt(*node.els.unwrap());
                    self.label(y);
                } else {
                    let r = self.gen_expr(*node.cond.unwrap());
                    self.add(IRType::UNLESS, r, x);
                    self.kill(r);
                    self.gen_stmt(*node.then.unwrap());
                    self.label(x);
                }
            },
            ND::FOR => {
                let x = self.label;
                self.label += 1;
                let y = self.label;
                self.label += 1;
                self.gen_stmt(*node.init.unwrap());
                self.label(x);
                let r2 = self.gen_expr(*node.cond.unwrap());
                self.add(IRType::UNLESS, r2, y);
                self.kill(r2);
                self.gen_stmt(*node.body.unwrap());
                let r3 = self.gen_expr(*node.inc.unwrap());
                self.kill(r3);
                self.add(IRType::JMP, x, 0);
                self.label(y);
            },
            ND::RETURN => {
                let r = self.gen_expr(*node.expr.unwrap());
                self.add(IRType::RETURN, r, 0);
                self.kill(r);
            },
            ND::EXPR_STMT => {
                let r = self.gen_expr(*node.expr.unwrap());
                self.kill(r);
            },
            ND::COMP_STMT => {
                for n in node.stmts {
                    self.gen_stmt(n);
                }
            },
            _ => unreachable!("unknown node: {:?}", node.op)
        }
    }

    pub fn gen_ir(&mut self, nodes: Vec<Node>) -> Vec<Function> {
        let mut funcs = Vec::new();
        for node in nodes {
            assert!(node.op ==ND::FUNC);
            self.code= Vec::new();
            self.regno = 1;
            let name = node.val.clone();
            if node.args.len() > 0 {
                self.add(IRType::SAVE_ARGS, node.args.len(), 0);
            }
            self.gen_stmt(*node.body.unwrap());
            funcs.push(Function{name, irs: self.code.clone(), stack_size: node.stack_size, ..Default::default()})
        }
        return funcs
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    # [test]
    fn can_gen_ir_arithmetic_expr() {
        let input = [
            Node {
                op: ND::FUNC,
                val: "main".to_string(),
                body: Some(Box::new(Node {
                    op: ND::COMP_STMT,
                    stmts: [
                        Node {
                            op: ND::RETURN,
                            expr: Some(Box::new(Node {
                                op: ND::OPE('-'),
                                lhs: Some(Box::new(Node {
                                    op: ND::OPE('/'),
                                    lhs: Some(Box::new(Node {
                                        op: ND::OPE('+'),
                                        lhs: Some(Box::new(Node {
                                            op: ND::NUM,
                                            val: "2".to_string(), ..Default::default() })),
                                        rhs: Some(Box::new(Node {
                                            op: ND::OPE('*'),
                                            lhs: Some(Box::new(Node {
                                                op: ND::NUM,
                                                val: "2".to_string(), ..Default::default()})),
                                            rhs: Some(Box::new(Node {
                                                op: ND::NUM,
                                                val: "3".to_string(), ..Default::default()})),
                                            ..Default::default()})),
                                        ..Default::default()})),
                                    rhs: Some(Box::new(Node {
                                        op: ND::NUM,
                                        val: "2".to_string(), ..Default::default()
                                    })),  ..Default::default()})),
                                rhs: Some(Box::new(Node {
                                    op: ND::NUM,
                                    val: "1".to_string(), ..Default::default() })),
                                ..Default::default()
                            })),
                            ..Default::default()
                        }].to_vec(),
                    ..Default::default() })),
                ..Default::default()}];

        let result = IrGenerator::new().gen_ir(input.to_vec());

        let expect = [
            Function {
                name: "main".to_string(),
                irs: [
                    IR { op: IRType::IMM, lhs: 1, rhs: 2, ..Default::default()},
                    IR { op: IRType::IMM, lhs: 2, rhs: 2, ..Default::default() },
                    IR { op: IRType::IMM, lhs: 3, rhs: 3, ..Default::default() },
                    IR { op: IRType::MUL, lhs: 2, rhs: 3, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 3, ..Default::default() },
                    IR { op: IRType::ADD, lhs: 1, rhs: 2, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 2, ..Default::default() },
                    IR { op: IRType::IMM, lhs: 4, rhs: 2, ..Default::default() },
                    IR { op: IRType::DIV, lhs: 1, rhs: 4, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 4, ..Default::default() },
                    IR { op: IRType::IMM, lhs: 5, rhs: 1, ..Default::default() },
                    IR { op: IRType::SUB, lhs: 1, rhs: 5, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 5, ..Default::default() },
                    IR { op: IRType::RETURN, lhs: 1, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 1, ..Default::default() }].to_vec(),
                stack_size: 0 }];

        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }

    # [test]
    fn can_gen_ir_function() {
        let input = [
            Node {
                op: ND::FUNC,
                val: "add".to_string(),
                args: [
                    Node { op: ND::VARDEF, val: "a".to_string(), offset: 8, ..Default::default() },
                    Node { op: ND::VARDEF, val: "b".to_string(), offset: 16,..Default::default() }
                ].to_vec(),
                body: Some(Box::new(Node {
                    op: ND::COMP_STMT,
                    stmts: [
                        Node {
                            op: ND::RETURN,
                            expr: Some(Box::new(Node {
                                op: ND::OPE('+'),
                                lhs: Some(Box::new(Node { op: ND::LVAR, val: "a".to_string(), offset: 8, ..Default::default() })),
                                rhs: Some(Box::new(Node { op: ND::LVAR, val: "b".to_string(), offset: 16, ..Default::default() })),
                                ..Default::default() })),
                            ..Default::default()
                        }].to_vec(),
                    ..Default::default() })),
                stack_size: 16,
                ..Default::default()
            },
            Node {
                op: ND::FUNC,
                val: "main".to_string(),
                body: Some(Box::new(Node {
                    op: ND::COMP_STMT,
                    stmts: [
                        Node {
                            op: ND::RETURN,
                            expr: Some(Box::new(Node {
                                op: ND::CALL,
                                val: "add".to_string(),
                                args: [
                                    Node { op: ND::NUM, val: "1".to_string(), ..Default::default()},
                                    Node { op: ND::NUM, val: "2".to_string(), ..Default::default() }
                                ].to_vec(), ..Default::default() })),
                            ..Default::default() }
                    ].to_vec(),
                    ..Default::default()})),
                ..Default::default()
            }];

        let result = IrGenerator::new().gen_ir(input.to_vec());

        let expect = [
            Function {
                name: "add".to_string(),
                irs: [
                    IR { op: IRType::SAVE_ARGS, lhs: 2, rhs: 0, ..Default::default() },
                    IR { op: IRType::MOV, lhs: 1, rhs: 0, ..Default::default() },
                    IR { op: IRType::SUB_IMM, lhs: 1, rhs: 8, ..Default::default() },
                    IR { op: IRType::LOAD, lhs: 1, rhs: 1, ..Default::default() },
                    IR { op: IRType::MOV, lhs: 2, rhs: 0, ..Default::default() },
                    IR { op: IRType::SUB_IMM, lhs: 2, rhs: 16, ..Default::default() },
                    IR { op: IRType::LOAD, lhs: 2, rhs: 2, ..Default::default() },
                    IR { op: IRType::ADD, lhs: 1, rhs: 2, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 2, rhs: 0, ..Default::default() },
                    IR { op: IRType::RETURN, lhs: 1, rhs: 0, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 1, rhs: 0, ..Default::default() }].to_vec(),
                stack_size: 16 },
            Function {
                name: "main".to_string(),
                irs: [
                    IR { op: IRType::IMM, lhs: 1, rhs: 1, ..Default::default() },
                    IR { op: IRType::IMM, lhs: 2, rhs: 2, ..Default::default() },
                    IR { op: IRType::CALL, lhs: 3, rhs: 0, name: "add".to_string(), args: [1, 2].to_vec() },
                    IR { op: IRType::KILL, lhs: 1, rhs: 0, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 2, rhs: 0, ..Default::default() },
                    IR { op: IRType::RETURN, lhs: 3, rhs: 0, ..Default::default() },
                    IR { op: IRType::KILL, lhs: 3, rhs: 0, ..Default::default() }].to_vec(),
                stack_size: 0 }];

        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }
}
