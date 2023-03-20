use crate::{
    ast::{Expr, ExprData},
    table::AllocTable,
};

pub type PResult<T> = Result<T, ParseError>;

#[allow(dead_code)]
pub fn parse<T: Parse>(text: &str, tables: &mut AllocTable<Expr, ExprData>) -> PResult<T> {
    let mut input = Input::new(text, tables);
    T::parse(&mut input)
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

pub struct Input<'text, 'arena> {
    pub tables: &'arena mut AllocTable<Expr, ExprData>,
    pub text: &'text str,
}

impl<'text, 'arena> Input<'text, 'arena> {
    #[allow(dead_code)]
    pub fn new(text: &'text str, tables: &'arena mut AllocTable<Expr, ExprData>) -> Self {
        Self { tables, text }
    }

    fn skip_trivia(&mut self) {
        loop {
            let len = self.text.len();

            self.text = self.text.trim_start();
            if self.text.starts_with("--") {
                self.text = self.text.find('\n').map_or("", |index| &self.text[index + 1..]);
            }

            if self.text.len() == len {
                break;
            }
        }
    }

    pub fn at(&self, ch: char) -> bool {
        self.text.chars().next() == ch.into()
    }

    pub fn expect(&mut self, edible: impl Edible) -> PResult<()> {
        edible.eat(self)
    }

    pub fn parse<T: Parse>(&mut self) -> PResult<T> {
        T::parse(self)
    }

    pub fn parse_comma<T: Parse>(&mut self, close: char) -> PResult<Vec<T>> {
        T::parse_comma(self, close)
    }
}

pub trait Parse: Sized {
    fn parse(input: &mut Input) -> PResult<Self>;

    fn parse_comma(input: &mut Input, close: char) -> PResult<Vec<Self>> {
        let mut items = Vec::new();

        loop {
            input.skip_trivia();

            if input.at(close) {
                break;
            }

            let item = input.parse()?;
            items.push(item);

            input.expect(',')?;
        }

        Ok(items)
    }
}

pub trait Edible {
    fn eat(self, input: &mut Input) -> PResult<()>;
}

impl Edible for char {
    fn eat(self, mut input: &mut Input) -> PResult<()> {
        input.skip_trivia();

        if let Some(rest) = input.text.strip_prefix(self) {
            input.text = rest;
            return Ok(());
        }

        Err(ParseError { message: format!("expected `{self}`") })
    }
}
