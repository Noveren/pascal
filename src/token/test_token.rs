use super::{Token, Token::*};

#[test]
fn add_2_expr() {
    assert_eq!(
        Token::split("1+3"),
        vec![Integer(1), Plus, Integer(3), EOF],
    );
}