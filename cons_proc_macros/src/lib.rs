mod list;
mod generic;

use list::impl_list;
use generic::impl_generic;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn list(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens);
    impl_list(input)
}

#[proc_macro_derive(Generic)]
pub fn derive_generic(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens);
    impl_generic(input)
}
