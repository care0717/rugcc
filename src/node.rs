extern crate rugcc;
use self::rugcc::common::{TK, Token, ND,  Node, Type, TY};

use std;

fn new_node(op: ND, lhs: Node, rhs: Node) -> Node {
    return Node{ op, lhs: Some(Box::new(lhs)), rhs: Some(Box::new(rhs)), ..Default::default()};
}

fn expect(ty: TK, tokens: &mut Vec<Token>) {
    let token = tokens.pop();
    if token.is_some() && token.unwrap().ty != ty {
        println!("{:?} expected, but not", ty);
        std::process::exit(1);
    }
    return
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

fn is_typename(tokens: &Vec<Token>) -> bool {
    return tokens[tokens.len()-1].ty==TK::INT
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
            return Node{ op: ND::NUM, val: token.val, ..Default::default()};
        },
        TK::IDENT => {
            let mut node = Node{ op: ND::IDENT, val: token.val, ..Default::default()};
            if !consume(TK::OPE('('), tokens) {
                return node
            }
            node.op = ND::CALL;
            if consume(TK::OPE(')'), tokens) {return node}
            node.args.push(assign(tokens));
            while consume(TK::OPE(','), tokens) {
                node.args.push(assign(tokens));
            }
            expect(TK::OPE(')'), tokens);
            return node
        },
        _ => {
            unreachable!("number expected, but got {}", token.val);
        },
    }
}

fn unary(tokens: &mut Vec<Token>) -> Node {
    if consume(TK::OPE('*'), tokens) {
        return Node{op: ND::DEREF, expr: Some(Box::new(mul(tokens))), ..Default::default()}
    } else if consume(TK::OPE('&'), tokens) {
        return Node{op: ND::ADDR, expr: Some(Box::new(mul(tokens))), ..Default::default()}
    } else {
        return term(tokens)
    }
}

