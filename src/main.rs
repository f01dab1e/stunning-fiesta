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

    impl Key for RawKey {
        fn from_usize(key: usize) -> Self {
            let key = key.try_into().unwrap();
            assert_ne!(key, u32::MAX);

            RawKey {
                key: unsafe { NonZeroU32::new_unchecked(key) },
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

    impl<K: Default, V: Default> Default for AllocTable<K, V> {
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
        Integer(u64),
        Float(u64), // We convert float values into bits and that's how we don't need to deal with f32 and f64.
        Boolean(bool),
        List(Vec<Expr>),
    }

    pub enum ExprKind {
        Literal(Literal),
        If(Expr, Expr, Option<Expr>),
    }
}

pub mod parse {
    use crate::ast::Expr;

    fn char(input: &str) -> (char, &str) {
        let ch = match input.chars().next() {
            Some(c) => c,
            None => panic!("unexpected eof"),
        };

        (ch, &input[ch.len_utf8()..])
    }

    fn expect_char(input: &str, ch: char) -> &str {
        let (actual, input) = char(input);

        if actual != ch {
            panic!("expected {ch}");
        }

        input
    }

    pub trait Parse: Sized {
        fn parse(input: &str) -> (Self, &str);
    }

    impl Parse for Expr {
        fn parse(_input: &str) -> (Self, &str) {
            todo!()
        }
    }

    impl<T: Parse> Parse for Vec<T> {
        fn parse(input: &str) -> (Self, &str) {
            let input = expect_char(input, '[');
            let xs = Vec::new();
            let input = expect_char(input, ']');
            (xs, input)
        }
    }
}

fn main() {
    use crate::ast::Expr;
    use crate::parse::Parse as _;

    let xs = <Vec<Expr>>::parse("[]");
    dbg!(xs);
}
