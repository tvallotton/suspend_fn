use crate::wrap_expr::wrap_expr;

use syn::Expr;

use crate::AddAwait;

impl AddAwait for Expr {
    fn add_await(&mut self) {
        match self {
            Expr::Block(block) => block.block.add_await(),
            Expr::Array(array) => array.add_await(),
            Expr::Assign(assign) => assign.add_await(),
            Expr::AssignOp(assign_op) => assign_op.add_await(),
            Expr::Async(_async_expr) => take_mut::take(self, wrap_expr),
            Expr::Await(await_expr) => {
                await_expr.add_await();
                take_mut::take(self, wrap_expr)
            }
            Expr::Binary(binary) => {
                binary.add_await();
                take_mut::take(self, wrap_expr)
            }
            Expr::Box(box_expr) => box_expr.add_await(),
            Expr::Break(break_expr) => break_expr.add_await(),
            Expr::Call(call_expr) => {
                call_expr.add_await();
                take_mut::take(self, wrap_expr);
            }
            Expr::Cast(cast_expr) => cast_expr.add_await(),
            Expr::Closure(closure_expr) => closure_expr.add_await(),
            Expr::Continue(continue_expr) => continue_expr.add_await(),
            Expr::Field(field_expr) => {
                field_expr.add_await();
                take_mut::take(self, wrap_expr)
            }
            Expr::ForLoop(for_loop_expr) => for_loop_expr.add_await(),
            Expr::Group(group_expr) => group_expr.add_await(),
            Expr::If(if_expr) => if_expr.add_await(),
            Expr::Index(index_expr) => index_expr.add_await(),
            Expr::Let(let_expr) => let_expr.add_await(),
            Expr::Lit(lit_expr) => lit_expr.add_await(),
            Expr::Loop(loop_expr) => loop_expr.add_await(),
            Expr::Macro(macro_expr) => macro_expr.add_await(),
            Expr::Match(match_expr) => match_expr.add_await(),
            Expr::MethodCall(method_call_expr) => {
                method_call_expr.add_await();
                take_mut::take(self, wrap_expr);
            }
            Expr::Paren(paren_expr) => paren_expr.add_await(),
            Expr::Path(path_expr) => {
                path_expr.add_await();
                take_mut::take(self, wrap_expr)
            }
            Expr::Range(range_expr) => range_expr.add_await(),
            Expr::Reference(reference_expr) => reference_expr.add_await(),
            Expr::Repeat(repeat_expr) => repeat_expr.add_await(),
            Expr::Return(return_expr) => return_expr.add_await(),
            Expr::Struct(struct_expr) => struct_expr.add_await(),
            Expr::Try(try_expr) => {
                try_expr.add_await();

                take_mut::take(self, wrap_expr)
            }
            Expr::TryBlock(try_block_expr) => try_block_expr.add_await(),
            Expr::Tuple(tuple_expr) => tuple_expr.add_await(),
            Expr::Type(type_expr) => type_expr.add_await(),
            Expr::Unary(unary_expr) => unary_expr.add_await(),
            Expr::Unsafe(unsafe_expr) => unsafe_expr.add_await(),
            Expr::Verbatim(_) => (),
            Expr::While(while_expr) => while_expr.add_await(),
            Expr::Yield(yield_expr) => yield_expr.add_await(),
            _ => todo!(),
        }
    }
}

impl AddAwait for syn::ExprArray {
    fn add_await(&mut self) {
        for expr in &mut self.elems {
            expr.add_await()
        }
    }
}

impl AddAwait for syn::ExprAssign {
    fn add_await(&mut self) {
        self.right.add_await();
    }
}

impl AddAwait for syn::ExprAssignOp {
    fn add_await(&mut self) {
        self.right.add_await();
    }
}

impl AddAwait for syn::ExprAsync {
    fn add_await(&mut self) {
        /*
            inner async blocks are not covered
        */
    }
}
impl AddAwait for syn::ExprAwait {
    fn add_await(&mut self) {
        self.base.add_await();
    }
}

impl AddAwait for syn::ExprBinary {
    fn add_await(&mut self) {
        self.left.add_await();
        self.right.add_await();
    }
}

impl AddAwait for syn::ExprBox {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}

