extern crate rugcc;
use self::rugcc::common::{ND, IRType, Function};
use {REGS, REGS8, REGS32};


static ARGREG64: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
static ARGREG32: [&str; 6] = ["edi", "esi", "edx", "ecx", "r8d", "r9d"];
static ARGREG8: [&str; 6] = ["dil", "sil", "dl", "cl", "r8b", "r9b"];


fn gen(func: Function, label: usize) {
    println!(".data");
    for node in func.strings {
        if node.op == ND::STR {
            println!("{}:", node.val);
            println!("\t.asciz \"{}\"", node.str);
        } else {
            unreachable!("ND::STR expected but got: {:?}", node.op);
        }
    }


    let ret = format!(".Lend{}", label);
    println!(".text");
    println!(".global _{}", func.name);
    println!("_{}:", func.name);
    println!("\tpush rbp");
    println!("\tmov rbp, rsp");
    println!("\tsub rsp, {}", func.stack_size);
    println!("\tpush r12");
    println!("\tpush r13");
    println!("\tpush r14");
    println!("\tpush r15");

    for ir in func.irs {
        match ir.op {
            IRType::IMM => {
                println!("\tmov {}, {}", REGS[ir.lhs], ir.rhs);
            }
            IRType::MOV => {
                println!("\tmov {}, {}", REGS[ir.lhs], REGS[ir.rhs]);
            },
            IRType::SUB_IMM => {
                println!("\tsub {}, {}", REGS[ir.lhs], ir.rhs);
            }
            IRType::RETURN => {
                println!("\tmov rax, {}", REGS[ir.lhs]);
                println!("\tjmp {}", ret);
            },
            IRType::ADD => println!("\tadd {}, {}", REGS[ir.lhs], REGS[ir.rhs]),
            IRType::SUB => println!("\tsub {}, {}", REGS[ir.lhs], REGS[ir.rhs]),
            IRType::MUL => {
                println!("\tmov rax, {}", REGS[ir.rhs]);
                println!("\tmul {}", REGS[ir.lhs]);
                println!("\tmov {}, rax", REGS[ir.lhs]);
            },
            IRType::DIV => {
                println!("\tmov rax, {}", REGS[ir.lhs]);
                println!("\tcqo");
                println!("\tdiv {}", REGS[ir.rhs]);
                println!("\tmov {}, rax", REGS[ir.lhs]);
            },
            IRType::CALL => {
                for i in 0..ir.args.clone().len(){
                    println!("\tmov {}, {}", ARGREG64[i], REGS[ir.args[i]]);
                }
                println!("\tpush r10");
                println!("\tpush r11");
                println!("\tmov rax, 0");
                println!("\tcall _{}", ir.name);
                println!("\tpop r11");
                println!("\tpop r10");
                println!("\tmov {}, rax", REGS[ir.lhs]);
            },
            IRType::STORE8_ARG => {
                println!("\tmov [rbp-{}], {}", ir.lhs, ARGREG8[ir.rhs]);
            },
            IRType::STORE32_ARG => {
                println!("\tmov [rbp-{}], {}", ir.lhs, ARGREG32[ir.rhs]);
            },
            IRType::STORE64_ARG => {
                println!("\tmov [rbp-{}], {}", ir.lhs, ARGREG64[ir.rhs]);
            },
            IRType::LT => {
                println!("\tcmp {}, {}", REGS[ir.lhs], REGS[ir.rhs]);
                println!("\tsetl {}", REGS8[ir.lhs]);
                println!("\tmovzx {}, {}", REGS[ir.lhs], REGS8[ir.lhs]);
            }
            IRType::LABEL => println!(".L{}:", ir.lhs),
            IRType::LABEL_ADDR => println!("\tlea {}, [rip + {}]", REGS[ir.lhs], ir.name),
            IRType::UNLESS => {
                println!("\tcmp {}, 0", REGS[ir.lhs]);
                println!("\tje .L{}", ir.rhs);
            },
            IRType::JMP => {
                println!("\tjmp .L{}", ir.lhs);
            },
            IRType::LOAD8 => {
                println!("\tmov {}, [{}]", REGS8[ir.lhs], REGS[ir.rhs]);
                println!("\tmovzx {}, {}", REGS[ir.lhs], REGS8[ir.lhs]);
            },
            IRType::LOAD32 => {
                println!("\tmov {}, [{}]", REGS32[ir.lhs], REGS[ir.rhs]);
            },
            IRType::LOAD64 => {
                println!("\tmov {}, [{}]", REGS[ir.lhs], REGS[ir.rhs]);
            },
            IRType::STORE8 => {
                println!("\tmov [{}], {}", REGS[ir.lhs], REGS8[ir.rhs]);
            },
            IRType::STORE32 => {
                println!("\tmov [{}], {}", REGS[ir.lhs], REGS32[ir.rhs]);
            },
            IRType::STORE64 => {
                println!("\tmov [{}], {}", REGS[ir.lhs], REGS[ir.rhs]);
            },
            IRType::NOP => {},
            IRType::KILL => unreachable!("unexpected IRType KILL"),
        }
    }

    println!("{}:", ret);
    println!("\tpop r15");
    println!("\tpop r14");
    println!("\tpop r13");
    println!("\tpop r12");
    println!("\tmov rsp, rbp");
    println!("\tpop rbp");
    println!("\tret");

}

pub fn gen_x86(fns: Vec<Function>){
    println!(".intel_syntax noprefix");
    let mut label = 0;
    for f in fns{
        gen(f, label);
        label += 1;
    }
}
