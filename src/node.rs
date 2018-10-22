extern crate rugcc;
use self::rugcc::common::{TK, Token, error, ND,  Node};

use std;

fn new_node(op: char, lhs: Node, rhs: Node) -> Node {
    return Node{ty: ND::OPE(op), lhs: Some(Box::new(lhs)), rhs: Some(Box::new(rhs)), val: op.to_string(), expr: None, stmts: Vec::new(), cond: None, then: None};
}

fn term(tokens: &mut Vec<Token>) -> Node {
    let token = tokens.pop().unwrap();
    if token.ty == TK::OPE('(') {
        let node = assign(tokens);
        expect(TK::OPE(')'), tokens);
        return node
    }

    if token.ty !=  TK::NUM && token.ty !=  TK::IDENT {
        error("number expected, but got ", Some(&token.val))
    }
    if token.ty == TK::NUM {
        return Node{ty: ND::NUM, lhs: None, rhs: None, val: token.val, expr: None, stmts: Vec::new(), cond: None, then: None};
    } else {
        return Node{ty: ND::IDENT, lhs: None, rhs: None, val: token.val, expr: None, stmts: Vec::new(), cond: None, then: None};
    }
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
    let mut lhs = term(tokens);
    loop {
        let token = tokens.pop();
        match token {
            Some(t) => {
                let op = t.clone().ty;
                match op {
                    TK::OPE(o) => {
                        match o {
                            '+' | '-' | '*' | '/' => lhs = new_node(o, lhs, term(&mut tokens)),
                            _ => {
                                tokens.push(t);
                                break
                            },
                        }
                    },
                    _ => {
                        tokens.push(t);
                        break
                    },
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
                let op = t.clone().ty;
                match op {
                    TK::OPE(o) => {
                        match o {
                            '+' | '-' => lhs = new_node(o, lhs, term(&mut tokens)),
                            _ => {
                                tokens.push(t);
                                break
                            },
                        }
                    },                    _ => {
                        tokens.push(t);
                        break
                    },
                }
            },
            None => break,
        }
    }

    return lhs;
}

fn consume(ope: char, tokens: &mut Vec<Token>) -> bool {
    let token = tokens.pop();
    match token {
        Some(t) => {
            if t.ty == TK::OPE(ope) {
                return true
            } else {
                tokens.push(t);
                return false
            }
        },
        None => return false,
    }
}

fn assign(tokens: &mut Vec<Token>) -> Node {
    let lhs = expr(tokens);
    if consume('=', tokens) {
        return new_node('=', lhs, expr(tokens));
    } else {
        return lhs;
    }
}

fn stmt(tokens: &mut Vec<Token>) -> Node {
    let token = tokens.pop().unwrap();
    let mut node = Node {ty: ND::EXPR_STMT, lhs: None, rhs: None, val: String::new(), expr: None, stmts: Vec::new(), cond: None, then: None};

    match token.ty {
        TK::IF => {
            node.ty = ND::IF;
            expect(TK::OPE('('), tokens);
            node.cond = Some(Box::new(assign(tokens)));
            expect(TK::OPE(')'), tokens);
            node.then = Some(Box::new(stmt(tokens)));
            return node
        }
        TK::RETURN => {
            node.ty = ND::RETURN;
            node.expr = Some(Box::new(assign(tokens)));
            expect(TK::END_LINE, tokens);
            return node
        }
        _ => {
            tokens.push(token);
            node.expr = Some(Box::new(assign(tokens)));
            expect(TK::END_LINE, tokens);
            return node
        }
    }

}

pub fn parse(tokens: &mut Vec<Token>) -> Node{
    let mut node = Node{ty: ND::COMP_STMT, lhs: None, rhs: None, val: String::new(), expr: None, stmts: Vec::new(), cond: None, then: None};
    loop {
        let optoken = tokens.pop();
        if optoken.is_none() {return node}
        let token = optoken.unwrap();
        if token.ty == TK::EOF { return node }
        tokens.push(token);
        node.stmts.push(stmt(tokens));
    }
}