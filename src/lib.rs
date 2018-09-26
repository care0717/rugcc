
pub mod common {
    use std;

    #[derive(PartialEq, Debug)]
    pub enum TK {
        NUM,
        OPE(char),
        EOF,
        RETURN,
        END_LINE,
    }

    #[derive(Debug)]
    pub struct Token {
        pub ty: TK,
        pub val: String,
    }

    #[derive(PartialEq, Debug)]
    pub enum ND {
        NUM,
        OPE(char),
        RETURN,
        COMP_STMT,
        EXPR_STMT,
    }

    #[derive(Debug)]
    pub struct Node {
        pub ty: ND,
        pub lhs: Option<Box<Node>>,
        pub rhs: Option<Box<Node>>,
        pub val: String,
        pub expr: Option<Box<Node>>,
        pub stmts: Vec<Node>
    }

    impl Node {
        pub fn get_ope(&self) -> char {
            match self.ty {
                ND::OPE(c) => return c,
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
