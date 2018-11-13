extern crate rugcc;
use self::rugcc::common::{ND,  Node, Type};
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
struct Var {
    ty: Type,
    offset: usize,
}

pub struct SemaGenerator {
    vars: HashMap<String, Var>,
    stack_size: usize,
}

impl SemaGenerator {
    pub fn new() -> SemaGenerator {
        SemaGenerator{vars: HashMap::new(), stack_size: 0}
    }
    fn walk(&mut self, mut node: Node) -> Node {
        match node.op {
            ND::NUM => { return node},
            ND::IDENT => {
                if self.vars.get(&node.val.to_string()).is_none() {
                    unreachable!("undefined variable: {}", node.val);
                }
                node.op = ND::LVAR;
                let var = self.vars.get(&node.val).unwrap().clone();
                node.offset = var.offset;
                node.ty = var.ty;
                return node
            },
            ND::VARDEF => {
                self.stack_size += 8;
                self.vars.insert(node.val.clone(), Var{ty: node.ty.clone(), offset: self.stack_size});
                node.offset = self.stack_size;
                if node.init.is_some() {
                    node.init = Some(Box::new(self.walk(*node.init.unwrap())));
                }
                return node
            },
            ND::IF => {
                node.cond = Some(Box::new(self.walk(*node.cond.unwrap())));
                node.then = Some(Box::new(self.walk(*node.then.unwrap())));
                if node.els.is_some() { node.els =  Some(Box::new(self.walk(*node.els.unwrap()))); }
                return node
            },
            ND::FOR => {
                node.init = Some(Box::new(self.walk(*node.init.unwrap())));
                node.cond = Some(Box::new(self.walk(*node.cond.unwrap())));
                node.inc = Some(Box::new(self.walk(*node.inc.unwrap())));
                node.body = Some(Box::new(self.walk(*node.body.unwrap())));
                return node
            },
            ND::OPE(_) | ND::LOGAND | ND::LOGOR => {
                node.lhs = Some(Box::new(self.walk(*node.lhs.unwrap())));
                node.rhs = Some(Box::new(self.walk(*node.rhs.unwrap())));
                node.ty = node.lhs.clone().unwrap().ty;
                return node
            },
            ND::RETURN | ND::DEREF => {
                node.expr = Some(Box::new(self.walk(*node.expr.unwrap())));
                return node
            },
            ND::CALL => {
                for i in 0..node.args.len() {
                    node.args[i] = self.walk(node.args[i].clone());
                }
                node.ty = Type{..Default::default()};
                return node
            },
            ND::FUNC => {
                for i in 0..node.args.len() {
                    node.args[i] = self.walk(node.args[i].clone());
                }
                node.body = Some(Box::new(self.walk(*node.body.unwrap())));
                return node
            },
            ND::COMP_STMT => {
                for i in 0..node.stmts.len() {
                    node.stmts[i] = self.walk(node.stmts[i].clone());
                }
                return node
            },
            ND::EXPR_STMT => {
                node.expr = Some(Box::new(self.walk(*node.expr.unwrap())));
                return node
            },
            ND::LVAR => {unreachable!("unexpected type: LVAR");}
        }
    }
    pub fn sema(&mut self, nodes: Vec<Node>)  -> Vec<Node>{
        let mut res = Vec::new();
        for mut node in nodes.clone() {
            assert!(node.op == ND::FUNC);
            self.vars = HashMap::new();
            self.stack_size = 0;
            node = self.walk(node);
            node.stack_size = self.stack_size;
            res.push(node);
        }
        return res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    # [test]
    fn can_gen_walk_arithmetic_expr() {
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

        let result = SemaGenerator::new().sema(input.to_vec());

        let expect = [
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

        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }

    # [test]
    fn can_gen_walk_function() {
        let input = [
            Node {
                op: ND::FUNC,
                val: "add".to_string(),
                args: [
                    Node { op: ND::VARDEF, val: "a".to_string(), ..Default::default() },
                    Node { op: ND::VARDEF, val: "b".to_string(), ..Default::default() }
                ].to_vec(),
                body: Some(Box::new(Node {
                    op: ND::COMP_STMT,
                    stmts: [
                        Node {
                            op: ND::RETURN,
                            expr: Some(Box::new(Node {
                                op: ND::OPE('+'),
                                lhs: Some(Box::new(Node { op: ND::IDENT, val: "a".to_string(), ..Default::default() })),
                                rhs: Some(Box::new(Node { op: ND::IDENT, val: "b".to_string(), ..Default::default() })),
                                ..Default::default() })),
                            ..Default::default()
                        }].to_vec(),
                    ..Default::default() })),
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

        let result = SemaGenerator::new().sema(input.to_vec());

        let expect = [
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

        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }
}
