mod debug;
mod expr;

pub use debug::Debug;
pub use expr::{Expr, ExprData, ExprKind};

use crate::{
    parse::{Input, PResult, Parse},
    table::AllocTable,
};

impl<T: Debug> Debug for Vec<T> {
    fn fmt(
        &self,
        tables: &AllocTable<Expr, ExprData>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let items = self.iter().map(|item| item.debug_with(tables));
        f.debug_list().entries(items).finish()
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(input: &mut Input) -> PResult<Self> {
        input.delimited('[', ']', |this| this.parse_comma(']'))
    }
}

pub fn parse<T: Parse>(text: &str, tables: &mut AllocTable<Expr, ExprData>) -> PResult<T> {
    let mut input = Input::new(text, tables);
    T::parse(&mut input)
}
