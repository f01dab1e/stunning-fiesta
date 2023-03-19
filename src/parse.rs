use crate::{
    ast::{Expr, ExprData},
    table::AllocTable,
};

pub struct Input<'text, 'arena> {
    pub tables: &'arena mut AllocTable<Expr, ExprData>,
    pub text: &'text str,
}

impl<'text, 'arena> Input<'text, 'arena> {
    pub fn new(text: &'text str, tables: &'arena mut AllocTable<Expr, ExprData>) -> Self {
        Self { tables, text }
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

    pub fn at(&self, ch: char) -> bool {
        self.text.chars().next() == ch.into()
    }

    pub fn expect(&mut self, pattern: impl Edible) {
        self.skip_trivia();
        pattern.eat(self);
    }

    pub fn parse<T: Parse>(&mut self) -> T {
        T::parse(self)
    }

    pub fn parse_comma<T: Parse>(&mut self, close: char) -> Vec<T> {
        T::parse_comma(self, close)
    }
}

pub trait Parse: Sized {
    fn parse(input: &mut Input) -> Self;

    fn parse_comma(input: &mut Input, close: char) -> Vec<Self> {
        let mut items = Vec::new();

        loop {
            input.skip_trivia();

            if input.at(close) {
                break;
            }

            let item = input.parse();
            items.push(item);

            input.expect(',');
        }

        items
    }
}

pub trait Edible {
    fn eat(self, input: &mut Input);
}

impl Edible for char {
    fn eat(self, mut input: &mut Input) {
        if let Some(rest) = input.text.strip_prefix(self) {
            input.text = rest;
        }

        panic!("expected {self}");
    }
}
