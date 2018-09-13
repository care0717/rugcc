
pub mod common {
    use std;

    #[derive(PartialEq, Debug)]
    pub enum TY {
        Num,
        Ope(char),
        EOF,
    }
    pub struct Token {
        pub ty: TY,
        pub val: String,
    }


    #[derive(Debug)]
    pub struct Node {
        pub ty: TY,
        pub lhs: Option<Box<Node>>,
        pub rhs: Option<Box<Node>>,
        pub val: String,
    }

    impl Node {
        pub fn get_ope(&self) -> char {
            match self.ty {
                TY::Ope(c) => return c,
                _ => {
                    assert!(true);
                    return 'a'
                },
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum IRType {
        IMN,
        MOV,
        RETURN,
        KILL,
        NOP,
        Ope(char),
    }

    #[derive(Clone, Debug)]
    pub struct IR {
        pub op: IRType,
        pub lhs: usize,
        pub rhs: usize,
    }

    pub fn error(mes: &str, val: Option<&String>) {
        match val {
            Some(v) => println!("{} {}",mes, v),
            None => println!("{}", mes),
        }
        std::process::exit(1);
    }
}
