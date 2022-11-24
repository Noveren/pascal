/// 解析结果类型
/// Result<(未解析部分, 解析结果), 未解析部分>
#[allow(unused)]
pub type PResult<'a, I, O> = Result<(I, O), (I, &'a str)>;

/// 解析器特征
pub trait Parser<'a, I, O> {
    fn parse(&self, input: I) -> PResult<'a, I, O>;
}

impl<'a, F, I, O> Parser<'a, I, O> for F
where
    F: Fn(I) -> PResult<'a, I, O>,
{
    fn parse(&self, input: I) -> PResult<'a, I, O> {
        return self(input);
    }
}