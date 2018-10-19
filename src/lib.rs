
pub mod common {
    use std;

    #[derive(PartialEq, Debug, Clone)]
    pub enum TK {
        NUM,
        OPE(char),
        EOF,
        RETURN,
        IDENT,
        END_LINE,
    }

    #[derive(Debug, Clone)]
    pub struct Token {
        pub ty: TK,
        pub val: String,
    }

    #[derive(PartialEq, Debug)]
    pub enum ND {
        NUM,
        OPE(char),
        IDENT,
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
        ALLOCA,
        LOAD,
        STORE,
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
        pub has_imm: bool,
        pub imm: usize,
    }

    pub fn error(mes: &str, val: Option<&String>) {
        match val {
            Some(v) => println!("{} {}",mes, v),
            None => println!("{}", mes),
        }
        std::process::exit(1);
    }
}
