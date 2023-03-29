#[macro_export]
macro_rules! key {
    ($name:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {
            raw: $crate::table::RawKey,
        }

        impl Key for $name {
            fn from_usize(key: usize) -> $name {
                $name { raw: RawKey::from_usize(key) }
            }

            fn as_usize(&self) -> usize {
                self.raw.as_usize()
            }
        }
    };
}
