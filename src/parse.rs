use crate::{
    syntax::{Expr, ExprData},
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

    pub fn shift(&mut self) -> PResult<char> {
        let ch = match self.text.chars().next() {
            Some(c) => c,
            None => return Err(ParseError { message: "unexpected end of input".into() }),
        };
        self.text = &self.text[char::len_utf8(ch)..];
        Ok(ch)
    }

    pub fn shift_if(&mut self, f: impl Fn(char) -> bool) -> Option<char> {
        let ch = self.text.chars().next();

        match ch {
            Some(ch) if f(ch) => {
                let _ = self.shift();
                Some(ch)
            }
            Some(_) | None => None,
        }
    }

    pub fn at(&self, ch: char) -> bool {
        self.text.chars().next() == ch.into()
    }

    pub fn delimited<T>(
        &mut self,
        open: char,
        close: char,
        f: impl FnOnce(&mut Self) -> PResult<T>,
    ) -> PResult<T> {
        self.expect(open)?;
        let item = f(self)?;
        self.expect(close)?;

        Ok(item)
    }

    pub fn expect(&mut self, fragment: impl Fragment) -> PResult<()> {
        fragment.expect(self)
    }

    pub fn parse<T: Parse>(&mut self) -> PResult<T> {
        T::parse(self)
    }

    pub fn parse_comma<T: Parse>(&mut self, close: char) -> PResult<Vec<T>> {
        T::parse_comma(self, close)
    }

    pub fn accumulate(
        &mut self,
        start_test: impl Fn(char) -> bool,
        continue_test: impl Fn(char) -> bool + Copy,
        description: &str,
    ) -> PResult<String> {
        self.skip_trivia();
        let mut buffer = String::new();

        let first_char = self.shift()?;
        if !start_test(first_char) {
            return Err(ParseError { message: format!("expected {description}") });
        }
        buffer.push(first_char);

        while let Some(ch) = self.shift_if(continue_test) {
            buffer.push(ch);
        }

        Ok(buffer)
    }
}

pub trait Parse: Sized {
    fn parse(input: &mut Input) -> PResult<Self>;

    fn parse_comma(input: &mut Input, close: char) -> PResult<Vec<Self>> {
        let mut items = Vec::new();

        while !input.at(close) {
            input.skip_trivia();

            let item = input.parse()?;
            items.push(item);

            if input.at(close) {
                break;
            }

            input.expect(',')?;
        }

        Ok(items)
    }
}

pub trait Fragment {
    fn expect(self, input: &mut Input) -> PResult<()>;
}

impl Fragment for char {
    fn expect(self, input: &mut Input) -> PResult<()> {
        input.skip_trivia();

        if input.shift()? != self {
            return Err(ParseError { message: format!("expected `{self}`") });
        }

        Ok(())
    }
}
