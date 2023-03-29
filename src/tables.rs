use crate::{
    syntax::{Expr, ExprData},
    table::AllocTable,
    type_checker::{Ty, TyData},
};

#[derive(Default)]
pub struct Tables {
    exprs: AllocTable<Expr, ExprData>,
    tys: AllocTable<Ty, TyData>,
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

impl AllocValue for TyData {
    type Key = Ty;

    fn add(self, tables: &mut Tables) -> Self::Key {
        tables.tys.add(self)
    }
}

impl AllocKey for Ty {
    type Value = TyData;

    fn data(self, tables: &Tables) -> &Self::Value {
        tables.tys.data(self)
    }
}
