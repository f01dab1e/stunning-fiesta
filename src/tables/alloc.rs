use std::marker::PhantomData;

use super::Key;

#[derive(Debug)]
pub struct AllocTable<K, V> {
    values: Vec<V>,
    marker: PhantomData<K>,
}

impl<K, V> Default for AllocTable<K, V> {
    fn default() -> Self {
        Self { values: <_>::default(), marker: <_>::default() }
    }
}

impl<K: Key, V> AllocTable<K, V> {
    pub fn add(&mut self, value: V) -> K {
        let key = self.values.len();
        self.values.push(value);
        K::from_usize(key)
    }

    pub fn data(&self, key: K) -> &V {
        &self.values[key.as_usize()]
    }
}
