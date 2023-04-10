use super::Key;
use crate::collection::IndexSet;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct InternTable<K, V> {
    values: IndexSet<V>,
    marker: PhantomData<K>,
}

impl<K, V> Default for InternTable<K, V> {
    fn default() -> Self {
        Self { values: <_>::default(), marker: <_>::default() }
    }
}

impl<K: Key, V: Hash + Eq> InternTable<K, V> {
    pub fn add(&mut self, value: V) -> K {
        let (key, _) = self.values.insert_full(value);
        K::from_usize(key)
    }

    pub fn data(&self, key: K) -> &V {
        &self.values[key.as_usize()]
    }
}
