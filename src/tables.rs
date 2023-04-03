mod alloc;
mod debug;
mod intern;
mod key;

use std::cell::{Ref, RefCell};

use crate::{
    syntax::{Expr, ExprData},
    type_checker::{Ty, TyData},
};

pub use alloc::AllocTable;
pub use debug::{Debug, DebugWith};
pub use intern::InternTable;
pub use key::{Key, RawKey};

#[derive(Default)]
pub struct RawTables {
    exprs: AllocTable<Expr, ExprData>,
    tys: InternTable<Ty, TyData>,
}

#[derive(Default)]
pub struct Tables {
    raw: RefCell<RawTables>,
}

impl Tables {
    pub fn add<V: AllocValue>(&self, value: V) -> V::Key {
        value.add(self)
    }

    pub fn data<K: AllocKey>(&self, key: K) -> K::Value<'_> {
        key.data(self)
    }
}

pub trait AllocValue {
    type Key;

    fn add(self, tables: &Tables) -> Self::Key;
}

pub trait AllocKey {
    type Value<'a>;

    fn data(self, tables: &Tables) -> Self::Value<'_>;
}

impl AllocValue for ExprData {
    type Key = Expr;

    fn add(self, tables: &Tables) -> Self::Key {
        tables.raw.borrow_mut().exprs.add(self)
    }
}

impl AllocKey for Expr {
    type Value<'a> = Ref<'a, ExprData>;

    fn data(self, tables: &Tables) -> Self::Value<'_> {
        Ref::map(tables.raw.borrow(), |table| table.exprs.data(self))
    }
}

impl AllocValue for TyData {
    type Key = Ty;

    fn add(self, tables: &Tables) -> Self::Key {
        tables.raw.borrow_mut().tys.add(self)
    }
}

impl AllocKey for Ty {
    type Value<'a> = Ref<'a, TyData>;

    fn data(self, tables: &Tables) -> Self::Value<'_> {
        Ref::map(tables.raw.borrow(), |table| table.tys.data(self))
    }
}
