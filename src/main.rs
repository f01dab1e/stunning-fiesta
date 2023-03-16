pub mod ast;
pub mod parse;
pub mod table;

fn main() {
    use crate::ast::Expr;
    use crate::parse::{Input, Parse as _};

    let input = Input::of("[]");
    let (items, _) = <Vec<Expr>>::parse(input);
    dbg!(items);
}
