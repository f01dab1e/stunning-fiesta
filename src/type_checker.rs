#![allow(dead_code, unused_imports)]

use self::Mode::{CheckType, Synthesize};
use crate::{
    syntax::{Expr, ExprData, ExprKind},
    table::AllocTable,
};

#[derive(Clone, Copy)]
struct Ty;

enum Mode {
    Synthesize,
    CheckType(Ty),
}

pub struct TypeChecker<'arena> {
    tables: &'arena AllocTable<Expr, ExprData>,

    bool_ty: Ty,
}

impl<'arena> TypeChecker<'arena> {
    pub fn new(tables: &'arena AllocTable<Expr, ExprData>) -> TypeChecker {
        TypeChecker { tables, bool_ty: Ty }
    }

    fn infer_variable(&self) -> Ty {
        todo!()
    }

    fn check_expr(&self, mode: Mode, expr: Expr) {
        let actual = self.infer_expr_ty(expr);
        if let CheckType(expected) = mode {
            self.equate(expr, actual, expected)
        }
    }

    fn infer_expr_ty(&self, expr: Expr) -> Ty {
        match self.tables.data(expr).kind {
            ExprKind::Integer(_) => todo!(),
            ExprKind::Float(_) => todo!(),
            ExprKind::Boolean(_) => todo!(),
            ExprKind::List(_) => todo!(),
            ExprKind::If(test, if_true, if_false) => {
                self.check_expr(CheckType(self.bool_ty), test);

                let ty = self.infer_variable();
                self.check_expr(CheckType(ty), if_true);
                if let Some(if_false) = if_false {
                    self.check_expr(CheckType(ty), if_false);
                }

                ty
            }
        }
    }

    fn equate(&self, _expr: Expr, _a: Ty, _b: Ty) {
        todo!()
    }
}
