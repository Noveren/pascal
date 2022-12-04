#[allow(unused)]
pub type PResult<'a, T> = Result<(&'a str, T), String>;

#[allow(unused)]
pub struct ParserBox<'a, T> {
    parser: Box<dyn Parser<'a, T> + 'a>,
}

impl<'a, T> ParserBox<'a, T> {
    #[allow(unused)]
    fn new<P: Parser<'a, T> + 'a>(p: P) -> Self {
        ParserBox { parser: Box::new(p) }
    }
}

impl<'a, T> Parser<'a, T> for ParserBox<'a, T> {
    fn parse(&self, input: &'a str) -> PResult<'a, T> {
        self.parser.parse(input)
    }
}

impl<'a, F, T> Parser<'a, T> for F
where
    F: Fn(&'a str) -> PResult<'a, T>,
{
    fn parse(&self, input: &'a str) -> PResult<'a, T> {
        self(input)
    }
}

pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> PResult<'a, T>;
    /// 不可反驳，对解析结果进行变换
    fn map<F: 'a, U: 'a>(self, op: F) -> ParserBox<'a, U>
    where
        Self: Sized + 'a,
        F: Fn(T) -> U,
    {
        ParserBox::new(
            move |input| self.parse(input)
                .map(|(next, result)| (next, op(result)))
        )
    }
    /// 可反驳，对解析结果进行断言
    fn assert<F: 'a>(self, op: F) -> ParserBox<'a, T>
    where
        Self: Sized + 'a,
        F: Fn(&T) -> bool,
    {
        ParserBox::new(
            move |input| self.parse(input)
                .and_then(|(next, result)| if op(&result) {
                    Ok((next, result))
                } else {
                    Err("Assert".to_string())
                })
        )
    }
    /// 不尾随
    fn not_tail<P: 'a, U: 'a>(self, p: P) -> ParserBox<'a, T>
    where
        Self: Sized + 'a,
        P: Parser<'a, U>,
    {
        ParserBox::new(
            move |input| match self.parse(input) {
                Err(err) => Err(err),
                Ok((next, result)) => {
                    match p.parse(next) {
                        Err(_) => Ok((next, result)),
                        Ok(_)  => Err("Not Tail".to_string()),
                    }
                }
            }

        )
    }
    /// 根据上文解析结果，使用 op 生成解析器，处理下文
    fn and_then<F: 'a, P: 'a, U: 'a>(self, op: F) -> ParserBox<'a, U>
    where
        Self: Sized + 'a,
        P: Parser<'a, U>,
        F: Fn(T) -> P,
    {
        ParserBox::new(
            move |input| match self.parse(input) {
                Err(err) => Err(err),
                Ok((next, result)) => op(result).parse(next),
            }
        )
    }
    /// 可反驳 one_more
    fn m1(self) -> ParserBox<'a, Vec<T>>
    where
        Self: Sized + 'a,
    {
        ParserBox::new(
            move |mut input| {
                let mut result = Vec::<T>::new();
                if let Ok((next, item)) = self.parse(input) {
                    input = next;
                    result.push(item);
                } else {
                    return Err("one_more".to_string());
                }
                while let Ok((next, item)) = self.parse(input) {
                    input = next;
                    result.push(item)
                }
                Ok((input, result))
            }
        )
    }
    /// zero_more
    fn m0(self) -> ParserBox<'a, Vec<T>>
    where
        Self: Sized + 'a,
    {
        ParserBox::new(
            move |mut input| {
                let mut result = Vec::<T>::new();
                while let Ok((next, item)) = self.parse(input) {
                    input = next;
                    result.push(item)
                }
                Ok((input, result))
            }
        )
    }
}

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
        _ => Err("any_char".to_string()),
    }
}

/// 解析器：指定字符
#[allow(unused)]
pub fn the_char<const C: char>(input: &str) -> PResult<char> {
    match input.chars().next() {
        Some(c) if c == C => Ok((&input[C.len_utf8()..], C)),
        _ => Err(format!("the char {}", C)),
    }
}

/// 解析器：字面量
#[allow(unused)]
pub fn literal<'a>(expected: &'a str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(c) if c == expected =>
            Ok((&input[expected.len()..], ())),
        _ => Err(format!("literal {}", expected)),
    }
}