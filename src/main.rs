pub mod ast;
pub mod parse;
pub mod table;

fn main() {
    use crate::ast::Expr;
    use crate::parse::{Input, Parse as _};

    let (items, _) = <Vec<Expr>>::parse(Input::of("[]"));
    dbg!(items);

    let (items, _) = <Vec<Expr>>::parse(Input::of(" [   ] "));
    dbg!(items);

    let (items, _) = <Vec<Expr>>::parse(Input::of("--\n[]"));
    dbg!(items);
}
