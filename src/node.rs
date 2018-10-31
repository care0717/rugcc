extern crate rugcc;
use self::rugcc::common::{TK, Token, error, ND,  Node};

use std;

fn new_node(ty: ND, lhs: Node, rhs: Node) -> Node {
    return Node{ty, lhs: Some(Box::new(lhs)), rhs: Some(Box::new(rhs)), ..Default::default()};
}

fn term(tokens: &mut Vec<Token>) -> Node {
    let token = tokens.pop().unwrap();

    match token.ty {
        TK::OPE('(') => {
            let node = assign(tokens);
            expect(TK::OPE(')'), tokens);
            return node
        },
        TK::NUM => {
            return Node{ty: ND::NUM, val: token.val, ..Default::default()};
        },
        TK::IDENT => {
            let mut node = Node{ty: ND::IDENT, val: token.val, ..Default::default()};
            if !consume(TK::OPE('('), tokens) {
                return node
            }
            node.ty = ND::CALL;
            if consume(TK::OPE(')'), tokens) {return node}
            node.args.push(assign(tokens));
            while consume(TK::OPE(','), tokens) {
                node.args.push(assign(tokens));
            }
            expect(TK::OPE(')'), tokens);
            return node
        },
        _ => {
            error("number expected, but got ", Some(&token.val));
            // イケてない　通るはずのないreturn
            return Node{ty: ND::NUM, val: token.val, ..Default::default()}
        },
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

fn mul(tokens: &mut Vec<Token>) -> Node {
    let mut lhs = term(tokens);
    loop {
        let token = tokens.pop().unwrap();
        match token.ty {
            TK::OPE(o) => {
                match o {
                    '*' | '/' => lhs = new_node(ND::OPE(o), lhs, term(tokens)),
                    _ => {
                        tokens.push(token);
                        break
                    },
                }
            },
            _ => {
                tokens.push(token);
                break
            },
        }
    }
    return lhs;
}

fn add(tokens: &mut Vec<Token>) -> Node {
    let mut lhs = mul(tokens);
    loop {
        let token = tokens.pop().unwrap();
        match token.ty {
            TK::OPE(o) => {
                match o {
                    '+' | '-' => lhs = new_node(ND::OPE(o), lhs, mul(tokens)),
                    _ => {
                        tokens.push(token);
                        break
                    },
                }
            },
            _ => {
                tokens.push(token);
                break
            },
        }
    }
    return lhs;
}

fn rel(tokens: &mut Vec<Token>) -> Node {
    let mut lhs = add(tokens);
    loop {
        let token = tokens.pop();
        match token {
            Some(t) => {
                let op = t.clone().ty;
                match op {
                    TK::OPE('<') => {
                        lhs = new_node(ND::OPE('<'), lhs, add(tokens));
                    },
                    TK::OPE('>') => {
                        lhs = new_node(ND::OPE('<'), add(tokens), lhs);
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

fn logand(tokens: &mut Vec<Token>) -> Node {
    let mut lhs = rel(tokens);
    loop {
        let token = tokens.pop();
        match token {
            Some(t) => {
                let op = t.clone().ty;
                match op {
                    TK::LOGAND => {
                        lhs = new_node(ND::LOGAND, lhs, rel(tokens));
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

fn logor(tokens: &mut Vec<Token>) -> Node {
    let mut lhs = logand(tokens);
    loop {
        let token = tokens.pop();
        match token {
            Some(t) => {
                let op = t.clone().ty;
                match op {
                    TK::LOGOR => {
                        lhs = new_node(ND::LOGOR, lhs, logand(tokens));
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

fn consume(ope: TK, tokens: &mut Vec<Token>) -> bool {
    let token = tokens.pop();
    match token {
        Some(t) => {
            if t.ty == ope {
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
    let lhs = logor(tokens);
    if consume(TK::OPE('='), tokens) {
        return new_node(ND::OPE('='), lhs, logor(tokens));
    } else {
        return lhs;
    }
}

fn stmt(tokens: &mut Vec<Token>) -> Node {
    let token = tokens.pop().unwrap();
    let mut node = Node {ty: ND::EXPR_STMT, ..Default::default()};

    match token.ty {
        TK::IF => {
            node.ty = ND::IF;
            expect(TK::OPE('('), tokens);
            node.cond = Some(Box::new(assign(tokens)));
            expect(TK::OPE(')'), tokens);
            node.then = Some(Box::new(stmt(tokens)));
            if consume(TK::ELSE, tokens) {node.els = Some(Box::new(stmt(tokens)));}
            return node
        },
        TK::FOR => {
            node.ty = ND::FOR;
            expect(TK::OPE('('), tokens);
            node.init = Some(Box::new(assign(tokens)));
            expect(TK::END_LINE, tokens);
            node.cond = Some(Box::new(assign(tokens)));
            expect(TK::END_LINE, tokens);
            node.inc = Some(Box::new(assign(tokens)));
            expect(TK::OPE(')'), tokens);
            node.body = Some(Box::new(stmt(tokens)));
            return node;
        },
        TK::RETURN => {
            node.ty = ND::RETURN;
            node.expr = Some(Box::new(assign(tokens)));
            expect(TK::END_LINE, tokens);
            return node
        },
        _ => {
            tokens.push(token);
            node.expr = Some(Box::new(assign(tokens)));
            expect(TK::END_LINE, tokens);
            return node
        }
    }
}

fn compound_stmt(tokens: &mut Vec<Token>) -> Node{
    let mut node = Node{ty: ND::COMP_STMT, ..Default::default()};
    while !consume(TK::OPE('}'), tokens) {
        let optoken = tokens.pop();
        if optoken.is_none() {return node}
        let token = optoken.unwrap();
        if token.ty == TK::EOF { return node }
        tokens.push(token);
        node.stmts.push(stmt(tokens));
    }
    return node
}

fn function(tokens: &mut Vec<Token>) -> Node{
    let token = tokens.pop().unwrap();
    if token.ty != TK::IDENT {error("function name expected, but got {}", Some(&token.val))}
    expect(TK::OPE('('), tokens);
    let mut node = Node{ty: ND::FUNC, val: token.val, ..Default::default()};
    if !consume(TK::OPE(')'), tokens) {
        node.args.push(term(tokens));
        while consume(TK::OPE(','), tokens){
            node.args.push(term(tokens));
        }
        expect(TK::OPE(')'), tokens);
    }

    expect(TK::OPE('{'), tokens);
    node.body = Some(Box::new(compound_stmt(tokens)));
    return node;
}

pub fn parse(tokens: &mut Vec<Token>) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut token = tokens.pop().unwrap();
    while token.ty != TK::EOF {
        tokens.push(token);
        nodes.push(function(tokens));
        token = tokens.pop().unwrap();
    }
    return nodes
}


#[cfg(test)]
mod tests {
    use super::*;
    # [test]
    fn can_parse_arithmetic_expr() {
        let input = [
            Token { ty: TK::EOF, val: "EOF".to_string() }, Token { ty: TK::OPE('}'), val: "}".to_string() },
            Token { ty: TK::END_LINE, val: ";".to_string() }, Token { ty: TK::NUM, val: "1".to_string() },
            Token { ty: TK::OPE('-'), val: "-".to_string() }, Token { ty: TK::NUM, val: "2".to_string() },
            Token { ty: TK::OPE('/'), val: "/".to_string() }, Token { ty: TK::OPE(')'), val: ")".to_string() },
            Token { ty: TK::NUM, val: "3".to_string() }, Token { ty: TK::OPE('*'), val: "*".to_string() },
            Token { ty: TK::NUM, val: "2".to_string() }, Token { ty: TK::OPE('+'), val: "+".to_string() },
            Token { ty: TK::NUM, val: "2".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() },
            Token { ty: TK::RETURN, val: "return".to_string() }, Token { ty: TK::OPE('{'), val: "{".to_string() },
            Token { ty: TK::OPE(')'), val: ")".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() }, Token { ty: TK::IDENT, val: "main".to_string() }
        ];
        let result = parse(&mut input.to_vec());

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
    fn can_parse_function() {
        let input = [
            Token { ty: TK::EOF, val: "EOF".to_string() }, Token { ty: TK::OPE('}'), val: "}".to_string() },
            Token { ty: TK::END_LINE, val: ";".to_string() }, Token { ty: TK::OPE(')'), val: ")".to_string() },
            Token { ty: TK::NUM, val: "2".to_string() }, Token { ty: TK::OPE(','), val: ",".to_string() },
            Token { ty: TK::NUM, val: "1".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() },
            Token { ty: TK::IDENT, val: "add".to_string() }, Token { ty: TK::RETURN, val: "return".to_string() },
            Token { ty: TK::OPE('{'), val: "{".to_string() }, Token { ty: TK::OPE(')'), val: ")".to_string() },
            Token { ty: TK::OPE('('), val: "(".to_string() }, Token { ty: TK::IDENT, val: "main".to_string() },
            Token { ty: TK::OPE('}'), val: "}".to_string() }, Token { ty: TK::END_LINE, val: ";".to_string() },
            Token { ty: TK::IDENT, val: "b".to_string() }, Token { ty: TK::OPE('+'), val: "+".to_string() },
            Token { ty: TK::IDENT, val: "a".to_string() }, Token { ty: TK::RETURN, val: "return".to_string() },
            Token { ty: TK::OPE('{'), val: "{".to_string() }, Token { ty: TK::OPE(')'), val: ")".to_string() },
            Token { ty: TK::IDENT, val: "b".to_string() }, Token { ty: TK::OPE(','), val: ",".to_string() },
            Token { ty: TK::IDENT, val: "a".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() }, Token { ty: TK::IDENT, val: "add".to_string() }
        ];

        let result = parse(&mut input.to_vec());

        let expect = [
            Node {
                ty: ND::FUNC,
                val: "add".to_string(),
                args: [
                    Node { ty: ND::IDENT, val: "a".to_string(), ..Default::default() },
                    Node { ty: ND::IDENT, val: "b".to_string(), ..Default::default() }
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

        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }

}
