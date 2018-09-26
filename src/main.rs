mod node;
mod token;
mod ir;
mod regalloc;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut tokens = token::tokenize(s.chars().collect());
    let mut regs = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"].to_vec();
    let node = node::stmt(&mut tokens);
    let mut ins = ir::gen_ir(node);
    regalloc::alloc_regs(&mut regs, &mut ins);
    print!(".intel_syntax noprefix\n");
    print!(".global _main\n");
    print!("_main:\n");
    regalloc::gen_x86(regs, ins);
}