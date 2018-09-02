
struct Token {
    ty: TK,
    val: String,
}

#[derive(PartialEq)]
enum TK {
    Num,
    Ope(char),
    EOF,
}

fn tokenize(s: Vec<char>) -> Vec<Token>{
    let mut counter: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();
    let size = s.len();
    while counter < size {
        let c = s[counter];
        if c.is_whitespace() {
            counter += 1;
            continue;
        }
        if c=='+' || c=='-' {
            tokens.push(Token{ty: TK::Ope(c), val: c.to_string()});
            counter += 1;
            continue;
        }
        if c.is_digit(10){
            let mut tmp = String::new();
            while s[counter].is_digit(10){
                tmp += &s[counter].to_string();
                counter += 1;
            }
            tokens.push(Token{ty: TK::Num, val: tmp});
            continue;
        }
        print!("cannot tokenize: {}\n", c);
        std::process::exit(1);
    }
    tokens.push(Token{ty: TK::EOF, val: "EOF".to_string()});
    tokens.reverse();
    return tokens
}

fn fail(val: String) {
    println!("unexpected token: {}", val);
    std::process::exit(1);
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut tokens = tokenize(s.chars().collect());

    print!(".intel_syntax noprefix\n");
    print!(".global _main\n");
    print!("_main:\n");
    let mut token = tokens.pop().unwrap();
    if token.ty != TK::Num { fail(token.val) } else { print!("\tmov rax, {}\n", token.val); }
    if tokens.len() == 0 {
        print!("\tret\n");
        std::process::exit(0);
    }
    token = tokens.pop().unwrap();
    while token.ty != TK::EOF {
        match token.ty {
            TK::Ope(x) => {
                match x {
                    '+' => {
                        let num = tokens.pop().unwrap();
                        if num.ty != TK::Num {
                            fail(num.val)
                        } else {
                            print!("\tadd rax, {}\n", num.val);
                        }
                    },
                    '-' => {
                        let num = tokens.pop().unwrap();
                        if num.ty != TK::Num {
                            fail(num.val)
                        } else {
                            print!("\tsub rax, {}\n", num.val);
                        }
                    },
                    _ => fail(x.to_string()),
                }
            },
            TK::Num | TK::EOF => fail("aaaa".to_string()),
        }
        token = tokens.pop().unwrap();
    }

    print!("\tret\n");
}