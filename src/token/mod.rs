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
    pub fn split(mut src: &str) -> Result<Tokens, &str> {
        let mut tokens = Vec::<Token>::new();   // 词法解析结果
        loop {
            match src.chars().next() {
                Some(c) => {
                    if c == '+' {
                        tokens.push(Token::Plus);
                        src = &src['+'.len_utf8()..];
                        continue;
                    } else if c.is_digit(10) {
                        if let Ok((v, len)) = catch_u32_interger(src) {
                            src = &src[len..];
                            tokens.push(Token::Integer(v));
                            continue;
                        } else {
                            return Err(src);
                        }
                    } else {
                        src = &src[c.len_utf8()..];
                        tokens.push(Token::Undefined);
                    }
                }
                None => {
                    tokens.push(Token::EOF);
                    break;
                },
            }
        }
        return Ok(tokens);
    }
}

#[allow(unused)]
fn catch_u32_interger(src: &str) -> Result<(u32, usize), ()> {
    let mut stack = String::new();
    for c in src.chars() {
        if c.is_digit(10) {
            stack.push(c);
        } else {
            break;
        }
    }
    if stack.len() == 0 {
        return Err(());
    } else {
        // TODO 考虑数字大于 u32 所能容纳
        match stack.parse::<u32>() {
            Ok(v) => Ok((v, stack.len())),
            Err(_) => Err(())
        }
    }
}