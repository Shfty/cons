
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Expr, LitInt, Token};

pub struct SizedListInput {
    expr: Expr,
    _semicolon: Token![;],
    count: LitInt,
}

impl Parse for SizedListInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;
        let semicolon = input.parse()?;
        let count = input.parse()?;

        Ok(SizedListInput {
            expr,
            _semicolon: semicolon,
            count,
        })
    }
}

pub enum ListInput {
    Literal(Punctuated<Expr, Token![,]>),
    Sized(Box<SizedListInput>),
}

impl Parse for ListInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(Token![;]) {
            let sized = input.parse::<SizedListInput>()?;
            Ok(ListInput::Sized(Box::new(sized)))
        } else {
            let literal = input.parse_terminated::<Expr, Token![,]>(Expr::parse)?;
            Ok(ListInput::Literal(literal))
        }
    }
}

pub fn impl_list(input: ListInput) -> TokenStream {
    let mut tokens = quote! { () };

    match input {
        ListInput::Literal(expressions) => {
            for expr in expressions.iter().rev() {
                tokens = quote! {
                    (#expr, #tokens)
                };
            }
        }
        ListInput::Sized(input) => {
            let SizedListInput {
                expr,
                _semicolon: _,
                count,
            } = input.as_ref();

            for _ in 0..count.base10_parse().unwrap() {
                tokens = quote! {
                    (#expr, #tokens)
                }
            }
        }
    }

    tokens.into()
}