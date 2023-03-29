use syntax::Debug;
use tables::Tables;

use crate::syntax::Expr;

use crate::type_checker::TypeChecker;

mod parse;
mod span;
mod syntax;
mod table;
mod tables;
mod type_checker;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(path).unwrap();

    let mut tables = Tables::default();
    let ast: Vec<Expr> = syntax::parse(&text, &mut tables).unwrap();

    let _type_checker = TypeChecker::new(&tables);

    println!("{:?}", ast.debug_with(&tables));
}
