use crate::token::{Token};

// ============================================================================

pub type Stream<'a> = &'a [Token];
pub type PResult<'a, O> = Result<(Stream<'a>, O), Stream<'a>>;

trait Parser<'a, O> {
    fn parse(&self, input: Stream<'a>) -> PResult<'a, O>;
}

impl<'a, O, F> Parser<'a, O> for F
where
    F: Fn(Stream<'a>) -> PResult<'a, O>,
{
    fn parse(&self, input: Stream<'a>) -> PResult<'a, O> {
        self(input)
    }
}

#[allow(unused)]
fn two<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| p1.parse(input)
        .and_then(|(rest, r1)| {
            p2.parse(rest)
                .map(|(rest, r2)| (rest, (r1, r2)))
        })
}

#[allow(unused)]
fn either<'a, P1, P2, R>(p1: P1, p2: P2) -> impl Parser<'a, R>
where
    P1: Parser<'a, R>,
    P2: Parser<'a, R>,
{
    move |input| match p1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => p2.parse(input),
    }
}


// ============================================================================

#[allow(unused)]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
enum BinOprator {
    Plus,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Number(i32),
    BinOP(BinOprator, Box<Expr>, Box<Expr>),
}

/// 解析器：解析 Token::Number(String)
#[allow(unused)]
fn number<'a>(input: Stream<'a>) -> PResult<'a, Expr> {
    if let Some((Token::Number(num_str), rest)) = input.split_first() {
        let num = num_str.parse::<i32>().map_err(|_| input)?;
        Ok((rest, Expr::Number(num)))
    } else {
        Err(input)
    }
}

#[allow(unused)]
fn symbol<'a>(expected: Token) -> impl Parser<'a, ()> {
    move |input: &'a [Token]| match input.split_first() {
        Some((token, rest)) if *token == expected => Ok((rest, ())),
        _ => Err(input),
    }
}

#[allow(unused)]
fn expr<'a>() -> impl Parser<'a, Expr> {
    either(
        number,
    )
}


#[test]
fn parser_number() {
    assert_eq!(
        Ok((&vec![][..], Expr::Number(32))),
        number.parse(&vec![Token::Number("0032".to_string())]),
    )
}

