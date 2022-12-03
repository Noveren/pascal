use std::fmt::Display;

use crate::token::{Token};

// ============================================================================

pub type PResult<'a, O> = Result<(&'a [Token], O), &'a [Token]>;

pub trait Parser<O> {
    fn parse<'a>(&self, input: &'a [Token]) -> PResult<'a, O>;
}

impl<O, F> Parser<O> for F
where
    F: Fn(&[Token]) -> PResult<O>,
{
    fn parse<'a>(&self, input: &'a [Token]) -> PResult<'a, O> {
        self(input)
    }
}

#[allow(unused)]
fn pair<'a, P1, P2, R1, R2>(p1: P1, p2: P2) -> impl Parser<(R1, R2)>
where
    P1: Parser<R1>,
    P2: Parser<R2>,
{
    move |input| p1.parse(input)
        .and_then(|(rest, r1)| {
            p2.parse(rest)
                .map(|(rest, r2)| (rest, (r1, r2)))
        })
}

#[allow(unused)]
fn either<'a, P1, P2, R>(p1: P1, p2: P2) -> impl Parser<R>
where
    P1: Parser<R>,
    P2: Parser<R>,
{
    move |input| match p1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => p2.parse(input),
    }
}

// ============================================================================
macro_rules! PubBinOprator {
    ($($ENUM: ident, $STR: expr);*$(;)?) => {
        #[allow(unused)]
        #[repr(u8)]
        #[derive(Debug, PartialEq, Eq)]
        pub enum BinOprator {
            $($ENUM),*
        }
        impl Display for BinOprator {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use BinOprator::*;
                match self {
                    $($ENUM => write!(f, $STR)),*
                }
            }
        }
    };
}

PubBinOprator!{
    Add, "+";
    Sub, "-";
    Undefined, "Undefined"
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Number(i32),
    BinOP(BinOprator, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn display(&self) -> String {
        Expr::_display(self, 0)
    }
    fn _display(e: &Expr, level: usize) -> String {
        use Expr::*;
        "  ".repeat(level) + &match e {
            Number(v) => format!("|- {}", v),
            BinOP(op, e1, e2) => format!("|- {}\n{}\n{}",
                op, Expr::_display(&e1, level+1), Expr::_display(&e2, level+1),
            ),
        }
    }
}

/// 解析器：解析 Token::Number(String)
#[allow(unused)]
fn number<'a>(input: &'a [Token]) -> PResult<'a, Expr> {
    if let Some((Token::Number(num_str), rest)) = input.split_first() {
        let num = num_str.parse::<i32>().map_err(|_| input)?;
        Ok((rest, Expr::Number(num)))
    } else {
        Err(input)
    }
}

#[allow(unused)]
fn symbol<'a, 'b>(expected: Token, s: &'b str) -> impl Fn(&'a [Token]) -> PResult<'a, &'b str>
{
    move |input: &'a [Token]| match input.split_first() {
        Some((token, rest)) if *token == expected => Ok((rest, s)),
        _ => Err(input),
    }
}

#[allow(unused)]
fn binop<'a>() -> impl Parser<Expr> {
    use BinOprator::*;
    |input| pair(
        number, pair(
        either(
            symbol(Token::Plus, "+"),
            symbol(Token::Minus, "-"),
        ),
        number,
    )).parse(input)
    .map(|(rest, (e1, (op, e2)))|
        (rest, Expr::BinOP(
            match op {
                s if s == "+" => Add,
                s if s == "-" => Sub,
                _ => Undefined,
            }, Box::new(e1), Box::new(e2))
        )
    )
}

#[allow(unused)]
fn expr<'a>() -> impl Parser<Expr> {
    either(
        binop(),
        number,
    )
}

#[allow(unused)]
pub fn pascal<'a>() -> impl Parser<Expr> {
    expr()
}

#[test]
fn parser_number() {
    assert_eq!(
        Ok((&vec![][..], Expr::Number(32))),
        number.parse(&vec![Token::Number("0032".to_string())]),
    )
}

#[test]
fn parser_expr() {
    use BinOprator::*;
    use Token::*;
    let s = vec![
        Number("1".to_string()),
        Plus,
        Number("2".to_string()),
        ];
    let p = expr();
    assert_eq!(
        Ok((&vec![][..], Expr::BinOP(Add, Box::new(Expr::Number(1)), Box::new(Expr::Number(2))))),
        p.parse(&s[..])
    );
}

