// 2.2

mod default_env;
mod eval;
mod parse;
mod types;

use std::io;
use std::io::Write;
use std::process::exit;
use crate::default_env::gen_default_env;
use crate::eval::eval;
use crate::parse::parse;

fn main() {
    println!("{}", "LISP INIT");
    println!("{}", "Initializing repositories");
    println!("{}", "Accessing the mainframe");
    println!("{}", "Harvesting security keys");
    println!("{}", "COMPLETE");
    println!("{:?}", eval(&parse(String::from("(+ 1\n 2)")), &mut gen_default_env()));
    let mut env = gen_default_env();
    loop {
        print!("wrist> ");
        io::stdout().flush().unwrap();
        let mut expr = String::new();
        io::stdin().read_line(&mut expr).unwrap();
        if expr.to_lowercase() == "exit\n" {
            exit(0)
        } else {
            println!("{:?}", eval(&parse(expr), &mut env))
        }
    }
}