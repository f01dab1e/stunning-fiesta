use std::num::NonZeroU32;

pub trait Key {
    fn from_usize(key: usize) -> Self;
    fn as_usize(&self) -> usize;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawKey {
    key: NonZeroU32,
}

impl RawKey {
    pub const MAX: u32 = 0xFFFF_FF00;

    unsafe fn new_unchecked(key: u32) -> Self {
        Self { key: NonZeroU32::new_unchecked(key + 1) }
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
