use proc_macro::TokenStream;
use quote::quote;
use syn::{self, braced, parse_macro_input, token, Field, Ident, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;




#[proc_macro_attribute]
pub fn neuron_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let old_item = item.clone();
    println!("{}", item.to_string());

    old_item
}



