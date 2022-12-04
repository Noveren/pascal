/// 解析结果类型
/// Result<(未解析部分, 解析结果), 未解析部分>
#[allow(unused)]
pub type PResult<'a, O> = Result<(&'a str, O), &'a str>;

/// 解析器容器
#[allow(unused)]
pub struct ParserBox<'a, O> {
    parser: Box<dyn Parser<'a, O> + 'a>
}

impl<'a, O> ParserBox<'a, O> {
    #[allow(unused)]
    fn new<P: Parser<'a, O> + 'a>(p: P) -> Self {
        return ParserBox { parser: Box::new(p) }
    }
}

/// 解析器容器可以直接作为解析器使用
impl<'a, O> Parser<'a, O> for ParserBox<'a, O> {
    fn parse(&self, input: &'a str) -> PResult<'a, O> {
        return self.parser.parse(input);
    }
}

/// 泛型实现
/// 为一定形式的函数/闭包实现解析器特征
impl<'a, F, O> Parser<'a, O> for F
where
    F: Fn(&'a str) -> PResult<'a, O>
{
    fn parse(&self, input: &'a str) -> PResult<'a, O> {
        return self(input);
    }
}

/// 解析器特征
/// 接收字符串切片，对其进行解析，返回解析结果
pub trait Parser<'a, O>
{
    /// 消费者适配器：执行解析
    fn parse(&self, input: &'a str) -> PResult<'a, O>;

    /// 解析器适配器：将解析结果进行变换
    fn map<F: 'a, NewO: 'a>(self, op: F) -> ParserBox<'a, NewO>
    where
        Self: Sized + 'a,
        F: Fn(O) -> NewO,
    {
        return ParserBox::new(
            move |input| self.parse(input)
                .map(|(next, result)| (next, op(result)))
        );
    }

    /// 解析器适配器：判断解析结果是否符合要求, 不符合要求则视为解析错误
    fn pred<F: 'a>(self, op: F) -> ParserBox<'a, O>
    where
        Self: Sized + 'a,
        F: Fn(&O) -> bool,
    {
        return ParserBox::new(
            move |input| {
                if let Ok((next, item)) = self.parse(input) {
                    if op(&item) {
                        return Ok((next, item));
                    }
                }
                return Err(input);
            }
        );
    }

    /// 解析器适配器：根据上文解析结果, 使用 op 生成解析器，处理下文
    fn and_then<F: 'a, P: 'a, NewO: 'a>(self, op: F) -> ParserBox<'a, NewO>
    where
        Self: Sized + 'a,
        P: Parser<'a, NewO>,
        F: Fn(O) -> P,
    {
        return ParserBox::new(
            move |input| match self.parse(input) {
                Ok((next, result)) => op(result).parse(next),
                Err(err) => Err(err),
            }
        )
    }

    fn one_more(self) -> ParserBox<'a, Vec<O>>
    where
        Self: Sized + 'a,
    {
        return ParserBox::new(
            move |mut input| {
                let mut result = Vec::<O>::new();
                if let Ok((next, item)) = self.parse(input) {
                    input = next;
                    result.push(item);
                } else {
                    return Err(input);
                }
                while let Ok((next, item)) = self.parse(input) {
                    input = next;
                    result.push(item);
                }
                return Ok((input, result));
            }
        );
    }

    fn zero_more(self) -> ParserBox<'a, Vec<O>>
    where
        Self: Sized + 'a,
    {
        return ParserBox::new(
            move |mut input| {
                let mut result = Vec::<O>::new();
                while let Ok((next, item)) = self.parse(input) {
                    input = next;
                    result.push(item);
                }
                return Ok((input, result));
            }
        );
    }

}

/// 解析器适配器：组合器
#[allow(unused)]
pub fn pair<'a, P1, P2, R1, R2>(p1: P1, p2: P2)
    -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        p1.parse(input).and_then(|(next, r1)| {
            p2.parse(next)
                .map(|(next, r2)| (next, (r1, r2)))
        })
    }
}

/// 解析器适配器：偏左组合器
#[allow(unused)]
pub fn left<'a, P1: 'a, R1: 'a, P2: 'a, R2: 'a>(p1: P1, p2: P2)
    -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>
{
    pair(p1, p2).map(|(left, _)| left)
}

/// 解析器适配器：偏右组合器
#[allow(unused)]
pub fn right<'a, P1: 'a, R1: 'a, P2: 'a, R2: 'a>(p1: P1, p2: P2)
    -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    pair(p1, p2).map(|(_, right)| right)
}

/// 解析器适配器：先后二择器
#[allow(unused)]
pub fn either<'a, P1, P2, R>(p1: P1, p2: P2)
    -> impl Parser<'a, R>
where
    P1: Parser<'a, R>,
    P2: Parser<'a, R>,
{
    move |input| match p1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => p2.parse(input),
    }
}

/// 解析器：任意字符
#[allow(unused)]
pub fn any_char(input: &str) -> PResult<char> {
    match input.chars().next() {
        Some(c) => Ok((&input[c.len_utf8()..], c)),
        _ => Err(input),
    }
}

/// 解析器：指定字符
#[allow(unused)]
pub fn the_char<const C: char>(input: &str) -> PResult<char> {
    match input.chars().next() {
        Some(c) if c == C => Ok((&input[C.len_utf8()..], C)),
        _ => Err(input),
    }
}

/// 解析器：字面量
#[allow(unused)]
pub fn literal<'a>(expected: &'a str)
    -> impl Parser<'a, ()>
{
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(c) if c == expected =>
            Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}