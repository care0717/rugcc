
pub mod common {
    use std;

    #[derive(PartialEq, Debug, Clone)]
    pub enum TK {
        NUM,
        OPE(char),
        EOF,
        RETURN,
        IDENT,
        IF,
        ELSE,
        LOGOR,
        LOGAND,
        END_LINE,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Token {
        pub ty: TK,
        pub val: String,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum ND {
        NUM,
        CALL,
        FUNC,
        OPE(char),
        IDENT,
        IF,
        LOGOR,
        LOGAND,
        RETURN,
        COMP_STMT,
        EXPR_STMT,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Node {
        pub ty: ND,
        pub lhs: Option<Box<Node>>,
        pub rhs: Option<Box<Node>>,
        pub val: String,
        pub expr: Option<Box<Node>>,
        pub stmts: Vec<Node>,
        pub cond: Option<Box<Node>>,
        pub then: Option<Box<Node>>,
        pub els: Option<Box<Node>>,
        pub args: Vec<Node>,
        pub body: Option<Box<Node>>,
    }
    impl Default for Node {
        fn default() -> Self {
            Self { ty: ND::NUM, lhs: None, rhs: None, val: String::new(), expr: None, cond: None, then: None, els: None, stmts: Vec::new(), args: Vec::new(), body: None}
        }
    }
    impl Node {
        pub fn get_ope(&self) -> char {
            match self.ty {
                ND::OPE(c) => return c,
                _ => {
                    assert!(false);
                    return 'a'
                },
            }
        }
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Function {
        pub name: String,
        pub irs: Vec<IR>,
        pub stack_size: usize,
    }
    impl Default for Function {
        fn default() -> Self {
            Self { name: String::new(), irs: Vec::new(), stack_size: 0 }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum IRType {
        IMN,
        SUB_IMN,
        MOV,
        LABEL,
        UNLESS,
        LOAD,
        STORE,
        RETURN,
        CALL,
        SAVE_ARGS,
        JMP,
        KILL,
        NOP,
        LT,
        Ope(char),
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct IR {
        pub op: IRType,
        pub lhs: usize,
        pub rhs: usize,
        pub name: String,
        pub args: Vec<usize>,
    }
    impl Default for IR {
        fn default() -> Self {
            Self { op: IRType::NOP, lhs: 0, rhs: 0, name: String::new(), args: Vec::new() }
        }
    }
    impl IR {
        pub fn get_irinfo(&self) -> IRInfo {
            // イケてない for info in irinfo と書きたい
            for i in 0..IRINFO.len() {
                if IRINFO[i].op == self.op {
                    return IRINFO[i];
                }
            }
            // イケてない 通るはずのない無駄なreturnを書いている
            assert!(false);
            return IRInfo{op: IRType::NOP, name: "NOP", ty: IRInfoType::NOARG}

        }

        fn tostr(&self) -> String {
            let info = self.get_irinfo();
            match info.ty {
                IRInfoType::LABEL => return format!(".L{}:", self.lhs),
                IRInfoType::REG => return format!("{} r{}", info.name, self.lhs),
                IRInfoType::REG_REG => return format!("{} r{}, r{}", info.name, self.lhs, self.rhs),
                IRInfoType::REG_IMN => return format!("{} r{}, {}", info.name, self.lhs, self.rhs),
                IRInfoType::REG_LABEL => return format!("{} r{}, .L{}", info.name, self.lhs, self.rhs),
                IRInfoType::NOARG => return format!("{}", info.name),
                IRInfoType::CALL => return format!("r{} = {}(", self.lhs, self.name),
                IRInfoType::IMN => return format!("{} {}\n", info.name, self.lhs),
                IRInfoType::JMP => return format!("  {} .L{}", info.name, self.lhs),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum IRInfoType {
        NOARG,
        REG,
        LABEL,
        REG_REG,
        REG_IMN,
        REG_LABEL,
        CALL,
        IMN,
        JMP,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct IRInfo<'a> {
        pub op: IRType,
        pub name: &'a str,
        pub ty: IRInfoType,
    }

    const IRINFO: [IRInfo; 18] = [
        IRInfo{op: IRType::Ope('+'), name: "ADD", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::Ope('-'), name: "SUB", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::Ope('*'), name: "MUL", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::Ope('/'), name: "DIV", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::IMN, name: "MOV", ty: IRInfoType::REG_IMN},
        IRInfo{op: IRType::SUB_IMN, name: "SUB", ty: IRInfoType::REG_IMN},
        IRInfo{op: IRType::MOV, name: "MOV", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::LABEL, name: "", ty: IRInfoType::LABEL},
        IRInfo{op: IRType::UNLESS, name: "UNLESS", ty: IRInfoType::REG_LABEL},
        IRInfo{op: IRType::RETURN, name: "RET", ty: IRInfoType::REG},
        IRInfo{op: IRType::LOAD, name: "LOAD", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::STORE, name: "STORE", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::LT, name: "LT", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::KILL, name: "KILL", ty: IRInfoType::NOARG},
        IRInfo{op: IRType::NOP, name: "NOP", ty: IRInfoType::NOARG},
        IRInfo{op: IRType::JMP, name: "JMP", ty: IRInfoType::JMP},
        IRInfo{op: IRType::CALL, name: "CALL", ty: IRInfoType::CALL},
        IRInfo{op: IRType::SAVE_ARGS, name: "SAVE_ARGS", ty: IRInfoType::IMN},
    ];



    pub fn dump_ir(fns: &Vec<Function>) {
        for f in fns {
            eprintln!("{}():", f.clone().name);
            for ir in f.clone().irs {
                eprintln!("{}", ir.tostr());
            }
        }
    }


    pub fn error(mes: &str, val: Option<&String>) {
        match val {
            Some(v) => println!("{} {}",mes, v),
            None => println!("{}", mes),
        }
        std::process::exit(1);
    }
}
