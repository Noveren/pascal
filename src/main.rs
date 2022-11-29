use std::io::Write;

use ast::Parser;
use token::Token;

mod token;
mod ast;
mod utils;

/// 解释算术表达式
#[allow(unused)]
fn expr(e: &ast::Expr) -> Result<i32, &'static str> {
    use ast::Expr::*;
    use ast::BinOprator::*;
    match e {
        Number(n) => Ok(*n),
        BinOP(op, e1, e2) => {
            let v1 = expr(e1)?;
            let v2 = expr(e2)?;
            match op {
                Add => Ok(v1+v2),
                Sub => Ok(v1-v2),
                _ => Err("Undefined"),
            }
        }
    }
}


#[allow(unused)]
struct Interpreter {}

impl Interpreter {
    #[allow(unused)]
    fn new() -> Self { Interpreter {} }
    #[allow(unused)]
    fn run(src: &str) {
        match Token::split(src) {
            Err(msg)   => println!("Tokenize: {}", msg),
            Ok(tokens) => {
                println!("{:?}", tokens);
                match ast::pascal().parse(&tokens) {
                    Err(rest)  => println!("Lexer: {:?}", rest),
                    Ok((_, e)) => {
                        println!("{}", e.display());
                        match expr(&e) {
                            Ok(val) => println!("{}", val),
                            Err(err) => println!("{}", err),
                        }
                    },
                }
            }
        }

    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    loop {
        print!(">> ");
        std::io::stdout().flush().expect("Can not flush the stdout");
        stdin.read_line(&mut input).expect("Failed to read a line");
        Interpreter::run(&input);
        input.clear();
    }
}