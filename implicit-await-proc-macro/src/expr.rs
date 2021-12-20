use std::ops::Add;

use syn::Expr;

use crate::AddAwait;

impl AddAwait for Expr {
    fn add_await(&mut self) {
        todo!()
        // match self {
        //     Expr::Block(block) => block.block.add_await(),
        //     Expr::Array(array) => array.add_await(),
        //     Expr::Assign(assign) => assign.add_await(),
        //     Expr::AssignOp(assign_op) => assign_op.add_await(),
        //     Expr::Async(async_expr) => async_expr.add_await(),
        //     Expr::Await(await_expr) => await_expr.add_await(),
        //     Expr::Binary(binary) => binary.add_await(),
        //     Expr::Box(box_expr) => box_expr.add_await(),
        //     Expr::Break(break_expr) => break_expr.add_await(),
        //     Expr::Call(call_expr) => call_expr.add_await(),
        //     Expr::Cast(cast_expr) => cast_expr.add_await(),
        //     Expr::Closure(closure_expr) => closure_expr.add_await(),
        //     Expr::Continue(continue_expr) => continue_expr.add_await(),
        //     Expr::Field(field_expr) => field_expr.add_await(),
        //     Expr::ForLoop(for_loop_expr) => for_loop_expr.add_await(),
        //     Expr::Group(group_expr) => group_expr.add_await(),
        //     Expr::If(if_expr) => if_expr.add_await(),
        //     Expr::Index(index_expr) => index_expr.add_await(),
        //     Expr::Let(let_expr) => let_expr.add_await(),
        //     Expr::Lit(lit_expr) => lit_expr.add_await(),
        //     Expr::Loop(loop_expr) => loop_expr.add_await(),
        //     Expr::Macro(macro_expr) => macro_expr.add_await(),
        //     Expr::Match(match_expr) => match_expr.add_await(),
        //     Expr::MethodCall(method_call_expr) => method_call_expr.add_await(),
        //     Expr::Paren(paren_expr) => paren_expr.add_await(),
        //     Expr::Path(path_expr) => path_expr.add_await(),
        //     Expr::Range(range_expr) => range_expr.add_await(),
        //     Expr::Reference(reference_expr) => reference_expr.add_await(),
        //     Expr::Repeat(repeat_expr) => repeat_expr.add_await(),
        //     Expr::Return(return_expr) => return_expr.add_await(),
        //     Expr::Struct(struct_expr) => struct_expr.add_await(),
        //     Expr::Try(try_expr) => try_expr.add_await(),
        //     Expr::TryBlock(try_block_expr) => try_block_expr.add_await(),
        //     Expr::Tuple(tuple_expr) => tuple_expr.add_await(),
        //     Expr::Type(type_expr) => type_expr.add_await(),
        //     Expr::Unary(unary_expr) => unary_expr.add_await(),
        //     Expr::Unsafe(unsafe_expr) => unsafe_expr.add_await(),
        //     Expr::Verbatim(verbatim_expr) => verbatim_expr.add_await(),
        //     Expr::While(while_expr) => while_expr.add_await(),
        //     Expr::Yield(yield_expr) => yield_expr.add_await(),
        // }
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
        self.block.add_await();
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

// impl AddAwait for syn::ExprCall {
//     fn add_await(&mut self) {
//         self.func.add_await();
//         for expr in &mut self.args {
//             expr.add_await();
//         }

//         self = self; 
//     }
// }


// fn wrap_exrp(expr: Expr) -> Expr {
    

// }