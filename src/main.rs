pub mod table {
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
    }

    impl Key for RawKey {
        fn from_usize(key: usize) -> Self {
            assert!(key < (Self::MAX as usize));

            RawKey {
                key: unsafe { NonZeroU32::new_unchecked(key as u32 + 1) },
            }
        }

        fn as_usize(&self) -> usize {
            self.key.get() as usize - 1
        }
    }

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
}

pub mod ast {
    use crate::table::RawKey;

    pub type Expr = RawKey;

    pub struct Span {
        pub lo: u32,
        pub hi: u32,
    }

    pub struct ExprData {
        pub kind: ExprKind,
        pub span: Span,
    }

    pub enum Literal {
        /// `42`, `69`, etc
        Integer(u64),
        /// `integer-part.fractional-part`
        /// We convert float values into bits and that's how we don't need to deal with f32 and f64
        Float(u64),
        /// `true`, `false`
        Boolean(bool),
        /// `[]` or `[a, b, ...]`
        List(Vec<Expr>),
    }

    pub enum ExprKind {
        /// Literal.
        Literal(Literal),
        /// `if condition { block } [else { block }]`
        If(Expr, Expr, Option<Expr>),
    }
}

pub mod parse {
    use crate::ast::Expr;

    trait Input {
        fn expect(&self, pattern: impl Pattern) -> Self;
    }

    impl Input for &str {
        fn expect(&self, pattern: impl Pattern) -> Self {
            pattern.check(self)
        }
    }

    trait Pattern {
        fn check(self, input: &str) -> &str;
    }

    impl Pattern for char {
        fn check(self, input: &str) -> &str {
            if let Some(input) = input.strip_prefix(self) {
                return input;
            }

            panic!("expected {self}");
        }
    }

    pub trait Parse: Sized {
        fn parse(input: &str) -> (Self, &str);

        fn parse_comma(mut input: &str, close: char) -> (Vec<Self>, &str) {
            let mut items = Vec::new();

            while !input.starts_with(close) {
                let (item, rest) = Self::parse(input);

                input = rest;
                items.push(item);

                input = input.expect(',');
            }

            (items, input)
        }
    }

    impl Parse for Expr {
        fn parse(_input: &str) -> (Self, &str) {
            todo!()
        }
    }

    impl<T: Parse> Parse for Vec<T> {
        fn parse(input: &str) -> (Self, &str) {
            let input = input.expect('[');
            let (items, input) = T::parse_comma(input, ']');
            let input = input.expect(']');
            (items, input)
        }
    }
}

fn main() {
    use crate::ast::Expr;
    use crate::parse::Parse as _;

    let xs = <Vec<Expr>>::parse("[]");
    dbg!(xs);
}
