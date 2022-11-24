mod token;
mod ast;

use token::Token;

pub fn pascal(input: &str) -> Result<u32, String> {
    match Token::split(input) {
        Ok(tokens) => match ast::expr_add(&tokens) {
            Ok((_, expr)) => Ok(expr.expr()),
            Err((_, msg)) => Err(msg.to_string()),
        },
        Err(msg) => {
            let mut m = "Failed to split until: ".to_string();
            m.push_str(msg);
            Err(m)
        },
    }
}