use super::{Token, Token::*, catch_u32_interger};

#[test]
fn one_bit_addition() {
    assert_eq!(
        Token::split("1+3").unwrap(),
        vec![Integer(1), Plus, Integer(3), EOF],
    );
}

#[test]
fn whitespace() {
    assert_eq!(
        Token::split(" 1   +   3").unwrap(),
        vec![WS, Integer(1), WS, Plus, WS, Integer(3), EOF],
    )
}

#[test]
fn multi_digit_decimal_integer() {
    assert_eq!(
        Token::split("11+33").unwrap(),
        vec![Integer(11), Plus, Integer(33), EOF],
    );
    assert_eq!(
        Token::split("011+033").unwrap(),
        vec![Integer(11), Plus, Integer(33), EOF],
    );
}

#[test]
fn parse_interger() {
    assert_eq!(
        Ok((123, "123".len())),
        catch_u32_interger("123abc"),
    );
}