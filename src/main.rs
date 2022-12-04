use std::io::Write;

mod parser;
mod utils;
mod vm;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    let mut pascal = vm::new();
    loop {
        print!(">> ");
        std::io::stdout().flush()
            .expect("Can not flush the stdout");
        stdin.read_line(&mut input)
            .expect("Failed to read a line");
        if let Err(msg) = pascal.run(&input) {
            println!("{}", msg);
        }
        input.clear();
    }
}