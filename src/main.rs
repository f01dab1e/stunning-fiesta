use crate::syntax::Expr;
use crate::table::AllocTable;

mod parse;
mod span;
mod syntax;
mod table;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(path).unwrap();

    let mut tables = AllocTable::default();
    let ast: Vec<Expr> = syntax::parse(&text, &mut tables).unwrap();

    dbg!(ast);
}
