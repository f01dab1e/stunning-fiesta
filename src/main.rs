use parse::Parse;

use crate::ast::Expr;

pub mod table {
    use std::marker::PhantomData;

    pub trait Key {
        fn from_usize(key: usize) -> Self;
        fn as_usize(&self) -> usize;
    }

    impl Key for usize {
        fn from_usize(key: usize) -> Self {
            key
        }

        fn as_usize(&self) -> usize {
            *self
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
    pub type Expr = usize;

    pub struct Span {
        pub lo: u32,
        pub hi: u32,
    }

    pub struct ExprData {
        pub kind: ExprKind,
        pub span: Span,
    }

    pub enum ExprKind {
        Integer(u64),
        List(Expr),
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
    let xs = <Vec<Expr>>::parse("[]");
    dbg!(xs);
}
