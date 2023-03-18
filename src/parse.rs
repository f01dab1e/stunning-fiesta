use crate::{
    ast::{Expr, ExprData},
    table::AllocTable,
};

pub struct Input<'text, 'arena> {
    tables: &'arena mut AllocTable<Expr, ExprData>,
    text: &'text str,
}

impl<'text, 'arena> Input<'text, 'arena> {
    pub fn new(text: &'text str, tables: &'arena mut AllocTable<Expr, ExprData>) -> Self {
        Self { tables, text }
    }

    pub fn of(self, text: &'text str) -> Self {
        Self {
            text,
            tables: self.tables,
        }
    }

    fn skip_trivia(&mut self) {
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
                break;
            }
        }
    }

    fn expect(&mut self, pattern: impl Pattern) {
        self.skip_trivia();
        pattern.eat(self);
    }

    fn parse<T: Parse>(&mut self) -> T {
        T::parse(self)
    }

    fn parse_comma<T: Parse>(&mut self, close: char) -> Vec<T> {
        T::parse_comma(self, close)
    }
}

trait Pattern {
    fn eat(self, input: &mut Input);
}

impl Pattern for char {
    fn eat(self, mut input: &mut Input) {
        if let Some(rest) = input.text.strip_prefix(self) {
            input.text = rest;
        }

        panic!("expected {self}");
    }
}

pub trait Parse: Sized {
    fn parse(input: &mut Input) -> Self;

    fn parse_comma(input: &mut Input, close: char) -> Vec<Self> {
        let mut items = Vec::new();

        loop {
            input.skip_trivia();

            if input.text.starts_with(close) {
                break;
            }

            let item = input.parse();
            items.push(item);

            input.expect(',');
        }

        items
    }
}

impl Parse for Expr {
    fn parse(_input: &mut Input) -> Self {
        todo!()
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(input: &mut Input) -> Self {
        input.expect('[');
        let items = input.parse_comma(']');
        input.expect(']');
        items
    }
}
