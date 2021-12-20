#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

mod block;
mod expr;
mod item;
mod stmt;
mod wrap_expr;

#[proc_macro_attribute]
pub fn suspend(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let suspend: syn::ItemFn = parse_macro_input!(input);
    quote::quote!(#suspend).into()
}

trait AddAwait {
    fn add_await(&mut self);
}
