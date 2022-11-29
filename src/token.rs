use crate::utils;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Plus,
    Minus,
    Number(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match *self {
            Plus         => write!(f, "+"),
            Minus        => write!(f, "-"),
            Number(ref num) => write!(f, "Number({})", num),
        }
    }
}

macro_rules! single_char_token_continue {
    ($token:expr, $ch:expr, $c:expr, $src:expr, $tokens:expr) => {
        if $c == $ch {
            ($tokens).push($token);
            ($src) = &($src)[($ch).len_utf8()..];
            continue;
        }
    };
}

impl Token {
    /// 对源码进行分词，返回 Token流，或发生词元划分错误，返回剩余部分
    #[allow(unused)]
    pub fn split<'a>(mut src: &'a str) -> Result<Vec<Token>, &'a str> {
        use Token::*;
        let mut tokens = Vec::<Token>::new();                                            // 词法解析结果
        return loop { match src.chars().next() {
            Some(c) => {
                // 过滤所有空白符
                if c.is_whitespace() {
                    src = &src[utils::catch(src, |ch| ch.is_whitespace())..];
                    continue;
                }
                single_char_token_continue!(Plus,  '+', c, src, tokens);
                single_char_token_continue!(Minus, '-', c, src, tokens);
                // 十进制数字
                if c.is_digit(10) {
                    let len = utils::catch(src, |ch| ch.is_digit(10));                   // 必然大于等于 1
                    tokens.push(Number(src[0..len].to_string()));
                    src = &src[len..];
                    continue;
                }
                break Err(src)

            },
            None => break Ok(tokens)
        }}
    }
}

#[cfg(test)]
mod test_token {
    use super::{Token, Token::*};
    #[test]
    fn one_bit_addition() {
        assert_eq!(
            Token::split("1+3").unwrap(),
            vec![Number("1".to_string()), Plus, Number("3".to_string())],
        );
    }
    
    #[test]
    fn whitespace() {
        assert_eq!(
            Token::split(" 1   +   3").unwrap(),
            vec![Number("1".to_string()), Plus, Number("3".to_string())],
        )
    }
    
    #[test]
    fn multi_digit_decimal_integer() {
        assert_eq!(
            Token::split("11+33").unwrap(),
            vec![Number("11".to_string()), Plus, Number("33".to_string())],
        );
        assert_eq!(
            Token::split("011+033").unwrap(),
            vec![Number("011".to_string()), Plus, Number("033".to_string())],
        );
    }
}
