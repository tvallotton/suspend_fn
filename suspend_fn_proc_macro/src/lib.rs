#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::parse::ParseBuffer;

mod block;
mod expr;
mod item;
mod stmt;
mod wrap_expr;

#[proc_macro_attribute]
pub fn suspend_fn(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut suspend: syn::ItemFn = parse_macro_input!(input);
    add_import(&mut suspend.block);
    suspend.add_await();
    quote::quote!(#[allow(unused_parens)] async #suspend).into()
}



#[proc_macro]
pub fn suspend(input: TokenStream) -> TokenStream {
    let suspend: BracelessBlock = parse_macro_input!(input);
    let mut suspend = suspend.0; 
    add_import(&mut suspend);
    suspend.add_await();
    quote::quote!(async #suspend).into()
}

#[proc_macro]
pub fn suspend_move(input: TokenStream) -> TokenStream {
    let suspend: BracelessBlock = parse_macro_input!(input);
    let mut suspend = suspend.0; 
    add_import(&mut suspend);
    suspend.add_await();
    quote::quote!({async move #suspend}).into()
}

trait AddAwait {
    fn add_await(&mut self);
}

/// Adds the folliwing line to the begining of the function:
///```
/// use suspend_fn::__private::AsFuture;
/// ```
fn add_import(block: &mut syn::Block) {
    block.stmts.insert(
        0,
        syn::parse_quote! {
            use suspend_fn::__private::AsFuture;
        },
    )
}

/// Block requires breaces. Because macros
/// aready include braces, this struct won't try to read them. 
struct BracelessBlock(syn::Block);
impl syn::parse::Parse for BracelessBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(syn::Block {
            brace_token: syn::token::Brace::default(),
            stmts: input.call(syn::Block::parse_within)?,
        }))
    }
}