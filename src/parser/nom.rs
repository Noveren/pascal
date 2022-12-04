
/// 上下文 字符串切片、解析位置、行、列
#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Context<'a> {
    pub src: &'a str,
    pos: (usize, usize)
}

impl<'a> Context<'a> {
    #[allow(unused)]
    pub const fn new(src: &'a str) -> Self {
        Context { src, pos: (0, 0) }
    }
    #[allow(unused)]
    pub fn move_char(self, c: char) -> Self {
        Context {
            src: &self.src[c.len_utf8()..],
            pos: if c == '\n' { (self.pos.0 + 1, 0) } else { (self.pos.0, self.pos.1 + 1) }
        }
    }
    #[allow(unused)]
    pub fn move_str(self, s: &str) -> Self {
        Context {
            src: &self.src[s.len()..],
            pos: {
                let ss: Vec<&str> = s.rsplit('\n').collect();
                if ss.len() >  1{
                    (self.pos.0 + ss.len() - 1, ss.last().unwrap().len())
                } else {
                    (self.pos.0, self.pos.1 + s.len())
                }
            }
        }
    }
}


#[allow(unused)]
pub type PResult<'a, T> = Result<(Context<'a>, T), (Context<'a>, String)>;

#[allow(unused)]
pub const fn pok<'a, T>(ctx: Context<'a>, result: T) -> PResult<'a, T> {
    Ok((ctx, result))
}

#[allow(unused)]
pub const fn perr<'a, T>(ctx: Context<'a>, err: String) -> PResult<'a, T> {
    Err((ctx, err))
}

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
    fn parse(&self, ctx: Context<'a>) -> PResult<'a, T> {
        self.parser.parse(ctx)
    }
}

impl<'a, F, T> Parser<'a, T> for F
where
    F: Fn(Context<'a>) -> PResult<'a, T>,
{
    fn parse(&self, ctx: Context<'a>) -> PResult<'a, T> {
        self(ctx)
    }
}

pub trait Parser<'a, T> {
    fn parse(&self, ctx: Context<'a>) -> PResult<'a, T>;
    /// 不可反驳，对解析结果进行变换
    fn map<F: 'a, U: 'a>(self, op: F) -> ParserBox<'a, U>
    where
        Self: Sized + 'a,
        F: Fn(T) -> U,
    {
        ParserBox::new(
            move |ctx| self.parse(ctx)
                .map(|(ctx, result)| (ctx, op(result)))
        )
    }
    /// 可反驳，对解析结果进行断言
    fn assert<F: 'a>(self, op: F) -> ParserBox<'a, T>
    where
        Self: Sized + 'a,
        F: Fn(&T) -> bool,
    {
        ParserBox::new(
            move |ctx| self.parse(ctx)
                .and_then(|(ctx, result)| if op(&result) {
                    pok(ctx, result)
                } else {
                    perr(ctx, "Assert".to_string())
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
            move |ctx| match self.parse(ctx) {
                Err(err) => Err(err),
                Ok((next, result)) => {
                    match p.parse(next) {
                        Err(_) => pok(ctx, result),
                        Ok(_)  => perr(ctx, "Not Tail".to_string()),
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
            move |ctx| match self.parse(ctx) {
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
            move |mut ctx| {
                let mut result = Vec::<T>::new();
                if let Ok((next, item)) = self.parse(ctx) {
                    ctx = next;
                    result.push(item);
                } else {
                    return perr(ctx, "One More".to_string());
                }
                while let Ok((next, item)) = self.parse(ctx) {
                    ctx = next;
                    result.push(item)
                }
                pok(ctx, result)
            }
        )
    }
    /// zero_more
    fn m0(self) -> ParserBox<'a, Vec<T>>
    where
        Self: Sized + 'a,
    {
        ParserBox::new(
            move |mut ctx| {
                let mut result = Vec::<T>::new();
                while let Ok((next, item)) = self.parse(ctx) {
                    ctx = next;
                    result.push(item)
                }
                Ok((ctx, result))
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
    move |ctx| {
        p1.parse(ctx).and_then(|(next, r1)| {
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
    move |ctx| match p1.parse(ctx) {
        ok @ Ok(_) => ok,
        Err(_) => p2.parse(ctx),
    }
}

/// 解析器：任意字符
#[allow(unused)]
pub fn any_char(ctx: Context) -> PResult<char> {
    match ctx.src.chars().next() {
        Some(c) => pok(ctx.move_char(c), c),
        _ => perr(ctx, "any_char".to_string()),
    }
}

/// 解析器：指定字符
#[allow(unused)]
pub fn the_char<const C: char>(ctx: Context) -> PResult<char> {
    match ctx.src.chars().next() {
        Some(c) if c == C => pok(ctx.move_char(C), C),
        _ => perr(ctx, format!("the char {}", C)),
    }
}

/// 解析器：字面量
#[allow(unused)]
pub fn literal<'a>(expected: &'a str) -> impl Parser<'a, ()> {
    move |ctx: Context<'a>| match ctx.src.get(0..expected.len()) {
        Some(c) if c == expected =>
            pok(ctx.move_str(expected), ()),
        _ => perr(ctx, format!("Literal {}", expected)),
    }
}