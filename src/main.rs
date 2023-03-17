pub mod ast;
pub mod parse;
pub mod table;

fn main() {
    use crate::ast::Expr;
    use crate::parse::{Input, Parse as _};
    use crate::table::AllocTable;

    let (items, _) = <Vec<Expr>>::parse(Input::new("[]", &mut AllocTable::default()));
    dbg!(items);

    let (items, _) = <Vec<Expr>>::parse(Input::new(" [   ] ", &mut AllocTable::default()));
    dbg!(items);

    let (items, _) = <Vec<Expr>>::parse(Input::new("--\n[]", &mut AllocTable::default()));
    dbg!(items);
}