fn mul(tokens: &mut Vec<Token>) -> Node {
    let mut lhs = unary(tokens);
    loop {
        let token = tokens.pop().unwrap();
        match token.ty {
            TK::OPE(o) => {
                match o {
                    '*' | '/' => lhs = new_node(ND::OPE(o), lhs, unary(tokens)),
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

fn assign(tokens: &mut Vec<Token>) -> Node {
    let lhs = logor(tokens);
    if consume(TK::OPE('='), tokens) {
        return new_node(ND::OPE('='), lhs, logor(tokens));
    } else {
        return lhs;
    }
}

fn get_type(tokens: &mut Vec<Token>) -> Type {
    let token = tokens.pop().unwrap();
    if token.ty != TK::INT {
        unreachable!("typename expected, but got {:?}", token.ty);
    }
    let mut ty = Type{..Default::default()};
    while consume(TK::OPE('*'), tokens) {
        ty = ty.ptr_of();
    }
    return ty
}

fn decl(tokens: &mut Vec<Token>) -> Node {
    // Read the first half of type name (e.g. `int *`).
    let mut node = Node { op: ND::VARDEF, ty: get_type(tokens), ..Default::default()};
    // Read an identifier.
    let token = tokens.pop().unwrap();
    if token.ty != TK::IDENT { unreachable!("variable name expected, but got {}", token.val)}
    node.val = token.val;
    // Read the second half of type name (e.g. `[3][5]`).
    let mut ary_size = Vec::new();
    while consume(TK::OPE('['), tokens) {
        let len = term(tokens);
        if len.op != ND::NUM {unreachable!("number expected")}
        ary_size.push(len);
        expect(TK::OPE(']'), tokens);
    }
    for len in ary_size {
        node.ty = node.ty.ary_of(len.val.parse().unwrap());
    }

    // Read an initializer.
    if consume(TK::OPE('='), tokens) {node.init = Some(Box::new(assign(tokens)));}
    expect(TK::END_LINE, tokens);

    return node
}

fn param(tokens: &mut Vec<Token>) -> Node {
    let mut node = Node { op: ND::VARDEF, ty: get_type(tokens), ..Default::default()};
    let token = tokens.pop().unwrap();
    if token.ty != TK::IDENT { unreachable!("parameter name expected, but got {}", token.val); }
    node.val = token.val;
    return node
}

fn expr_stmt(tokens: &mut Vec<Token>) -> Node {
    let node = Node { op: ND::EXPR_STMT, expr: Some(Box::new(assign(tokens))), ..Default::default()};
    expect(TK::END_LINE, tokens);
    return node;
}

fn stmt(tokens: &mut Vec<Token>) -> Node {
    let token = tokens.pop().unwrap();
    let mut node = Node { op: ND::EXPR_STMT, ..Default::default()};

    match token.ty {
        TK::INT => {
            tokens.push(token);
            decl(tokens)
        },
        TK::IF => {
            node.op = ND::IF;
            expect(TK::OPE('('), tokens);
            node.cond = Some(Box::new(assign(tokens)));
            expect(TK::OPE(')'), tokens);
            node.then = Some(Box::new(stmt(tokens)));
            if consume(TK::ELSE, tokens) {node.els = Some(Box::new(stmt(tokens)));}
            return node
        },
        TK::OPE('{') => {
            node.op = ND::COMP_STMT;
            while !consume(TK::OPE('}'), tokens) {
                node.stmts.push(stmt(tokens));
            }
            return node;
        },
        TK::FOR => {
            node.op = ND::FOR;
            expect(TK::OPE('('), tokens);
            if is_typename(tokens) {
                node.init = Some(Box::new(decl(tokens)));
            } else {
                node.init = Some(Box::new(expr_stmt(tokens)));
            }
            node.cond = Some(Box::new(assign(tokens)));
            expect(TK::END_LINE, tokens);
            node.inc = Some(Box::new(assign(tokens)));
            expect(TK::OPE(')'), tokens);
            node.body = Some(Box::new(stmt(tokens)));
            return node;
        },
        TK::RETURN => {
            node.op = ND::RETURN;
            node.expr = Some(Box::new(assign(tokens)));
            expect(TK::END_LINE, tokens);
            return node
        },
        _ => {
            tokens.push(token);
            return expr_stmt(tokens)
        }
    }
}

fn compound_stmt(tokens: &mut Vec<Token>) -> Node{
    let mut node = Node{ op: ND::COMP_STMT, ..Default::default()};
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
    // 関数宣言の最初のintは読み飛ばす
    let t = tokens.pop().unwrap();
    if t.ty != TK::INT {
        unreachable!("function return type expected, but got {}", t.val);
    }

    let token = tokens.pop().unwrap();
    if token.ty != TK::IDENT {unreachable!("function name expected, but got {}", token.val)}
    expect(TK::OPE('('), tokens);
    let mut node = Node{ op: ND::FUNC, val: token.val, ..Default::default()};
    if !consume(TK::OPE(')'), tokens) {
        node.args.push(param(tokens));
        while consume(TK::OPE(','), tokens){
            node.args.push(param(tokens));
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
            Token { ty: TK::OPE(')'), val: ")".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() },
            Token { ty: TK::IDENT, val: "main".to_string() }, Token { ty: TK::INT, val: "int".to_string() }
        ];
        let result = parse(&mut input.to_vec());

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
    fn can_parse_function() {
        let input = [
            Token { ty: TK::EOF, val: "EOF".to_string() }, Token { ty: TK::OPE('}'), val: "}".to_string() },
            Token { ty: TK::END_LINE, val: ";".to_string() }, Token { ty: TK::OPE(')'), val: ")".to_string() },
            Token { ty: TK::NUM, val: "2".to_string() }, Token { ty: TK::OPE(','), val: ",".to_string() },
            Token { ty: TK::NUM, val: "1".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() },
            Token { ty: TK::IDENT, val: "add".to_string() }, Token { ty: TK::RETURN, val: "return".to_string() },
            Token { ty: TK::OPE('{'), val: "{".to_string() }, Token { ty: TK::OPE(')'), val: ")".to_string() },
            Token { ty: TK::OPE('('), val: "(".to_string() }, Token { ty: TK::IDENT, val: "main".to_string() },
            Token { ty: TK::INT, val: "int".to_string() },
            Token { ty: TK::OPE('}'), val: "}".to_string() }, Token { ty: TK::END_LINE, val: ";".to_string() },
            Token { ty: TK::IDENT, val: "b".to_string() }, Token { ty: TK::OPE('+'), val: "+".to_string() },
            Token { ty: TK::IDENT, val: "a".to_string() }, Token { ty: TK::RETURN, val: "return".to_string() },
            Token { ty: TK::OPE('{'), val: "{".to_string() }, Token { ty: TK::OPE(')'), val: ")".to_string() },
            Token { ty: TK::IDENT, val: "b".to_string() }, Token { ty: TK::INT, val: "int".to_string() },
            Token { ty: TK::OPE(','), val: ",".to_string() }, Token { ty: TK::IDENT, val: "a".to_string() },
            Token { ty: TK::INT, val: "int".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() },
            Token { ty: TK::IDENT, val: "add".to_string() }, Token { ty: TK::INT, val: "int".to_string() }
        ];

        let result = parse(&mut input.to_vec());

        let expect = [
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

        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }

    # [test]
    fn can_parse_pointer() {
        let input = [
            Token { ty: TK::EOF, val: "EOF".to_string() }, Token { ty: TK::OPE('}'), val: "}".to_string() },
            Token { ty: TK::END_LINE, val: ";".to_string() }, Token { ty: TK::OPE(')'), val: ")".to_string() },
            Token { ty: TK::NUM, val: "1".to_string() }, Token { ty: TK::OPE('+'), val: "+".to_string() },
            Token { ty: TK::IDENT, val: "ary".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() },
            Token { ty: TK::OPE('*'), val: "*".to_string() }, Token { ty: TK::OPE('+'), val: "+".to_string() },
            Token { ty: TK::IDENT, val: "ary".to_string() }, Token { ty: TK::OPE('*'), val: "*".to_string() },
            Token { ty: TK::RETURN, val: "return".to_string() }, Token { ty: TK::END_LINE, val: ";".to_string() },
            Token { ty: TK::NUM, val: "7".to_string() }, Token { ty: TK::OPE('='), val: "=".to_string() },
            Token { ty: TK::OPE(')'), val: ")".to_string() }, Token { ty: TK::NUM, val: "1".to_string() },
            Token { ty: TK::OPE('+'), val: "+".to_string() }, Token { ty: TK::IDENT, val: "ary".to_string() },
            Token { ty: TK::OPE('('), val: "(".to_string() }, Token { ty: TK::OPE('*'), val: "*".to_string() },
            Token { ty: TK::END_LINE, val: ";".to_string() }, Token { ty: TK::NUM, val: "3".to_string() },
            Token { ty: TK::OPE('='), val: "=".to_string() }, Token { ty: TK::IDENT, val: "ary".to_string() },
            Token { ty: TK::OPE('*'), val: "*".to_string() }, Token { ty: TK::END_LINE, val: ";".to_string() },
            Token { ty: TK::OPE(']'), val: "]".to_string() }, Token { ty: TK::NUM, val: "2".to_string() },
            Token { ty: TK::OPE('['), val: "[".to_string() }, Token { ty: TK::IDENT, val: "ary".to_string() },
            Token { ty: TK::INT, val: "int".to_string() }, Token { ty: TK::OPE('{'), val: "{".to_string() },
            Token { ty: TK::OPE(')'), val: ")".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() },
            Token { ty: TK::IDENT, val: "main".to_string() }, Token { ty: TK::INT, val: "int".to_string() }
        ];

        let result = parse(&mut input.to_vec());
        
        let expect = [
            Node {
                op: ND::FUNC,
                val: "main".to_string(),
                body: Some(Box::new(Node {
                    op: ND::COMP_STMT,
                    stmts: [
                        Node {
                            op: ND::VARDEF,
                            ty: Type { ty: TY::ARY, ary_of: Some(Box::new(Type {..Default::default()})), len: 2, ..Default::default()},
                            val: "ary".to_string(), ..Default::default()
                        },
                        Node {
                            op: ND::EXPR_STMT,
                            expr: Some(Box::new(Node {
                                op: ND::OPE('='),
                                lhs: Some(Box::new(Node {
                                    op: ND::DEREF,
                                    expr: Some(Box::new(Node {
                                        op: ND::IDENT,
                                        val: "ary".to_string(), ..Default::default() })),
                                    ..Default::default() })),
                                rhs: Some(Box::new(Node {
                                    op: ND::NUM,
                                    val: "3".to_string(), ..Default::default() })), ..Default::default()})),
                            ..Default::default()},
                        Node {
                            op: ND::EXPR_STMT,
                            expr: Some(Box::new(Node {
                                op: ND::OPE('='),
                                lhs: Some(Box::new(Node {
                                    op: ND::DEREF,
                                    expr: Some(Box::new(Node {
                                        op: ND::OPE('+'),
                                        lhs: Some(Box::new(Node {
                                            op: ND::IDENT,
                                            val: "ary".to_string(), ..Default::default() })),
                                        rhs: Some(Box::new(Node {
                                            op: ND::NUM,
                                            val: "1".to_string(),
                                            ..Default::default()})),
                                        ..Default::default()})),
                                    ..Default::default()})),
                                rhs: Some(Box::new(Node { op: ND::NUM, val: "7".to_string(), ..Default::default() })),
                                ..Default::default()})),
                            ..Default::default()},
                        Node {
                            op: ND::RETURN,
                            expr: Some(Box::new(Node {
                                op: ND::OPE('+'),
                                lhs: Some(Box::new(Node {
                                    op: ND::DEREF,
                                    expr: Some(Box::new(Node {
                                        op: ND::IDENT,
                                        val: "ary".to_string(), ..Default::default() })),
                                    ..Default::default() })),
                                rhs: Some(Box::new(Node {
                                    op: ND::DEREF,
                                    expr: Some(Box::new(Node {
                                        op: ND::OPE('+'),
                                        lhs: Some(Box::new(Node {
                                            op: ND::IDENT,
                                            val: "ary".to_string(), ..Default::default()})),
                                        rhs: Some(Box::new(Node {
                                            op: ND::NUM,
                                            val: "1".to_string(), ..Default::default() })),
                                        ..Default::default() })),
                                    ..Default::default() })),
                                ..Default::default() })),
                            ..Default::default() }].to_vec(),
                    ..Default::default() })),
                ..Default::default() }
        ];

        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }

}
