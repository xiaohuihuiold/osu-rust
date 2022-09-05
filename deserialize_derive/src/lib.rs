extern crate proc_macro;

mod de;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DeserializeJson, attributes(json_value))]
pub fn deserialize_json_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    de::impl_deserialize_json(&ast)
}
