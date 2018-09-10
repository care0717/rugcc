
pub mod common {
    use std;

    #[derive(PartialEq, Debug)]
    pub enum TY {
        Num,
        Ope(char),
        EOF,
    }
    pub struct Token {
        pub ty: TY,
        pub val: String,
    }

    pub fn error(mes: &str, val: Option<&String>) {
        match val {
            Some(v) => println!("{} {}",mes, v),
            None => println!("{}", mes),
        }
        std::process::exit(1);
    }
}
