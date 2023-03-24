use crate::table::AllocTable;

use super::{Expr, ExprData};

pub trait Debug: Sized {
    fn debug_with<'data, 'arena>(
        &'data self,
        tables: &'arena AllocTable<Expr, ExprData>,
    ) -> DebugWith<'data, 'arena, Self> {
        DebugWith(self, tables)
    }

    fn fmt(
        &self,
        tables: &AllocTable<Expr, ExprData>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result;
}

pub struct DebugWith<'data, 'arena, T>(&'data T, &'arena AllocTable<Expr, ExprData>);

impl<'data, 'arena, T: Debug> std::fmt::Debug for DebugWith<'data, 'arena, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.0, self.1, f)
    }
}
