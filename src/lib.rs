pub mod common {
    #[derive(PartialEq, Debug, Clone)]
    pub enum TK {
        NUM,
        OPE(char),
        EOF,
        RETURN,
        IDENT,
        SIZEOF,
        INT,
        CHAR,
        IF,
        ELSE,
        LOGOR,
        LOGAND,
        FOR,
        END_LINE,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Token {
        pub ty: TK,
        pub val: String,
    }


    #[derive(PartialEq, Debug, Clone)]
    pub enum TY {
        INT,
        CHAR,
        PTR,
        ARY,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct Type {
        pub ty: TY,
        pub ptr_of: Option<Box<Type>>,
        pub ary_of: Option<Box<Type>>,
        pub len: usize,
    }
    impl Default for Type {
        fn default() -> Self {
            Type{ ty: TY::INT, ptr_of: None, ary_of: None, len: 0}
        }
    }
    impl Type {
        pub fn new_char() -> Type {
            return Type {ty: TY::CHAR, ..Default::default()}
        }
        pub fn size_of(&self) -> usize {
            match self.ty {
                TY::INT => return 4,
                TY::ARY => return self.ary_of.clone().unwrap().size_of() * self.len,
                TY::PTR => return 8,
                TY::CHAR => return 1,
            }
        }
        pub fn ary_of(&self, len: usize) -> Type {
            return Type{ty: TY::ARY, ary_of: Some(Box::new(self.clone())), len, ..Default::default()};
        }
        pub fn ptr_of(&self) -> Type {
            return Type{ty: TY::PTR, ptr_of: Some(Box::new(self.clone())), ..Default::default()};
        }
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum ND {
        NUM,
        IDENT,
        VARDEF,
        LVAR,
        DEREF,     // pointer dereference ("*")
        ADDR,
        SIZEOF,
        CALL,
        FUNC,
        FOR,
        OPE(char),
        IF,
        LOGOR,
        LOGAND,
        RETURN,
        COMP_STMT,
        EXPR_STMT,
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct Node {
        pub op: ND,
        pub ty: Type,
        pub lhs: Option<Box<Node>>,
        pub rhs: Option<Box<Node>>,
        pub val: String,
        pub expr: Option<Box<Node>>,
        pub stmts: Vec<Node>,
        // "if" ( cond ) then "else" els
        pub cond: Option<Box<Node>>,
        pub then: Option<Box<Node>>,
        pub els: Option<Box<Node>>,
        // "for" ( init; cond; inc ) body
        pub init: Option<Box<Node>>,
        pub inc: Option<Box<Node>>,
        pub args: Vec<Node>,
        pub body: Option<Box<Node>>,
        // Function definition
        pub stack_size: usize,
        // Local variable
        pub offset: usize,
    }
    impl Default for Node {
        fn default() -> Self {
            Self { op: ND::NUM, ty: Type{..Default::default()}, lhs: None, rhs: None, val: String::new(), expr: None,
                cond: None, then: None, els: None, init: None, inc: None, stmts: Vec::new(),
                args: Vec::new(), body: None, stack_size: 0, offset: 0}
        }
    }
    impl Node {
        pub fn get_ope(&self) -> char {
            match self.op {
                ND::OPE(c) => return c,
                _ => {
                    assert!(false);
                    return 'a'
                },
            }
        }
        pub fn addr_of(&self, ty: Type) -> Node {
            return Node{op: ND::ADDR, ty: ty.ptr_of(), expr: Some(Box::new(self.clone())), ..Default::default()}
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
        IMM,
        SUB_IMM,
        MOV,
        LABEL,
        UNLESS,
        LOAD8,
        LOAD32,
        LOAD64,
        STORE8,
        STORE32,
        STORE64,
        STORE8_ARG,
        STORE32_ARG,
        STORE64_ARG,
        RETURN,
        CALL,
        JMP,
        KILL,
        NOP,
        LT,
        ADD,
        SUB,
        MUL,
        DIV,
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
            for i in 0..IRINFO.len() {
                if IRINFO[i].op == self.op {
                    return IRINFO[i];
                }
            }
            unreachable!()
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
                IRInfoType::IMM => return format!("{} {}\n", info.name, self.lhs),
                IRInfoType::IMM_IMM =>return format!("{} {}, {}", info.name, self.lhs, self.rhs),
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
        IMM,
        IMM_IMM,
        JMP,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct IRInfo<'a> {
        pub op: IRType,
        pub name: &'a str,
        pub ty: IRInfoType,
    }

    const IRINFO: [IRInfo; 24] = [
        IRInfo{op: IRType::ADD, name: "ADD", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::SUB, name: "SUB", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::MUL, name: "MUL", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::DIV, name: "DIV", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::IMM, name: "MOV", ty: IRInfoType::REG_IMN},
        IRInfo{op: IRType::SUB_IMM, name: "SUB", ty: IRInfoType::REG_IMN},
        IRInfo{op: IRType::MOV, name: "MOV", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::LABEL, name: "", ty: IRInfoType::LABEL},
        IRInfo{op: IRType::UNLESS, name: "UNLESS", ty: IRInfoType::REG_LABEL},
        IRInfo{op: IRType::RETURN, name: "RET", ty: IRInfoType::REG},
        IRInfo{op: IRType::LOAD8, name: "LOAD8", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::LOAD32, name: "LOAD32", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::LOAD64, name: "LOAD64", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::STORE8, name: "STORE8", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::STORE32, name: "STORE32", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::STORE64, name: "STORE32", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::STORE8_ARG, name: "STORE8_ARG", ty: IRInfoType::IMM_IMM},
        IRInfo{op: IRType::STORE32_ARG, name: "STORE32_ARG", ty: IRInfoType::IMM_IMM},
        IRInfo{op: IRType::STORE64_ARG, name: "STORE64_ARG", ty: IRInfoType::IMM_IMM},
        IRInfo{op: IRType::LT, name: "LT", ty: IRInfoType::REG_REG},
        IRInfo{op: IRType::KILL, name: "KILL", ty: IRInfoType::NOARG},
        IRInfo{op: IRType::NOP, name: "NOP", ty: IRInfoType::NOARG},
        IRInfo{op: IRType::JMP, name: "JMP", ty: IRInfoType::JMP},
        IRInfo{op: IRType::CALL, name: "CALL", ty: IRInfoType::CALL},
    ];

    pub fn dump_ir(fns: &Vec<Function>) {
        for f in fns {
            eprintln!("{}():", f.clone().name);
            for ir in f.clone().irs {
                eprintln!("{}", ir.tostr());
            }
        }
    }
    pub fn dump_nodes(nodes: &Vec<Node>) {
        let space = "  ";
        for n in nodes {
            let strs: Vec<char> = format!("{:?}", n).chars().collect();
            let mut size = 0;
            let mut result = Vec::new();
            let mut temp = String::new();
            for s in strs {
                temp = [temp, s.to_string()].concat();
                if s == '{' {
                    let length =  temp.len();
                    if temp.find("None").is_none() && temp.find("[]").is_none()  {
                        result.push(temp);
                    }
                    temp = String::new();
                    size += 1;
                    for _i in 0..size {
                        temp = [temp, space.to_string()].concat();
                    }
                } else if  s == '}' {
                    size -= 1;
                } else if  s == ',' {
                    let length =  temp.len();
                    if temp.find("None").is_none() && temp.find("[]").is_none() {
                        result.push(temp);
                    }
                    temp = String::new();
                    for _i in 0..size {
                        temp = [temp, space.to_string()].concat();
                    }
                }
            }
            for r in result {
                eprintln!("{}", r);
            }
        }
    }
}

#[macro_export]
#[allow(unused)]
macro_rules! error {
        ( $m:expr, $t:ty ) => {
            eprintln!("{:?}", $m);
            std::process::exit(1);
            return <$t as Default>::default() as $t
        }
    }
