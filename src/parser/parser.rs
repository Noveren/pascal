use super::nom::{Parser, Context, perr, pok};

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
    move |ctx: Context<'a>| {
        let len = catch(ctx.src, |c| c.is_whitespace());
        if len == 0 && NEC {
            perr(ctx, "No Whitespace".to_string())
        } else {
            pok(ctx.move_str(&ctx.src[0..len]), ())
        }
    }
}

/// 数字
#[allow(unused)]
pub fn number<'a, const RADIX: u32>() -> impl Parser<'a, String> {
    move |ctx: Context<'a>| {
        let len = catch(ctx.src, |c| c.is_digit(RADIX));
        if len > 0 {
            let num_str = &ctx.src[0..len];
            pok(ctx.move_str(num_str), num_str.to_string())
        } else {
            perr(ctx, "No Number".to_string())
        }
    }
}