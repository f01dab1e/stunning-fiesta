use crate::{
    parse::{Input, PResult, Parse, ParseError},
    span::Span,
    table::{AllocTable, Key, RawKey},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Expr {
    pub raw: RawKey,
}

trait DebugWithTables {
    fn debug(&self, tables: &AllocTable<Expr, ExprData>) -> String;
}

impl DebugWithTables for Expr {
    fn debug(&self, tables: &AllocTable<Expr, ExprData>) -> String {
        match tables.data(*self).kind {
            ExprKind::Integer(n) => n.to_string(),
            _ => unreachable!(),
        }
    }
}

impl<T: DebugWithTables> DebugWithTables for Vec<T> {
    fn debug(&self, tables: &AllocTable<Expr, ExprData>) -> String {
        let items = self.iter().map(|item| item.debug(tables)).collect::<Vec<_>>().join(", ");
        format!("[{items}]")
    }
}

impl Key for Expr {
    fn from_usize(key: usize) -> Self {
        Expr { raw: RawKey::from_usize(key) }
    }

    fn as_usize(&self) -> usize {
        self.raw.as_usize()
    }
}

#[derive(Debug)]
pub struct ExprData {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ExprKind {
    /// `42`, `69`, etc
    Integer(u64),
    /// `integer-part.fractional-part`
    /// We convert float values into bits and that's how we don't need to deal with f32 and f64
    Float(u64),
    /// `true`, `false`
    Boolean(bool),
    /// `[]` or `[a, b, ...]`
    List(Vec<Expr>),
    /// `if condition { block } [else { block }]`
    If(Expr, Expr, Option<Expr>),
}

impl Parse for Expr {
    fn parse(input: &mut Input) -> PResult<Self> {
        let string =
            input.accumulate(|ch| ch.is_ascii_digit(), |ch| ch.is_ascii_digit(), "number")?;
        match string.parse() {
            Ok(t) => {
                let expr = ExprData { kind: ExprKind::Integer(t), span: Span::default() };
                Ok(input.tables.add(expr))
            }
            Err(error) => Err(ParseError { message: format!("parse error: {error}") }),
        }
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(input: &mut Input) -> PResult<Self> {
        input.expect('[')?;
        let items = input.parse_comma(']')?;
        input.expect(']')?;

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::DebugWithTables, parse::parse, table::AllocTable};

    use super::Expr;

    #[test]
    fn empty_vec() {
        let mut table = AllocTable::default();

        let items: Vec<Expr> = parse("[]", &mut table).unwrap();
        assert_eq!(items, []);

        let items: Vec<Expr> =
            parse("-- Мы прячем золото в трастовые фонды\n[]", &mut table).unwrap();
        assert_eq!(items, []);

        let error = parse::<Vec<Expr>>("[", &mut table).unwrap_err();
        assert_eq!(error.message, "unexpected end of input");

        // TODO: use https://crates.io/crates/expect-test
        let items: Vec<Expr> = parse("[40]", &mut table).unwrap();
        let actual = items.debug(&table);
        assert_eq!(actual, "[40]");

        let items: Vec<Expr> = parse("[40, 2, 42,]", &mut table).unwrap();
        let actual = items.debug(&table);
        assert_eq!(actual, "[40, 2, 42]");
    }
}
