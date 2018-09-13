extern crate rugcc;
use self::rugcc::common::{TY, Token, error, Node};

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