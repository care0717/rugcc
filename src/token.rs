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
        if c=='+' || c=='-' || c=='*' || c=='/' || c=='=' {
            tokens.push(Token{ty: TK::OPE(c), val: c.to_string()});
            counter += 1;
            continue;
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