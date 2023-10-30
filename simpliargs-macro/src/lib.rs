use proc_macro::TokenStream;
use simpliargs_macro_impl;

#[proc_macro_attribute]
pub fn simpliargs(attr: TokenStream, body: TokenStream) -> TokenStream {
    simpliargs_macro_impl::simpliargs(attr.into(), body.into()).into()
}
