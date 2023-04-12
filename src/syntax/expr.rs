use crate::{
    parse::{Input, PResult, Parse, ParseError},
    span::Span,
    syntax::Debug,
    tables::{Key, RawKey, Tables},
};

#[derive(Debug)]
pub struct ExprData {
    pub kind: ExprKind,
    pub span: Span,
}

key![Expr];

impl Debug for Expr {
    fn fmt(&self, tables: &Tables, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &tables.data(*self).kind {
            ExprKind::Integer(n) => write!(f, "{n}"),
            ExprKind::List(entries) => {
                let entries = entries.iter().map(|expr| expr.debug_with(tables));
                f.debug_list().entries(entries).finish()
            }
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
        input.require_unambiguous([input.try_parse(number), input.try_parse(list)], "expression")
    }
}

fn number(input: &mut Input) -> PResult<Expr> {
    let mut number = input.accumulate(
        |ch| ch.is_ascii_digit(),
        |ch| ch.is_ascii_digit() || ch == '_',
        "number",
    )?;

    if number.contains('_') {
        number.retain(|ch| ch != '_');
    }

    match number.parse() {
        Ok(t) => Ok(input.mk_expr(ExprKind::Integer(t))),
        Err(error) => Err(ParseError { message: format!("parse error: {error}") }),
    }
}

fn list(input: &mut Input) -> PResult<Expr> {
    input.parse().map(|list| input.mk_expr(ExprKind::List(list)))
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};
    use extension_trait::extension_trait;

    use crate::{
        parse::Parse,
        syntax::parse,
        tables::Tables,
        {syntax::Debug, Expr},
    };

    #[extension_trait]
    impl Assert for Tables {
        #[track_caller]
        fn assert_eq(&self, actual: impl Debug, expect: Expect) {
            let actual = actual.debug_with(self);
            expect.assert_debug_eq(&actual)
        }

        #[track_caller]
        fn assert_ast<T: Parse + Debug>(&mut self, text: &str, expect: Expect) {
            let actual = parse::<T>(text, self).unwrap();
            self.assert_eq(actual, expect)
        }
    }

    #[test]
    fn it_works() {
        let mut table = Tables::default();

        table.assert_ast::<Expr>(
            "42",
            expect![[r#"
                42
            "#]],
        );

        table.assert_ast::<Expr>(
            "[]",
            expect![[r#"
            []
        "#]],
        );
        table.assert_ast::<Expr>(
            "-- Мы прячем золото в трастовые фонды\n[]",
            expect![[r#"
            []
        "#]],
        );

        let error = parse::<Expr>("[", &mut table).unwrap_err();
        assert_eq!(error.message, "expected expression");

        table.assert_ast::<Expr>(
            "[40]",
            expect![[r#"
            [
                40,
            ]
        "#]],
        );

        table.assert_ast::<Expr>(
            "[40, 2, 42]",
            expect![[r#"
            [
                40,
                2,
                42,
            ]
        "#]],
        );

        table.assert_ast::<Expr>(
            "[4_000_000]",
            expect![[r#"
            [
                4000000,
            ]
        "#]],
        );
    }
}
