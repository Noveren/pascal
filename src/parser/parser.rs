use super::{Node, Symbol};
use super::{nom, nom::{Parser, pair, right, whitespace}};
use super::nom::{the_char, PResult};

#[allow(unused)]
pub fn number<'a>() -> impl Parser<'a, Node> {
    right(
        whitespace::<false>(),
        nom::number::<10>(),
    ).map(|s| Node::Number(s))
}

#[allow(unused)]
pub fn expr<'a>() -> impl Parser<'a, Node> {
    pair(
        number(),
        pair(
                right(whitespace::<false>(), the_char::<'+'>)
            .or(right(whitespace::<false>(), the_char::<'-'>))
            .or(right(whitespace::<false>(), the_char::<'*'>))
            .or(right(whitespace::<false>(), the_char::<'/'>))
            .map_err(|ctx| format!("Undefined Symbol at {}", ctx)),
            number(),
        )
    ).map(|(n1, (op, n2))| Node::Expr(
        match op {
            '+' => Symbol::ADD,
            '-' => Symbol::SUB,
            '*' => Symbol::MUL,
             _  => Symbol::DIV,
        }, vec![n1, n2]
    ))
}

/// 执行 Block，记录并打印执行时间，然后返回 Block 的值
macro_rules! time {
    ($code: block) => {{
        let start = std::time::SystemTime::now();
        let result = $code;
        let end   = std::time::SystemTime::now();
        println!("The time of parsing source code: {:?}",
            end.duration_since(start)
                .expect("Clock may have gone backwards")
        );
        result
    }};
}

/// 语法分析器接口
#[allow(unused)]
pub fn parse(src: &str) -> PResult<Node> {   
    time!({
        // 生成解析器
        let p = expr();
        // 执行解析
        p.parse(super::nom::Context::new(src))
    })
}