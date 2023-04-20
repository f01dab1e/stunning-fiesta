#![allow(dead_code, unused_imports)]

mod unify;

use self::Mode::{CheckType, Synthesize};
use crate::{
    syntax::{Expr, ExprData, ExprKind},
    tables::{Key, RawKey, Tables},
};

key![Ty];

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TyData {
    Bool,
    Integer,
    Float,
    List(Ty),
}

enum Mode {
    Synthesize,
    CheckType(Ty),
}

pub struct TypeChecker<'tables> {
    tables: &'tables Tables,

    bool_ty: Ty,
}

impl<'tables> TypeChecker<'tables> {
    pub fn new(tables: &'tables Tables) -> TypeChecker {
        TypeChecker { tables, bool_ty: tables.add(TyData::Bool) }
    }

    fn infer_variable(&self) -> Ty {
        todo!()
    }

    fn equate(&self, _expr: Expr, _a: Ty, _b: Ty) {
        todo!()
    }

    fn check_expr(&self, mode: Mode, expr: Expr) {
        let actual = self.infer_expr(expr);
        if let CheckType(expected) = mode {
            self.equate(expr, actual, expected)
        }
    }

    fn infer_expr(&self, expr: Expr) -> Ty {
        match self.tables.data(expr).kind {
            ExprKind::Integer(_) => todo!(),
            ExprKind::Float(_) => todo!(),
            ExprKind::Boolean(_) => self.bool_ty,
            ExprKind::List(_) => todo!(),
            ExprKind::If(test, if_true, if_false) => {
                self.check_expr(CheckType(self.bool_ty), test);

                let ty = self.infer_variable();
                self.check_expr(CheckType(ty), if_true);

                match if_false {
                    Some(if_false) => self.check_expr(CheckType(ty), if_false),
                    None => todo!(),
                }

                ty
            }
        }
    }
}
