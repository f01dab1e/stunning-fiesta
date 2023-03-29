use crate::{
    syntax::{Expr, ExprData},
    table::AllocTable,
};

#[derive(Default)]
pub struct Tables {
    exprs: AllocTable<Expr, ExprData>,
}

impl Tables {
    pub fn add<V: AllocValue>(&mut self, value: V) -> V::Key {
        value.add(self)
    }

    pub fn data<K: AllocKey>(&self, key: K) -> &K::Value {
        key.data(self)
    }
}

pub trait AllocValue {
    type Key;

    fn add(self, tables: &mut Tables) -> Self::Key;
}

pub trait AllocKey {
    type Value;

    fn data(self, tables: &Tables) -> &Self::Value;
}

impl AllocValue for ExprData {
    type Key = Expr;

    fn add(self, tables: &mut Tables) -> Self::Key {
        tables.exprs.add(self)
    }
}

impl AllocKey for Expr {
    type Value = ExprData;

    fn data(self, tables: &Tables) -> &Self::Value {
        tables.exprs.data(self)
    }
}
