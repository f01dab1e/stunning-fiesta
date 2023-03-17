use crate::ast::Expr;

pub struct Input<'a> {
    text: &'a str,
}

impl<'a> Input<'a> {
    pub fn of(text: &'a str) -> Self {
        Self { text }
    }

    fn skip_trivia(mut self) -> Input<'a> {
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

    fn expect(self, pattern: impl Pattern) -> Input<'a> {
        pattern.check(self.skip_trivia())
    }

    fn parse<T: Parse>(self) -> (T, Input<'a>) {
        T::parse(self)
    }

    fn parse_comma<T: Parse>(self, close: char) -> (Vec<T>, Input<'a>) {
        T::parse_comma(self, close)
    }
}

trait Pattern {
    fn check(self, input: Input) -> Input;
}

impl Pattern for char {
    fn check(self, input: Input) -> Input {
        if let Some(input) = input.text.strip_prefix(self) {
            return Input::of(input);
        }

        panic!("expected {self}");
    }
}

pub trait Parse: Sized {
    fn parse(input: Input) -> (Self, Input);

    fn parse_comma(mut input: Input, close: char) -> (Vec<Self>, Input) {
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
    fn parse(_input: Input) -> (Self, Input) {
        todo!()
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(input: Input) -> (Self, Input) {
        let input = input.expect('[');
        let (items, input) = input.parse_comma(']');
        let input = input.expect(']');
        (items, input)
    }
}
