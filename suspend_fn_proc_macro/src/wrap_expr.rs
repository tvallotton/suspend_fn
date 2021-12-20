use syn::{
    punctuated::Punctuated, spanned::Spanned, token, Expr, Ident, Path, PathArguments, PathSegment,
};

/// in this monster function is where the magic happens. It takes an
/// expression and wraps it like this:
///```rust
/// (no_await::Expr(#expr).as_future().await)
/// ```
pub fn wrap_expr(expr: Expr) -> Expr {
    let span = expr.span();
    Expr::Paren(syn::ExprParen {
        attrs: vec![],
        paren_token: token::Paren(span),
        expr: Box::new(Expr::Await(syn::ExprAwait {
            attrs: vec![],
            dot_token: token::Dot(span),
            await_token: token::Await(span),
            base: Box::new(Expr::MethodCall(syn::ExprMethodCall {
                attrs: vec![],
                dot_token: token::Dot(span),
                paren_token: token::Paren(span),
                receiver: Box::new(Expr::Call(syn::ExprCall {
                    attrs: vec![],
                    func: Box::new(Expr::Path(syn::ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: {
                                let mut segments = Punctuated::new();
                                segments.push(PathSegment {
                                    ident: Ident::new("no_await", span),
                                    arguments: PathArguments::None,
                                });
                                segments.push(PathSegment {
                                    ident: Ident::new("__private", span),
                                    arguments: PathArguments::None,
                                });
                                segments.push(PathSegment {
                                    ident: Ident::new("Expr", span),
                                    arguments: PathArguments::None,
                                });
                                segments
                            },
                        },
                    })),
                    paren_token: token::Paren(span),
                    args: {
                        let mut args = Punctuated::new();
                        args.push(expr);
                        args
                    },
                })),
                method: Ident::new("as_future", span),
                turbofish: None,
                args: Punctuated::new(),
            })),
        })),
    })
}
