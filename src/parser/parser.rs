use std::str::FromStr;
use super::nom::Parser;

/// 从字符串头开始匹配符合 op 的字符，返回字符的长度
#[allow(unused)]
fn catch<F>(src: &str, op: F) -> usize
where
    F: Fn(char) -> bool
{
    let mut len: usize = 0;
    for c in src.chars() {
        if !op(c) { break } 
        len += c.len_utf8();
    }
    return len;
}

/// 空白符
#[allow(unused)]
pub fn whitespace<'a, const NEC: bool>() -> impl Parser<'a, ()> {
    move |input: &'a str| {
        let len = catch(input, |c| c.is_whitespace());
        if len == 0 && NEC {
            Err("No Whitespace".to_string())
        } else {
            Ok((&input[len..], ()))
        }
    }
}

/// 数字
#[allow(unused)]
pub fn number<'a, T: FromStr, const RADIX: u32>() -> impl Parser<'a, T> {
    move |input: &'a str| {
        let len = catch(input, |c| c.is_digit(RADIX));
        if len > 0 {
            (&input[0..len]).parse::<T>().map_or_else(
                |_| Err(format!("Number {}", &input[0..len])),
                |v| Ok((&input[len..], v)),
            )
        } else {
            Err("No Number".to_string())
        }
    }
}