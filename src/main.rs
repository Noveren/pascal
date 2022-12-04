mod parser;

fn main() {
    todo!("...");
}

// use std::io::Write;

// use ast::Parser;
// use token::Token;

// mod token;
// mod ast;
// mod utils;


// #[allow(unused)]
// struct Interpreter {}

// impl Interpreter {
//         /// 解释算术表达式
//         #[allow(unused)]
//         fn expr(e: &ast::Expr) -> Result<i32, &'static str> {
//             use ast::Expr::*;
//             use ast::BinOprator::*;
//             match e {
//                 Number(n) => Ok(*n),
//                 BinOP(op, e1, e2) => {
//                     let v1 = Interpreter::expr(e1)?;
//                     let v2 = Interpreter::expr(e2)?;
//                     match op {
//                         Add => Ok(v1+v2),
//                         Sub => Ok(v1-v2),
//                         _ => Err("Undefined"),
//                     }
//                 }
//             }
//         }
//     }

// fn main() {
//     let stdin = std::io::stdin();
//     let mut input = String::new();
//     let ast_parser = ast::pascal();
//     loop {
//         // 用户输入
//         print!(">> ");
//         std::io::stdout().flush().expect("Can not flush the stdout");
//         stdin.read_line(&mut input).expect("Failed to read a line");
//         if let Err(msg) = (|src: &str| -> Result<(), String> {
//             // 词法分析
//             let tokens = Token::split(src)
//                 .map_err(|msg| format!("Tokenize: {}", msg))?;
//             println!("{:?}", tokens);
//             // 语法分析
//             let (_, rst) = ast_parser.parse(&tokens)
//                 .map_err(|rest| format!("Lexer: {:?}", rest))?;
//             println!("{}", rst.display());
//             // 解释执行
//             let value = Interpreter::expr(&rst)
//                 .map_err(|err| err.to_string())?;
//             println!("{}", value);
//             return Ok(());
//         })(&input) {
//             println!("{}", msg);
//         }
//         input.clear();
//     }
// }