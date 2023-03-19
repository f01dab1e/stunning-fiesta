pub mod ast;
pub mod parse;
mod span;
pub mod table;

fn main() {
    use crate::ast::Expr;
    use crate::parse::{Input, Parse as _};
    use crate::table::AllocTable;

    let items = <Vec<Expr>>::parse(&mut Input::new("[]", &mut AllocTable::default()));
    dbg!(items);

    let items = <Vec<Expr>>::parse(&mut Input::new(" [   ] ", &mut AllocTable::default()));
    dbg!(items);

    let items = <Vec<Expr>>::parse(&mut Input::new("--\n[]", &mut AllocTable::default()));
    dbg!(items);
}
