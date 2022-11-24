use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    loop {
        print!(">>> ");
        if let Err(_) = std::io::stdout().flush() {
            println!("Failed to flush");
            break;
        }
        let mut input_line = String::new();
        match stdin.read_line(&mut input_line) {
            Ok(_) => {
                match pascal::pascal(input_line.trim_end_matches(['\r', '\n'])) {
                    Ok(v) => println!("{}", v),
                    Err(msg) => println!("{}", msg),
                }
            },
            Err(err) => {
                println!("{}", err);
                break;
            },
        }
    }

}