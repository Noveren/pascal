mod nom;
mod parser;

#[cfg(test)]
mod test_parser;
#[cfg(test)]
mod test_nom;

pub use parser::parse;

// TODO AST 数据结构定义

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    ADD,
    SUB,
    MUL,
    DIV,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Number(String),
    Expr(Symbol, Vec<Node>),
}