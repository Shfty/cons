use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Pair, Data, DeriveInput, FieldsNamed, FieldsUnnamed, Ident};

pub fn impl_generic(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let data_struct = if let Data::Struct(data_struct) = input.data {
        data_struct
    } else {
        panic!("Generic can only be derived for structs");
    };

    let (named, fields) = match data_struct.fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => (true, named),
        syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (false, unnamed),
        syn::Fields::Unit => panic!("Generic can't be derived for unit structs"),
    };

    let (names, types): (Vec<_>, Vec<_>) = fields
        .into_pairs()
        .map(Pair::into_value)
        .map(|field| (field.ident, field.ty))
        .unzip();

    let names: Vec<_> = if named {
        names.into_iter().map(|name| quote! { #name }).collect()
    } else {
        (0..names.len())
            .map(|i| {
                let name = Ident::new(&format!("v{}", i), Span::call_site());
                quote! { #name }
            })
            .collect()
    };

    let mut type_iter = types.iter().rev();
    let first_type = type_iter.next().unwrap();
    let cons_list_type = type_iter.fold(quote! { (#first_type,) }, |acc, next| {
        quote! {
            ((#next,), #acc)
        }
    });

    let mut name_iter = names.iter().rev();
    let first_name = name_iter.next().unwrap();
    let cons_list_value = name_iter.fold(quote! { (#first_name,) }, |acc, next| {
        quote! {
            ((#next,), #acc)
        }
    });

    let struct_value = names.iter().fold(quote! {}, |acc, next| {
        quote! {
            #acc
            #next,
        }
    });

    let struct_into_cons_list = if named {
        quote! {
            #[allow(clippy::type_complexity)]
            impl StructIntoConsList for #name {
                type ConsList = #cons_list_type;

                fn into_cons_list(self) -> Self::ConsList {
                    let #name {
                        #struct_value
                    } = self;

                    #cons_list_value
                }
            }
        }
    } else {
        quote! {
            #[allow(clippy::type_complexity)]
            impl StructIntoConsList for #name {
                type ConsList = #cons_list_type;

                fn into_cons_list(self) -> Self::ConsList {
                    let #name(#struct_value) = self;
                    #cons_list_value
                }
            }
        }
    };

    let cons_list_into_struct = if named {
        quote! {
            impl ConsListIntoStruct<#name> for #cons_list_type {
                fn into_struct(self) -> #name {
                    let #cons_list_value = self;

                    #name {
                        #struct_value
                    }
                }
            }
        }
    } else {
        quote! {
            impl ConsListIntoStruct<#name> for #cons_list_type {
                fn into_struct(self) -> #name {
                    let #cons_list_value = self;
                    #name(#struct_value)
                }
            }
        }
    };

    let tokens = quote! {
        #struct_into_cons_list

        #cons_list_into_struct
    };

    tokens.into()
}
