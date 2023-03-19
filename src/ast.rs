use crate::{
    parse::{Input, Parse},
    span::Span,
    table::RawKey,
};

#[derive(Debug)]
pub struct Expr {
    pub raw: RawKey,
}

pub struct ExprData {
    pub kind: ExprKind,
    pub span: Span,
}

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
