mod node;
mod token;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut tokens = token::tokenize(s.chars().collect());
    let mut regs = ["rdi", "rsi", "r10", "r11", "r12", "r13", "r14", "r15"].to_vec();
    let node = node::expr(&mut tokens);
    print!(".intel_syntax noprefix\n");
    print!(".global _main\n");
    print!("_main:\n");
    print!("\tmov rax, {}\n", node::gen(node, &mut regs));
    print!("\tret\n");
}