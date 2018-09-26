extern crate rugcc;
use self::rugcc::common::{TK, Token, error, ND,  Node};

use std;

fn new_node(op: char, lhs: Node, rhs: Node) -> Node {
    return Node{ty: ND::OPE(op), lhs: Some(Box::new(lhs)), rhs: Some(Box::new(rhs)), val: op.to_string(), expr: None, stmts: Vec::new()};
}

fn number(tokens: &mut Vec<Token>) -> Node {
    let token = tokens.pop().unwrap();
    if token.ty != TK::NUM { error("number expected, but got ", Some(&token.val))}
    return Node{ty: ND::NUM, lhs: None, rhs: None, val: token.val, expr: None, stmts: Vec::new()};
}

fn expect(ty: TK, tokens: &mut Vec<Token>) {
    let token = tokens.pop();
    if token.is_some() && token.unwrap().ty != ty {
        println!("{:?} expected, but not", ty);
        std::process::exit(1);
    }
    return
}

fn mul(mut tokens: &mut Vec<Token>) -> Node {
    let mut lhs = number(tokens);
    loop {
        let token = tokens.pop();
        match token {
            Some(t) => {
                let op = t.ty;
                match op {
                    TK::OPE(o) => lhs = new_node(o, lhs, number(&mut tokens)),
                    _ => break,
                }
            },
            None => break,
        }
    }

    return lhs;
}

fn expr(mut tokens: &mut Vec<Token>) -> Node {
    let mut lhs = mul(tokens);

    loop {
        let token = tokens.pop();
        match token {
            Some(t) => {
                let op = t.ty;
                match op {
                    TK::OPE(o) => lhs = new_node(o, lhs, mul(&mut tokens)),
                    _ => break,
                }
            },
            None => break,
        }
    }

    return lhs;
}

pub fn stmt(tokens: &mut Vec<Token>) -> Node{
    let mut node = Node{ty: ND::COMP_STMT, lhs: None, rhs: None, val: String::new(), expr: None, stmts: Vec::new()};
    loop {
        let optoken = tokens.pop();
        if optoken.is_none() {return node}
        let token = optoken.unwrap();
        if token.ty == TK::EOF { return node }
        let ty = if token.ty == TK::RETURN {
            ND::RETURN
        } else {
            ND::EXPR_STMT
        };
        let e = Node{ty, lhs: None, rhs: None, val: String::new(), expr: Some(Box::new(expr(tokens))), stmts: Vec::new()};
        node.stmts.push(e);
        expect(TK::END_LINE, tokens);
    }
}