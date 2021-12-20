use crate::AddAwait;
use syn::{Local, Stmt};

impl AddAwait for Stmt {
    fn add_await(&mut self) {
        match self {
            Stmt::Local(local) => local.add_await(),
            Stmt::Expr(expr) => expr.add_await(),
            Stmt::Semi(expr, _) => expr.add_await(),
            Stmt::Item(item) => item.add_await(),
        }
    }
}

impl AddAwait for Local {
    fn add_await(&mut self) {
        for (_, expr) in &mut self.init {
            expr.add_await()
        }
    }
}
