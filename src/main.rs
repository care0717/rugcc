extern crate clap;
use clap::{App, Arg};
extern crate rugcc;
use self::rugcc::common::{dump_ir};
mod node;
mod token;
mod ir;
mod sema;
mod regalloc;
mod codegen;

const REGS: [&str; 8] = ["rbp", "r10", "r11", "r9", "r12", "r13", "r14", "r15"];
const REGS8: [&str; 8] = ["bpl", "r10b", "r11b", "bl", "r12b", "r13b", "r14b", "r15b"];


fn main() {
    let app = App::new("rugcc")
        .version("0.0.1")
        .author("care0717")
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

    let mut nodes =  node::parse(&mut tokens);
    let mut sema = sema::SemaGenerator::new();
    nodes = sema.sema(nodes);
    //eprintln!("{:?}", nodes);
    let mut fns = ir::IrGenerator::new().gen_ir(nodes);
    //eprintln!("{:?}", fns);

    if dump_ir1 {dump_ir(&fns)}
    //eprintln!("{:?}", irs);
    regalloc::alloc_regs(&mut fns);
    if dump_ir2 {dump_ir(&fns)}
    codegen::gen_x86(fns);
}
