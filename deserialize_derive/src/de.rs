use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Meta, MetaList, NestedMeta, Type};

const ATTR_JSON_VALUE: &str = "json_value";
const ATTR_JSON_VALUE_NAME: &str = "name";

#[derive(Default, Debug)]
struct JsonValue {
    name: Option<String>,
}

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
        let path = if let Type::Path(path) = ty {
            path
        } else {
            continue;
        };
        let ident = path.path.segments.last();
        let ident = if let Some(ident) = ident {
            ident
        } else {
            continue;
        };
        let type_name = ident.ident.to_string();
        for attr in field.attrs.iter() {
            if !attr.path.is_ident(ATTR_JSON_VALUE) {
                continue;
            }
            let meta = if let Ok(Meta::List(meta)) = attr.parse_meta() {
                meta
            } else {
                break;
            };
            let json_value = get_json_value(&meta);
            let json_name = json_value.name;
            let block = match type_name.as_str() {
                "Option" => {
                    quote! {#name: json.get(#json_name).map(String::from)}
                }
                "bool" => {
                    quote! {#name:json.get(#json_name).map_or(default.#name, |e| e == "1")}
                }
                _ => {
                    quote! {#name: json.get(#json_name).map_or(default.#name, |e| e.parse().unwrap_or(default.#name))}
                }
            };
            blocks.push(block);
            break;
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

fn get_json_value(meta: &MetaList) -> JsonValue {
    let mut json_value = JsonValue::default();
    let mut attrs = meta.nested.iter();
    while let Some(NestedMeta::Meta(Meta::NameValue(name_value))) = attrs.next() {
        let lit = if let Lit::Str(lit) = &name_value.lit {
            lit
        } else {
            continue;
        };
        let name = name_value.path.segments.last().unwrap().ident.to_string();
        match name.as_str() {
            ATTR_JSON_VALUE_NAME => {
                json_value.name = Some(lit.value().clone());
            }
            _ => {
                continue;
            }
        }
    }
    json_value
}
