#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

mod block;
mod expr;
mod item;
mod stmt;
mod wrap_expr;

#[proc_macro_attribute]
pub fn suspend_fn(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut suspend: syn::ItemFn = parse_macro_input!(input);
    suspend.add_await();
    add_import(&mut suspend.block);
    quote::quote!(#[allow(unused_parens)] async #suspend).into()
}

#[proc_macro]
pub fn suspend(input: TokenStream) -> TokenStream {
    let mut suspend: syn::Block = parse_macro_input!(input);
    suspend.add_await();
    add_import(&mut suspend);
    quote::quote!(#[allow(unused_parens)] async #suspend).into()
}

#[proc_macro]
pub fn suspend_move(input: TokenStream) -> TokenStream {
    let mut suspend: syn::Block = parse_macro_input!(input);
    suspend.add_await();
    add_import(&mut suspend);
    quote::quote!(#[allow(unused_parens)] async move #suspend).into()
}

trait AddAwait {
    fn add_await(&mut self);
}

/// Adds the folliwing line to the begining of the function:
///```
/// use no_await::__private::AsFuture;
/// ```
fn add_import(block: &mut syn::Block) {
    block.stmts.insert(
        0,
        syn::parse_quote! {
            use no_await::__private::AsFuture;
        },
    )
}
