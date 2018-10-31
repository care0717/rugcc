extern crate rugcc;
use self::rugcc::common::{IRType, Function};
use {REGS, REGS8};


static ARGREG: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

fn gen(func: Function, label: usize) {
    let ret = format!(".Lend{}", label);
    if func.name == "main" {
        print!(".global _{}\n", func.name);
        print!("_{}:\n", func.name);
    } else {
        print!(".global {}\n", func.name);
        print!("{}:\n", func.name);
    }
    print!("\tpush rbp\n");
    print!("\tmov rbp, rsp\n");
    print!("\tsub rsp, {}\n", func.stack_size);
    print!("\tpush r12\n");
    print!("\tpush r13\n");
    print!("\tpush r14\n");
    print!("\tpush r15\n");

    for ir in func.irs {
        match ir.op {
            IRType::IMM => {
                print!("\tmov {}, {}\n", REGS[ir.lhs], ir.rhs);
            }
            IRType::MOV => {
                print!("\tmov {}, {}\n", REGS[ir.lhs], REGS[ir.rhs]);
            },
            IRType::SUB_IMM => {
                print!("\tsub {}, {}\n", REGS[ir.lhs], ir.rhs);
            }
            IRType::RETURN => {
                print!("\tmov rax, {}\n", REGS[ir.lhs]);
                print!("\tjmp {}\n", ret);
            },
            IRType::Ope(o) => {
                match o {
                    '+' => print!("\tadd {}, {}\n", REGS[ir.lhs], REGS[ir.rhs]),
                    '-' => print!("\tsub {}, {}\n", REGS[ir.lhs], REGS[ir.rhs]),
                    '*' => {
                        print!("\tmov rax, {}\n", REGS[ir.rhs]);
                        print!("\tmul {}\n", REGS[ir.lhs]);
                        print!("\tmov {}, rax\n", REGS[ir.lhs]);
                    }
                    '/' => {
                        print!("\tmov rax, {}\n", REGS[ir.lhs]);
                        print!("\tcqo\n");
                        print!("\tdiv {}\n", REGS[ir.rhs]);
                        print!("\tmov {}, rax\n", REGS[ir.lhs]);
                    }
                    _ => assert!(false),
                }
            },
            IRType::CALL => {
                for i in 0..ir.args.clone().len(){
                    print!("\tmov {}, {}\n", ARGREG[i], REGS[ir.args[i]]);
                }
                print!("\tpush r10\n");
                print!("\tpush r11\n");
                print!("\tmov rax, 0\n");
                print!("\tcall {}\n", ir.name);
                print!("\tpop r11\n");
                print!("\tpop r10\n");
                print!("\tmov {}, rax\n", REGS[ir.lhs]);
            },
            IRType::SAVE_ARGS => {
                for i in 0..ir.lhs {
                    print!("\tmov [rbp-{}], {}\n", (i + 1) * 8, ARGREG[i]);
                }
            },
            IRType::LT => {
                print!("\tcmp {}, {}\n", REGS[ir.lhs], REGS[ir.rhs]);
                print!("\tsetl {}\n", REGS8[ir.lhs]);
                print!("\tmovzx {}, {}\n", REGS[ir.lhs], REGS8[ir.lhs]);
            }
            IRType::LABEL => print!(".L{}:\n", ir.lhs),
            IRType::UNLESS => {
                print!("\tcmp {}, 0\n", REGS[ir.lhs]);
                print!("\tje .L{}\n", ir.rhs);
            },
            IRType::JMP => {
                print!("\tjmp .L{}\n", ir.lhs);
            },
            IRType::LOAD => {
                print!("\tmov {}, [{}]\n", REGS[ir.lhs], REGS[ir.rhs]);
            },
            IRType::STORE => {
                print!("\tmov [{}], {}\n", REGS[ir.lhs], REGS[ir.rhs]);
            },
            IRType::NOP => {},
            IRType::KILL => assert!(false),
        }
    }

    print!("{}:\n", ret);
    print!("\tpop r15\n");
    print!("\tpop r14\n");
    print!("\tpop r13\n");
    print!("\tpop r12\n");
    print!("\tmov rsp, rbp\n");
    print!("\tpop rbp\n");
    print!("\tret\n");

}

pub fn gen_x86(fns: Vec<Function>){
    print!(".intel_syntax noprefix\n");
    let mut label = 0;
    for f in fns{
        gen(f, label);
        label += 1;
    }
}
