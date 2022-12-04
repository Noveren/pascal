use super::{Node, Symbol};
pub use super::nom::{Parser, Context};
use super::{nom, nom::{pair, right, whitespace, the_char, PResult}};

#[allow(unused)]
pub fn number<'a>() -> impl Parser<'a, Node> {
    right(
        whitespace::<false>(),
        nom::integer::<10>(),                           // 十进制整数
    ).map(|s| Node::Number(s))
}

#[allow(unused)]
fn factor<'a>() -> impl Parser<'a, Node> {
    number()
}

#[allow(unused)]
pub fn term<'a>() -> impl Parser<'a, Node> {
    pair(
        factor(),
        pair(
                right(whitespace::<false>(), the_char::<'*'>)
            .or(right(whitespace::<false>(), the_char::<'/'>))
            .map_err(|_| "Undefined Symbol".to_string()),
            factor(),
        ).m0()
    ).map(|(n1, vs)| {
        let mut rev_iter = vs.into_iter().rev();
        if let Some((op, node)) = rev_iter.next() {
            let mut temp = (op, node);
            while let Some((op, node)) = rev_iter.next() {
                temp.1 = Node::Expr(
                    if op == '*' { Symbol::MUL } else { Symbol::DIV },
                    vec![node, temp.1],
                );
                temp.0 = op;
            }
            Node::Expr(
                if temp.0 == '*' { Symbol::MUL } else { Symbol::DIV },
                vec![n1, temp.1],
            )
        } else {
            n1
        }
    })
}

#[allow(unused)]
pub fn expr<'a>() -> impl Parser<'a, Node> {
    term()
    // pair(
    //     number(),
    //     pair(
    //             right(whitespace::<false>(), the_char::<'+'>)
    //         .or(right(whitespace::<false>(), the_char::<'-'>))
    //         .or(right(whitespace::<false>(), the_char::<'*'>))
    //         .or(right(whitespace::<false>(), the_char::<'/'>))
    //         .map_err(|_| "Undefined Symbol".to_string()),
    //         number(),
    //     )
    // ).map(|(n1, (op, n2))| Node::Expr(
    //     match op {
    //         '+' => Symbol::ADD,
    //         '-' => Symbol::SUB,
    //         '*' => Symbol::MUL,
    //          _  => Symbol::DIV,
    //     }, vec![n1, n2]
    // ))
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