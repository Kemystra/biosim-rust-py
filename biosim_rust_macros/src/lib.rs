use proc_macro::TokenStream;
use quote::quote;
use syn::{self, ItemStruct, parse_macro_input, token, LitInt, Ident, Token, parenthesized};
use syn::parse::{Parse, ParseStream};


#[proc_macro_attribute]
pub fn neuron_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as ItemStruct);
    let neuron_id = parse_macro_input!(attr as LitInt);

    let struct_name = &item_ast.ident;
    if !struct_name.to_string().contains("Neuron") {
        panic!("Expected `Neuron` in identifier");
    }

    let generated_impl = quote! {
        #item_ast
        impl NeuronType for #struct_name {
            const ID: u8 = #neuron_id;
        }
    };

    TokenStream::from(generated_impl)
}
