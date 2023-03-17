use crate::{
    ast::{Expr, ExprData},
    table::AllocTable,
};

pub struct Input<'text, 'arena> {
    tables: &'arena mut AllocTable<Expr, ExprData>,
    text: &'text str,
}

impl<'me, 'arena> Input<'me, 'arena> {
    pub fn new(text: &'me str, tables: &'arena mut AllocTable<Expr, ExprData>) -> Self {
        Self { tables, text }
    }

    pub fn of(self, text: &'me str) -> Self {
        Self {
            text,
            tables: self.tables,
        }
    }

    fn skip_trivia(mut self) -> Self {
        loop {
            let len = self.text.len();

            self.text = self.text.trim_start();
            if self.text.starts_with("--") {
                self.text = self
                    .text
                    .find('\n')
                    .map_or("", |index| &self.text[index + 1..]);
            }

            if self.text.len() == len {
                return self;
            }
        }
    }

    fn expect(self, pattern: impl Pattern) -> Self {
        pattern.check(self.skip_trivia())
    }

    fn parse<T: Parse>(self) -> (T, Self) {
        T::parse(self)
    }

    fn parse_comma<T: Parse>(self, close: char) -> (Vec<T>, Self) {
        T::parse_comma(self, close)
    }
}

trait Pattern {
    fn check<'text, 'arena>(self, input: Input<'text, 'arena>) -> Input<'text, 'arena>;
}

impl Pattern for char {
    fn check<'text, 'arena>(self, input: Input<'text, 'arena>) -> Input<'text, 'arena> {
        if let Some(rest) = input.text.strip_prefix(self) {
            return input.of(rest);
        }

        panic!("expected {self}");
    }
}

pub trait Parse: Sized {
    fn parse<'text, 'arena>(input: Input<'text, 'arena>) -> (Self, Input<'text, 'arena>);

    fn parse_comma<'text, 'arena>(
        mut input: Input<'text, 'arena>,
        close: char,
    ) -> (Vec<Self>, Input<'text, 'arena>) {
        let mut items = Vec::new();

        loop {
            input = input.skip_trivia();

            if input.text.starts_with(close) {
                break;
            }

            let (item, rest) = input.parse();
            input = rest;
            items.push(item);

            input = input.expect(',');
        }

        (items, input)
    }
}

impl Parse for Expr {
    fn parse<'text, 'arena>(_input: Input<'text, 'arena>) -> (Self, Input<'text, 'arena>) {
        todo!()
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse<'text, 'arena>(input: Input<'text, 'arena>) -> (Self, Input<'text, 'arena>) {
        let input = input.expect('[');
        let (items, input) = input.parse_comma(']');
        let input = input.expect(']');
        (items, input)
    }
}
