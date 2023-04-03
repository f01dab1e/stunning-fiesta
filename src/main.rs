use tables::{Debug, Tables};

use crate::syntax::Expr;
use crate::type_checker::TypeChecker;

mod collection {
    pub type IndexSet<T> =
        indexmap::IndexSet<T, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
}

#[macro_use]
mod macros;
mod parse;
mod span;
mod syntax;
mod tables;
mod type_checker;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(path).unwrap();

    let tables = Tables::default();
    let ast: Vec<Expr> = syntax::parse(&text, &tables).unwrap();

    let _type_checker = TypeChecker::new(&tables);

    println!("{:?}", ast.debug_with(&tables));
}
