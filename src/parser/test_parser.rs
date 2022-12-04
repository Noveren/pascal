#[test]
fn number() {
    use super::{parser, nom::*};
    let p = right(
        parser::whitespace::<true>(),
        parser::number::<i32, 10>(),
    );
    assert_eq!(
        Ok(("symbol", 12)),
        p.parse("  12symbol"),
    );
    assert_eq!(
        Err("No Whitespace".to_string()),
        p.parse("12symbol"),
    );
}