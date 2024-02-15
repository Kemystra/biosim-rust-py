use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ItemEnum, parse_macro_input};


#[proc_macro_attribute]
pub fn enum_from_id(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as ItemEnum);
    let enum_name = &item_ast.ident;
    let match_arms = item_ast.variants
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, x)| quote! { #i => Some(#enum_name::#x), });

    let generated_impl = quote! {
        #item_ast

        impl #enum_name {
            pub fn from_id(id: usize) -> Option<Self> {
                match id {
                    #(#match_arms)*
                    _ => None
                }
            }
        }
    };

    TokenStream::from(generated_impl)
}