impl AddAwait for syn::ExprBreak {
    fn add_await(&mut self) {
        if let Some(expr) = &mut self.expr {
            expr.add_await();
        }
    }
}

impl AddAwait for syn::ExprCall {
    fn add_await(&mut self) {
        self.func.add_await();
        for expr in &mut self.args {
            expr.add_await();
        }
    }
}

impl AddAwait for syn::ExprCast {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}

impl AddAwait for syn::ExprClosure {
    fn add_await(&mut self) {
        /*
            closures are not futures
        */
    }
}
impl AddAwait for syn::ExprContinue {
    fn add_await(&mut self) {
        /*
            continue is not a future
        */
    }
}

impl AddAwait for syn::ExprField {
    fn add_await(&mut self) {
        self.base.add_await();
    }
}

impl AddAwait for syn::ExprForLoop {
    fn add_await(&mut self) {
        self.expr.add_await();
        self.body.add_await();
    }
}
impl AddAwait for syn::ExprGroup {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}
impl AddAwait for syn::ExprIf {
    fn add_await(&mut self) {
        self.cond.add_await();
        self.then_branch.add_await();
        if let Some(else_branch) = &mut self.else_branch {
            else_branch.1.add_await();
        }
    }
}
impl AddAwait for syn::ExprIndex {
    fn add_await(&mut self) {
        self.expr.add_await();
        self.index.add_await();
    }
}

impl AddAwait for syn::ExprLet {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}
impl AddAwait for syn::ExprLit {
    fn add_await(&mut self) {
        /*
            literals are not futures
        */
    }
}

impl AddAwait for syn::ExprLoop {
    fn add_await(&mut self) {
        self.body.add_await();
    }
}
impl AddAwait for syn::ExprMacro {
    fn add_await(&mut self) {
        /*
            macros are not futures
        */
    }
}

impl AddAwait for syn::ExprMatch {
    fn add_await(&mut self) {
        self.expr.add_await();
        for arm in &mut self.arms {
            arm.add_await();
        }
    }
}

impl AddAwait for syn::Arm {
    fn add_await(&mut self) {
        for (_, expr) in &mut self.guard {
            expr.add_await();
        }
        self.body.add_await();
    }
}

impl AddAwait for syn::ExprMethodCall {
    fn add_await(&mut self) {
        self.receiver.add_await();
        for expr in &mut self.args {
            expr.add_await();
        }
    }
}

impl AddAwait for syn::ExprParen {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}

impl AddAwait for syn::ExprPath {
    fn add_await(&mut self) {
        /*
            paths are not futures
        */
    }
}

impl AddAwait for syn::ExprRange {
    fn add_await(&mut self) {
        if let Some(from) = &mut self.from {
            from.add_await();
        }
        if let Some(to) = &mut self.to {
            to.add_await();
        }
    }
}

impl AddAwait for syn::ExprReference {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}

impl AddAwait for syn::ExprRepeat {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}

impl AddAwait for syn::ExprReturn {
    fn add_await(&mut self) {
        if let Some(expr) = &mut self.expr {
            expr.add_await();
        }
    }
}
/* TODO: decide what to do with `rest` */
impl AddAwait for syn::ExprStruct {
    fn add_await(&mut self) {
        for field in &mut self.fields {
            field.expr.add_await();
        }
    }
}

impl AddAwait for syn::ExprTry {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}

impl AddAwait for syn::ExprTryBlock {
    fn add_await(&mut self) {
        self.block.add_await();
    }
}

impl AddAwait for syn::ExprUnary {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}
/* NOT CHECKED */
impl AddAwait for syn::ExprUnsafe {
    fn add_await(&mut self) {
        self.block.add_await();
    }
}
impl AddAwait for syn::ExprWhile {
    fn add_await(&mut self) {
        self.cond.add_await();
        self.body.add_await();
    }
}
impl AddAwait for syn::ExprYield {
    fn add_await(&mut self) {
        if let Some(expr) = &mut self.expr {
            expr.add_await();
        }
    }
}
impl AddAwait for syn::ExprTuple {
    fn add_await(&mut self) {
        for expr in &mut self.elems {
            expr.add_await();
        }
    }
}

impl AddAwait for syn::ExprType {
    fn add_await(&mut self) {
        self.expr.add_await();
    }
}
