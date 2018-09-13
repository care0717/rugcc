extern crate rugcc;
use self::rugcc::common::{TY, Token};

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
        if c=='+' || c=='-' || c=='*' || c=='/' {
            tokens.push(Token{ty: TY::Ope(c), val: c.to_string()});
            counter += 1;
            continue;
        }
        if c.is_digit(10){
            let mut tmp = String::new();
            while s[counter].is_digit(10){
                tmp += &s[counter].to_string();
                counter += 1;
            }
            tokens.push(Token{ty: TY::Num, val: tmp});
            continue;
        }
        print!("cannot tokenize: {}\n", c);
        process::exit(1);
    }
    tokens.push(Token{ty: TY::EOF, val: "EOF".to_string()});
    tokens.reverse();
    return tokens
}