use crate::table::RawKey;

pub type Expr = RawKey;

pub struct Span {
    pub lo: u32,
    pub hi: u32,
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
