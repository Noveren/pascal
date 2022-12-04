mod nom;
mod parser;

#[cfg(test)]
mod test_parser;
#[cfg(test)]
mod test_nom;

pub use parser::parse;
use std::fmt::Display;

macro_rules! symbol {
    ($($id: ident, $st: expr);*$(;)?) => {
        #[allow(unused)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum Symbol {
            $($id),*
        }
        impl Display for Symbol {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use Symbol::*;
                match self {
                    $($id => write!(f, $st)),*
                }
            }
        }
    };
}

symbol!{
    ADD, "ADD";
    SUB, "SUB";
    MUL, "MUL";
    DIV, "DIV";
}


#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Number(String),
    Expr(Symbol, Vec<Node>),
}

impl Node {
    #[allow(unused)]
    fn _display(node: &Self, level: usize) -> String {
        use Node::*;
        match node {
            Number(v)  => format!("{}|- {}\n", "  ".repeat(level), v),
            Expr(s, v) => {
                let mut ss = format!("{}|- {}\n", "  ".repeat(level), s);
                for n in v.iter() {
                    ss += &Node::_display(n, level + 1);
                }
                ss
            }
        }
    }
    #[allow(unused)]
    pub fn display(&self) -> String {
        Node::_display(self, 0)
    }
}