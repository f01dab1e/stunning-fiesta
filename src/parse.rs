use crate::ast::Expr;

trait Input {
    fn expect(&self, pattern: impl Pattern) -> &Self;
    fn parse1<T: Parse>(&self) -> (T, &str);
    fn parse_comma<T: Parse>(&self, close: char) -> (Vec<T>, &str);
}

impl Input for str {
    fn expect(&self, pattern: impl Pattern) -> &str {
        pattern.check(self)
    }

    fn parse1<T: Parse>(&self) -> (T, &str) {
        T::parse(self)
    }

    fn parse_comma<T: Parse>(&self, close: char) -> (Vec<T>, &str) {
        T::parse_comma(self, close)
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
            let (item, rest) = input.parse1();

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
        let (items, input) = input.parse_comma(']');
        let input = input.expect(']');
        (items, input)
    }
}
