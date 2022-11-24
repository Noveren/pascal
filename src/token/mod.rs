#[cfg(test)]
mod test_token;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Undefined,
    EOF,
    Plus,
    Integer(u32),
}

pub type Tokens = Vec<Token>;

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Integer(ref num) => write!(f, "Integer({})", *num),
            Token::Plus => write!(f, "+"),
            Token::EOF  => write!(f, "EOF"),
            Token::Undefined => write!(f, "Undefined")
        }
    }
}

impl Token {
    #[allow(unused)]
    pub fn split(mut src: &str) -> Tokens {
        let mut tokens = Vec::<Token>::new();
        loop {
            match src.chars().next() {
                Some(c) => {
                    src = &src[c.len_utf8()..];
                    if c == '+' {
                        tokens.push(Token::Plus);
                        continue;
                    }
                    if c.is_digit(10) {
                        tokens.push(Token::Integer(c.to_digit(10).unwrap()));
                        continue;
                    }
                    tokens.push(Token::Undefined);
                }
                None => {
                    tokens.push(Token::EOF);
                    break;
                },
            }
        }
        return tokens;
    }
}