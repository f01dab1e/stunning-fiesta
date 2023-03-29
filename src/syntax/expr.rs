use crate::{
    parse::{Input, PResult, Parse, ParseError},
    span::Span,
    syntax::Debug,
    table::{Key, RawKey},
    tables::Tables,
};

#[derive(Debug)]
pub struct ExprData {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Expr {
    pub raw: RawKey,
}

impl Key for Expr {
    fn from_usize(key: usize) -> Expr {
        Expr { raw: RawKey::from_usize(key) }
    }

    fn as_usize(&self) -> usize {
        self.raw.as_usize()
    }
}

impl Debug for Expr {
    fn fmt(&self, tables: &Tables, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match tables.data(*self).kind {
            ExprKind::Integer(n) => write!(f, "{n}"),
            _ => unreachable!(),
        }
    }
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
        let mut number = input.accumulate(
            |ch| ch.is_ascii_digit(),
            |ch| ch.is_ascii_digit() || ch == '_',
            "number",
        )?;

        if number.contains('_') {
            number = number.chars().filter(|&ch| ch != '_').collect();
        }

        match number.parse() {
            Ok(t) => {
                let expr = ExprData { kind: ExprKind::Integer(t), span: Span::default() };
                Ok(input.tables.add(expr))
            }
            Err(error) => Err(ParseError { message: format!("parse error: {error}") }),
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};
    use extension_trait::extension_trait;

    use crate::{
        syntax::parse,
        tables::Tables,
        {syntax::Debug, Expr},
    };

    #[extension_trait]
    impl Assert for Tables {
        fn assert_eq(&self, actual: impl Debug, expect: Expect) {
            let actual = actual.debug_with(self);
            expect.assert_debug_eq(&actual)
        }
    }

    #[test]
    fn it_works() {
        let mut table = Tables::default();

        let items: Vec<Expr> = parse("[]", &mut table).unwrap();
        assert_eq!(items, []);

        let items: Vec<Expr> =
            parse("-- Мы прячем золото в трастовые фонды\n[]", &mut table).unwrap();
        assert_eq!(items, []);

        let error = parse::<Vec<Expr>>("[", &mut table).unwrap_err();
        assert_eq!(error.message, "unexpected end of input");

        let items: Vec<Expr> = parse("[40]", &mut table).unwrap();
        table.assert_eq(
            items,
            expect![[r#"
            [
                40,
            ]
        "#]],
        );

        let items: Vec<Expr> = parse("[40, 2, 42,]", &mut table).unwrap();
        table.assert_eq(
            items,
            expect![[r#"
            [
                40,
                2,
                42,
            ]
        "#]],
        );

        let items: Vec<Expr> = parse("[4_000_000]", &mut table).unwrap();
        table.assert_eq(
            items,
            expect![[r#"
            [
                4000000,
            ]
        "#]],
        );
    }
}
