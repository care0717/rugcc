extern crate rugcc;
use self::rugcc::common::{ND,  Node};
use std::collections::HashMap;

pub struct SemaGenerator {
    vars: HashMap<String, usize>,
    stack_size: usize,
}

impl SemaGenerator {
    pub fn new() -> SemaGenerator {
        SemaGenerator{vars: HashMap::new(), stack_size: 0}
    }
    fn walk(&mut self, mut node: Node) -> Node {
        match node.ty {
            ND::NUM => { return node},
            ND::IDENT => {
                if self.vars.get(&node.val.to_string()).is_none() {
                    unreachable!("undefined variable: {}", node.val);
                }
                node.ty = ND::LVAR;
                node.offset = *self.vars.get(&node.val).unwrap();
                return node
            },
            ND::VARDEF => {
                self.stack_size += 8;
                self.vars.insert(node.val.clone(), self.stack_size);
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
                return node
            },
            ND::RETURN => {
                node.expr = Some(Box::new(self.walk(*node.expr.unwrap())));
                return node
            },
            ND::CALL => {
                for i in 0..node.args.len() {
                    node.args[i] = self.walk(node.args[i].clone());
                }
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
            assert!(node.ty == ND::FUNC);
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
                ty: ND::FUNC,
                val: "main".to_string(),
                body: Some(Box::new(Node {
                    ty: ND::COMP_STMT,
                    stmts: [
                        Node {
                            ty: ND::RETURN,
                            expr: Some(Box::new(Node {
                                ty: ND::OPE('-'),
                                lhs: Some(Box::new(Node {
                                    ty: ND::OPE('/'),
                                    lhs: Some(Box::new(Node {
                                        ty: ND::OPE('+'),
                                        lhs: Some(Box::new(Node {
                                            ty: ND::NUM,
                                            val: "2".to_string(), ..Default::default() })),
                                        rhs: Some(Box::new(Node {
                                            ty: ND::OPE('*'),
                                            lhs: Some(Box::new(Node {
                                                ty: ND::NUM,
                                                val: "2".to_string(), ..Default::default()})),
                                            rhs: Some(Box::new(Node {
                                                ty: ND::NUM,
                                                val: "3".to_string(), ..Default::default()})),
                                            ..Default::default()})),
                                        ..Default::default()})),
                                    rhs: Some(Box::new(Node {
                                        ty: ND::NUM,
                                        val: "2".to_string(), ..Default::default()
                                    })),  ..Default::default()})),
                                rhs: Some(Box::new(Node {
                                    ty: ND::NUM,
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
                ty: ND::FUNC,
                val: "main".to_string(),
                body: Some(Box::new(Node {
                    ty: ND::COMP_STMT,
                    stmts: [
                        Node {
                            ty: ND::RETURN,
                            expr: Some(Box::new(Node {
                                ty: ND::OPE('-'),
                                lhs: Some(Box::new(Node {
                                    ty: ND::OPE('/'),
                                    lhs: Some(Box::new(Node {
                                        ty: ND::OPE('+'),
                                        lhs: Some(Box::new(Node {
                                            ty: ND::NUM,
                                            val: "2".to_string(), ..Default::default() })),
                                        rhs: Some(Box::new(Node {
                                            ty: ND::OPE('*'),
                                            lhs: Some(Box::new(Node {
                                                ty: ND::NUM,
                                                val: "2".to_string(), ..Default::default()})),
                                            rhs: Some(Box::new(Node {
                                                ty: ND::NUM,
                                                val: "3".to_string(), ..Default::default()})),
                                            ..Default::default()})),
                                        ..Default::default()})),
                                    rhs: Some(Box::new(Node {
                                        ty: ND::NUM,
                                        val: "2".to_string(), ..Default::default()
                                    })),  ..Default::default()})),
                                rhs: Some(Box::new(Node {
                                    ty: ND::NUM,
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
                ty: ND::FUNC,
                val: "add".to_string(),
                args: [
                    Node { ty: ND::VARDEF, val: "a".to_string(), ..Default::default() },
                    Node { ty: ND::VARDEF, val: "b".to_string(), ..Default::default() }
                ].to_vec(),
                body: Some(Box::new(Node {
                    ty: ND::COMP_STMT,
                    stmts: [
                        Node {
                            ty: ND::RETURN,
                            expr: Some(Box::new(Node {
                                ty: ND::OPE('+'),
                                lhs: Some(Box::new(Node { ty: ND::IDENT, val: "a".to_string(), ..Default::default() })),
                                rhs: Some(Box::new(Node { ty: ND::IDENT, val: "b".to_string(), ..Default::default() })),
                                ..Default::default() })),
                            ..Default::default()
                        }].to_vec(),
                    ..Default::default() })),
                ..Default::default()
            },
            Node {
                ty: ND::FUNC,
                val: "main".to_string(),
                body: Some(Box::new(Node {
                    ty: ND::COMP_STMT,
                    stmts: [
                        Node {
                            ty: ND::RETURN,
                            expr: Some(Box::new(Node {
                                ty: ND::CALL,
                                val: "add".to_string(),
                                args: [
                                    Node { ty: ND::NUM, val: "1".to_string(), ..Default::default()},
                                    Node { ty: ND::NUM, val: "2".to_string(), ..Default::default() }
                                ].to_vec(), ..Default::default() })),
                            ..Default::default() }
                    ].to_vec(),
                    ..Default::default()})),
                ..Default::default()
            }];

        let result = SemaGenerator::new().sema(input.to_vec());

        let expect = [
            Node {
                ty: ND::FUNC,
                val: "add".to_string(),
                args: [
                    Node { ty: ND::VARDEF, val: "a".to_string(), offset: 8, ..Default::default() },
                    Node { ty: ND::VARDEF, val: "b".to_string(), offset: 16,..Default::default() }
                ].to_vec(),
                body: Some(Box::new(Node {
                    ty: ND::COMP_STMT,
                    stmts: [
                        Node {
                            ty: ND::RETURN,
                            expr: Some(Box::new(Node {
                                ty: ND::OPE('+'),
                                lhs: Some(Box::new(Node { ty: ND::LVAR, val: "a".to_string(), offset: 8, ..Default::default() })),
                                rhs: Some(Box::new(Node { ty: ND::LVAR, val: "b".to_string(), offset: 16, ..Default::default() })),
                                ..Default::default() })),
                            ..Default::default()
                        }].to_vec(),
                    ..Default::default() })),
                stack_size: 16,
                ..Default::default()
            },
            Node {
                ty: ND::FUNC,
                val: "main".to_string(),
                body: Some(Box::new(Node {
                    ty: ND::COMP_STMT,
                    stmts: [
                        Node {
                            ty: ND::RETURN,
                            expr: Some(Box::new(Node {
                                ty: ND::CALL,
                                val: "add".to_string(),
                                args: [
                                    Node { ty: ND::NUM, val: "1".to_string(), ..Default::default()},
                                    Node { ty: ND::NUM, val: "2".to_string(), ..Default::default() }
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
