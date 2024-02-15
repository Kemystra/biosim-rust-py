use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ItemEnum, Ident, parse_macro_input};

// For god sake, I REALLY DON'T WANT TO HANDLE THIS
use convert_case::{Case, Casing};


#[proc_macro_attribute]
pub fn enum_from_id(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as ItemEnum);

    let enum_name = &item_ast.ident;
    let max_variant_num_ident = Ident::new(
        &format!("TOTAL_{}_VARIANT", enum_name
            .to_string()
            .from_case(Case::Pascal)
            .to_case(Case::UpperSnake)),
        enum_name.span()
    );

    let match_arms = item_ast.variants
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, x)| quote! { #i => Some(#enum_name::#x), });

    let total_variants = match_arms.len();

    let generated_impl = quote! {
        #item_ast

        pub const #max_variant_num_ident: usize = #total_variants;

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
