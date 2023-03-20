use crate::ast::Expr;
use crate::table::AllocTable;

mod ast;
mod parse;
mod span;
mod table;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();

    let mut tables = AllocTable::default();
    let ast: Vec<Expr> = parse::parse(&text, &mut tables).unwrap();

    dbg!(ast);
}
