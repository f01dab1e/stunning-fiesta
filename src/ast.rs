use crate::{
    parse::{Input, Parse},
    span::Span,
    table::RawKey,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Expr {
    pub raw: RawKey,
}

pub struct ExprData {
    pub kind: ExprKind,
    pub span: Span,
}

#[allow(dead_code)]
pub enum Literal {
    /// `42`, `69`, etc
    Integer(u64),
    /// `integer-part.fractional-part`
    /// We convert float values into bits and that's how we don't need to deal with f32 and f64
    Float(u64),
    /// `true`, `false`
    Boolean(bool),
    /// `[]` or `[a, b, ...]`
    List(Vec<Expr>),
}

#[allow(dead_code)]
pub enum ExprKind {
    /// Literal.
    Literal(Literal),
    /// `if condition { block } [else { block }]`
    If(Expr, Expr, Option<Expr>),
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

#[cfg(test)]
mod tests {
    use crate::{parse::parse, table::AllocTable};

    use super::Expr;

    #[test]
    fn empty_vec() {
        let mut table = AllocTable::default();

        let items: Vec<Expr> = parse("[]", &mut table);
        assert_eq!(items, []);

        let items: Vec<Expr> = parse("-- Мы прячем золото в трастовые фонды\n[]", &mut table);
        assert_eq!(items, []);
    }
}
