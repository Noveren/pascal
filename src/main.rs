use std::io::Write;
// use parser::Node;

mod parser;
mod utils;

struct Interpreter {}

impl Interpreter {
    fn exec(src: &str) -> Result<(), String> {
        // 解析源码，生成 AST
        let (_, ast) = parser::parse(&src)
            .map_err(|e| e.1)?;
        println!("{}", ast.display());
        // 执行 AST
        return Ok(());
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    loop {
        print!(">> ");
        std::io::stdout().flush().expect("Can not flush the stdout");
        stdin.read_line(&mut input).expect("Failed to read a line");
        if let Err(msg) = Interpreter::exec(&input) {
            println!("{}", msg);
        }
        input.clear();
    }
}