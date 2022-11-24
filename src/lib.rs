mod token;
mod ast;

use token::Token;

pub fn pascal(input: &str) -> Result<u32, &str> {
    match ast::expr_add(&Token::split(input)) {
        Ok((_, expr)) => Ok(expr.expr()),
        Err((_, msg)) => Err(msg.clone()),
    }
}