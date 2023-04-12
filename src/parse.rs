use crate::{
    span::Span,
    syntax::{Expr, ExprData, ExprKind},
    tables::Tables,
};

pub type PResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

#[derive(Clone, Copy, Debug)]
pub struct Input<'text, 'tables> {
    pub text: &'text str,
    pub tables: &'tables Tables,
}

impl<'text, 'tables> Input<'text, 'tables> {
    pub fn new(text: &'text str, tables: &'tables Tables) -> Self {
        Self { text, tables }
    }

    pub fn mk_expr(&self, kind: ExprKind) -> Expr {
        self.tables.add(ExprData { kind, span: Span::default() })
    }

    pub fn fork(self) -> Input<'text, 'tables> {
        self
    }

    pub fn try_parse<T: Parse>(
        &self,
        f: impl FnOnce(&mut Self) -> PResult<T>,
    ) -> PResult<(T, Self)> {
        let mut snapshot = self.fork();
        f(&mut snapshot).map(|t| (t, snapshot))
    }

    pub fn require_unambiguous<const N: usize, T: std::fmt::Debug>(
        &mut self,
        results: [PResult<(T, Self)>; N],
        expected: &'static str,
    ) -> PResult<T> {
        let mut items: Vec<_> = results.into_iter().flatten().collect();

        match items.len() {
            0 => Err(ParseError { message: format!("expected {expected}") }),
            1 => {
                let (t, input) = items.pop().unwrap();
                *self = input;
                Ok(t)
            }
            _ => panic!("parsing ambiguity: {:#?}", items),
        }
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
        let mut snapshot = self.fork();

        match snapshot.shift().ok() {
            Some(ch) if f(ch) => {
                *self = snapshot;
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
