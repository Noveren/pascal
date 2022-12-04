use super::{parser, nom};
#[allow(unused)]
macro_rules! test_ok {
    ($parser: expr, $result: expr, $src: expr, $go: expr) => {
        assert_eq!(
            nom::pok(nom::Context::new($src).move_str($go), $result),
            nom::Parser::parse(&$parser, nom::Context::new($src)),
        )
    };
    ($parser: expr, $result: expr, $src: expr) => {
        assert_eq!(
            nom::pok(nom::Context::new($src), $result),
            nom::Parser::parse(&$parser, nom::Context::new($src)),
        )
    };
}
#[allow(unused)]
macro_rules! test_err {
    ($parser: expr, $result: expr, $src: expr, $go: expr) => {
        assert_eq!(
            nom::perr(nom::Context::new($src).move_str($go), $result),
            nom::Parser::parse(&$parser, nom::Context::new($src)),
        )
    };
    ($parser: expr, $result: expr, $src: expr) => {
        assert_eq!(
            nom::perr(nom::Context::new($src), $result),
            nom::Parser::parse(&$parser, nom::Context::new($src)),
        )
    };
}


#[test]
fn number() {
    let p = nom::right(
        parser::whitespace::<false>(),
        parser::number::<10>(),
    );
    test_ok!(
        p, "12".to_string(), "   12symbol", "   12"
    );
    test_err!(
        p, "No Number".to_string(), "symbol"
    );
}