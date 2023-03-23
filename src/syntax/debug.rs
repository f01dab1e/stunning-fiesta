use crate::{
    syntax::{Expr, ExprData},
    table::AllocTable,
};

pub trait DebugWithTables {
    fn debug(&self, tables: &AllocTable<Expr, ExprData>) -> String;
}
