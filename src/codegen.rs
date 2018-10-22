extern crate rugcc;
use self::rugcc::common::{IR, IRType};

pub fn gen_x86(regs: Vec<&str>, ins: Vec<IR>) {
    let ret = ".Lend";
    print!("\tpush rbp\n");
    print!("\tmov rbp, rsp\n");

    for ir in ins {
        match ir.op {
            IRType::IMN => {
                print!("\tmov {}, {}\n", regs[ir.lhs], ir.rhs);
            },
            IRType::MOV => {
                print!("\tmov {}, {}\n", regs[ir.lhs], regs[ir.rhs]);
            },
            IRType::ADD_IMN => {
                print!("\tadd {}, {}\n", regs[ir.lhs], ir.rhs);
            },
            IRType::RETURN => {
                print!("\tmov rax, {}\n", regs[ir.lhs]);
                print!("\tjmp {}\n", ret);
            },
            IRType::Ope(o) => {
                match o {
                    '+' => print!("\tadd {}, {}\n", regs[ir.lhs], regs[ir.rhs]),
                    '-' => print!("\tsub {}, {}\n", regs[ir.lhs], regs[ir.rhs]),
                    '*' => {
                        print!("\tmov rax, {}\n", regs[ir.rhs]);
                        print!("\tmul {}\n", regs[ir.lhs]);
                        print!("\tmov {}, rax\n", regs[ir.lhs]);
                    }
                    '/' => {
                        print!("\tmov rax, {}\n", regs[ir.lhs]);
                        print!("\tcqo\n");
                        print!("\tdiv {}\n", regs[ir.rhs]);
                        print!("\tmov {}, rax\n", regs[ir.lhs]);
                    }
                    _ => assert!(true),
                }
            },
            IRType::ALLOCA => {
                if ir.rhs != 0 {
                    print!("\tsub rsp, {}\n", ir.rhs);
                    print!("\tmov {}, rsp\n", regs[ir.lhs]);
                }
            },
            IRType::LOAD => {
                print!("\tmov {}, [{}]\n", regs[ir.lhs], regs[ir.rhs]);
            },
            IRType::STORE => {
                print!("\tmov [{}], {}\n", regs[ir.lhs], regs[ir.rhs]);
            },
            IRType::NOP => {},
            IRType::KILL => assert!(true),
        }
    }

    print!("{}:\n", ret);
    print!("\tmov rsp, rbp\n");
    print!("\tmov rsp, rbp\n");
    print!("\tpop rbp\n");
    print!("\tret\n");

}
