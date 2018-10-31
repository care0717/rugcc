extern crate rugcc;
use self::rugcc::common::{TK, Token};

use std::process;

pub fn tokenize(s: Vec<char>) -> Vec<Token>{
    let mut counter: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();
    let size = s.len();
    while counter < size {
        let c = s[counter];
        if c.is_whitespace() {
            counter += 1;
            continue;
        }
        let opes: Vec<char> = "+-*/=(),{}&|<>".chars().collect();
        if opes.contains(&c) {
            if c=='&' {
                counter += 1;
                if s[counter] == '&' {
                    tokens.push(Token{ty: TK::LOGAND, val: "&&".to_string()});
                    counter += 1;
                    continue;
                } else {
                    print!("cannot tokenize: {}\n", c);
                    process::exit(1);
                }
            } else if c=='|' {
                counter += 1;
                if s[counter] == '|' {
                    tokens.push(Token{ty: TK::LOGOR, val: "||".to_string()});
                    counter += 1;
                    continue;
                } else {
                    print!("cannot tokenize: {}\n", c);
                    process::exit(1);
                }
            } else {
                tokens.push(Token{ty: TK::OPE(c), val: c.to_string()});
                counter += 1;
                continue;
            }
        }
        if c==';' {
            tokens.push(Token{ty: TK::END_LINE, val: c.to_string()});
            counter += 1;
            continue;
        }
        if c.is_alphabetic() || c=='_' {
            let mut name = Vec::new();
            name.push(c);
            counter += 1;
            while s[counter].is_alphabetic() || s[counter].is_digit(10) || s[counter] == '_' {
                name.push(s[counter]);
                counter += 1;
            }
            match  name.iter().collect::<String>().as_str()  {
                "return" => tokens.push(Token{ty: TK::RETURN, val: name.iter().collect()}),
                "if" => tokens.push(Token{ty: TK::IF, val: name.iter().collect()}),
                "else" => tokens.push(Token{ty: TK::ELSE, val: name.iter().collect()}),
                _ => tokens.push(Token{ty: TK::IDENT, val: name.iter().collect()}),
            }
            continue;
        }
        if c.is_digit(10){
            let mut tmp = String::new();
            while s[counter].is_digit(10){
                tmp += &s[counter].to_string();
                counter += 1;
            }
            tokens.push(Token{ty: TK::NUM, val: tmp});
            continue;
        }
        print!("cannot tokenize: {}\n", c);
        process::exit(1);
    }
    tokens.push(Token{ty: TK::EOF, val: "EOF".to_string()});
    tokens.reverse();
    return tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    # [test]
    fn can_tokenize_arithmetic_expr() {
        let input = "main() { return (2+2*3)/2-1;}".chars().collect();

        let result = tokenize(input);
        let expect = [
            Token { ty: TK::EOF, val: "EOF".to_string().to_string() }, Token { ty: TK::OPE('}'), val: "}".to_string() },
            Token { ty: TK::END_LINE, val: ";".to_string() }, Token { ty: TK::NUM, val: "1".to_string() },
            Token { ty: TK::OPE('-'), val: "-".to_string() }, Token { ty: TK::NUM, val: "2".to_string() },
            Token { ty: TK::OPE('/'), val: "/".to_string() }, Token { ty: TK::OPE(')'), val: ")".to_string() },
            Token { ty: TK::NUM, val: "3".to_string() }, Token { ty: TK::OPE('*'), val: "*".to_string() },
            Token { ty: TK::NUM, val: "2".to_string() }, Token { ty: TK::OPE('+'), val: "+".to_string() },
            Token { ty: TK::NUM, val: "2".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() },
            Token { ty: TK::RETURN, val: "return".to_string() }, Token { ty: TK::OPE('{'), val: "{".to_string() },
            Token { ty: TK::OPE(')'), val: ")".to_string() }, Token { ty: TK::OPE('('), val: "(".to_string() }, Token { ty: TK::IDENT, val: "main".to_string() }
        ];
        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }

    # [test]
    fn can_tokenize_function() {
        let input = "add(a,b) {return a+b;} main() { return add(1,2); }".chars().collect();

        let result = tokenize(input);

        let expect = [
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

        assert_eq!(result.len(), expect.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expect[i]);
        }
    }
}
