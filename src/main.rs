extern crate clap;
use clap::{App, Arg};
extern crate rugcc;
use self::rugcc::common::{dump_ir};
mod node;
mod token;
mod ir;
mod regalloc;
mod codegen;

const REGS: [&str; 7] = ["r10", "r11", "r9", "r12", "r13", "r14", "r15"];

fn main() {
    let app = App::new("rugcc")
        .version("0.0.1")
        .author("asai ")
        .about("Toy clang compiler")
        .arg(Arg::with_name("code")
            .help("enter code")
            .required(true)
        ).arg(Arg::with_name("dump-ir1")
            .help("dump ir vec before regalloc")
            .long("dump-ir1")
        ).arg(Arg::with_name("dump-ir2")
            .help("dump ir vec after regalloc")
            .long("dump-ir2")
        );
    let matches = app.get_matches();
    let input = matches.value_of("code").unwrap();
    let dump_ir1 = matches.is_present("dump-ir1");
    let dump_ir2 = matches.is_present("dump-ir2");

    let mut tokens = token::tokenize(input.chars().collect());
    //eprintln!("{:?}", tokens);
    let mut fns = ir::gen_ir(node::parse(&mut tokens));
    //eprintln!("{:?}", node);
    if dump_ir1 {dump_ir(&fns)}
    //eprintln!("{:?}", irs);
    regalloc::alloc_regs(&mut fns);
    if dump_ir2 {dump_ir(&fns)}
    codegen::gen_x86(fns);
}