pub mod ast;
pub mod parse;
pub mod table;

fn main() {
    use crate::ast::Expr;
    use crate::parse::Parse as _;

    let xs = <Vec<Expr>>::parse("[]");
    dbg!(xs);
}
