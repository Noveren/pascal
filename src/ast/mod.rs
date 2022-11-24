mod parser;

pub use crate::token::Token;
#[allow(unused_imports)]
use parser::{PResult, Parser};

#[derive(Debug, PartialEq, Eq)]
pub struct ExprAdd {
    operand: (u32, u32),
}

impl ExprAdd {
    #[allow(unused)]
    pub fn expr(&self) -> u32 {
        self.operand.0 + self.operand.1
    }
}

#[allow(unused)]
pub fn expr_add<'a, 'b>(tokens: &'a [Token]) -> PResult<'b, &'a [Token], ExprAdd> {
    println!("{:?}", tokens);
    if tokens.len() < 4 {
        return Err((tokens, "too short to parser"));
    }
    if let &[Token::Integer(v0), Token::Plus, Token::Integer(v1)] = &tokens[0..3] {
        Ok((&tokens[3..], ExprAdd {operand: (v0, v1)}))
    } else {
        Err((tokens, "failed to match"))
    }
}

#[test]
fn test_expr_add() {
    use Token::*;
    assert_eq!(
        Ok((&vec![EOF][..], ExprAdd {operand: (1, 3)})),
        expr_add.parse(&Token::split("1+3").unwrap()),
    );
    assert_eq!(
        Ok((&vec![EOF][..], ExprAdd {operand: (11, 3)})),
        expr_add.parse(&Token::split("11+3").unwrap()),
    );
    assert_eq!(
        Err((&vec![Integer(1), Plus, EOF][..], "too short to parser")),
        expr_add.parse(&Token::split("1+").unwrap()),
    );
}