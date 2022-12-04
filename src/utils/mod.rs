/// 从字符串头开始匹配符合 op 的字符，返回字符的长度
#[allow(unused)]
pub fn catch<F>(src: &str, op: F) -> usize
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