use std::{marker::PhantomData, num::NonZeroU32};

pub trait Key {
    fn from_usize(key: usize) -> Self;
    fn as_usize(&self) -> usize;
}

#[derive(Debug)]
pub struct RawKey {
    key: NonZeroU32,
}

impl RawKey {
    pub const MAX: u32 = 0xFFFF_FF00;

    unsafe fn new_unchecked(key: u32) -> Self {
        Self {
            key: NonZeroU32::new_unchecked(key + 1),
        }
    }
}

impl Key for RawKey {
    fn from_usize(key: usize) -> Self {
        assert!(key < (Self::MAX as usize));

        unsafe { RawKey::new_unchecked(key as u32) }
    }

    fn as_usize(&self) -> usize {
        (self.key.get() - 1) as usize
    }
}

#[allow(dead_code)]
pub struct AllocTable<K, V> {
    values: Vec<V>,
    marker: PhantomData<K>,
}

impl<K, V> Default for AllocTable<K, V> {
    fn default() -> Self {
        Self {
            values: <_>::default(),
            marker: <_>::default(),
        }
    }
}

#[allow(dead_code)]
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
