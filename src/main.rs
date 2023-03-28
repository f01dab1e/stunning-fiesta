use crate::syntax::Expr;
use crate::table::AllocTable;
use crate::type_checker::TypeChecker;

mod parse;
mod span;
mod syntax;
mod table;
mod type_checker;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(path).unwrap();

    let mut tables = AllocTable::default();
    let ast: Vec<Expr> = syntax::parse(&text, &mut tables).unwrap();

    let type_checker = TypeChecker::new(&tables);

    drop((ast, type_checker));
}
