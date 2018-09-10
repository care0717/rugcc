mod node;
mod token;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut tokens = token::tokenize(s.chars().collect());
    let mut regs = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"].to_vec();
    let regno = 0;
    let mut ins = Vec::new();
    let node = node::expr(&mut tokens);
    node::gen_ir(node, &mut regs, &mut ins, regno);
    node::alloc_regs(&mut regs, &mut ins);
    print!(".intel_syntax noprefix\n");
    print!(".global _main\n");
    print!("_main:\n");
    node::gen_x86(regs, ins);
}