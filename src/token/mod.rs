#[cfg(test)]
mod test_token;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Undefined,
    EOF,
    WS,
    Plus,
    Integer(u32),
}

pub type Tokens = Vec<Token>;

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::Undefined => write!(f, "Undefined"),
            Token::WS => write!(f, " "),
            Token::EOF  => write!(f, "EOF"),
            Token::Plus => write!(f, "+"),
            Token::Integer(ref num) => write!(f, "Integer({})", *num),
        }
    }
}

impl Token {
    /// 对源码进行分词，返回 Token流，或发生词元解析错误，返回因错误未解析部分
    #[allow(unused)]
    pub fn split(mut src: &str) -> Result<Tokens, &str> {
        let mut tokens = Vec::<Token>::new();   // 词法解析结果
        loop {
            match src.chars().next() {
                Some(c) => {
                    if c.is_whitespace() {
                        let mut len: usize = 0;
                        for c in src.chars() {                    // 连续的空白符号将最终视为一个
                            if c.is_whitespace() {
                                len += c.len_utf8();
                            } else {
                                break;
                            }
                        }
                        src = &src[len..];
                        tokens.push(Token::WS);
                    } else if c == '+' {
                        src = &src['+'.len_utf8()..];
                        tokens.push(Token::Plus);
                        continue;
                    } else if c.is_digit(10) {
                        // 十进制多位数字解析 [0-9]+
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
    let mut len: usize = 0;
    for c in src.chars() {
        if c.is_digit(10) {
            len += c.len_utf8();
        } else {
            break;
        }
    }
    if len == 0 {
        return Err(());
    } else {
        // TODO 考虑数字大于 u32 所能容纳
        match (&src[0..len]).parse::<u32>() {
            Ok(v) => Ok((v, len)),
            Err(_) => Err(())
        }
    }
}