mod debug;
mod expr;

pub use debug::DebugWithTables;
pub use expr::{Expr, ExprData, ExprKind};

use crate::{
    parse::{Input, PResult, Parse},
    table::AllocTable,
};

impl<T: DebugWithTables> DebugWithTables for Vec<T> {
    fn debug(&self, tables: &AllocTable<Expr, ExprData>) -> String {
        let items = self.iter().map(|item| item.debug(tables)).collect::<Vec<_>>().join(", ");
        format!("[{items}]")
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(input: &mut Input) -> PResult<Self> {
        input.delimited('[', ']', |this| this.parse_comma(']'))
    }
}
