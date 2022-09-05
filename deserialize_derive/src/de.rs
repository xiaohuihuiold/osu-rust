use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Meta, NestedMeta, Type};

const ATTR_JSON_VALUE: &str = "json_value";
const ATTR_JSON_VALUE_NAME: &str = "name";

pub fn impl_deserialize_json(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let data = match data {
        Data::Struct(data) => data,
        _ => panic!("只能作用Struct上"),
    };
    let fields = match &data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("只能作用Struct上"),
    };
    let fields = fields.named.iter();
    let mut blocks = Vec::new();
    for field in fields {
        let name = &field.ident;
        let ty = &field.ty;

        for attr in field.attrs.iter() {
            if attr.path.is_ident(ATTR_JSON_VALUE) {
                if let Ok(Meta::List(meta)) = attr.parse_meta() {
                    if let NestedMeta::Meta(Meta::NameValue(m)) = meta.nested.first().unwrap() {
                        if let Lit::Str(lit) = &m.lit {
                            println!("{:?}", lit.value());
                            let value = lit.value();
                            let is_option = match ty {
                                Type::Path(path) => {
                                    path.path.segments.first().unwrap().ident == "Option"
                                }
                                _ => false,
                            };
                            let is_bool = match ty {
                                Type::Path(path) => {
                                    path.path.segments.first().unwrap().ident == "bool"
                                }
                                _ => false,
                            };
                            println!("{:?}", is_option);
                            if is_bool {
                                blocks.push(quote! {
                                    #name:json.get(#value).map_or(default.#name, |e| e=="1")
                                });
                            } else if is_option {
                                blocks.push(quote! {
                                    #name: json.get(#value).map(String::from)
                                });
                            } else {
                                blocks.push(quote! {
                                    #name: json.get(#value).map_or(default.#name, |e| e.parse().unwrap())
                                });
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    let gen = quote! {
        impl DeserializeJson for #name {
            fn from_json(json: &HashMap<String, String>) -> Self{
                let default = Self::default();
                Self {
                    #(#blocks,)*
                }
            }
        }
    };
    TokenStream::from(gen)
}
