use std::collections::{ BTreeMap, HashMap };

use syn;

use from_lit::FromLit;

pub struct AttributesMap(BTreeMap<String, syn::Lit>);

pub struct Attributes {
    pub root: AttributesMap,
    pub fields: HashMap<syn::Ident, AttributesMap>,
}

impl AttributesMap {
    pub fn get<T>(&self, key: &str) -> Option<T> where T: FromLit {
        self.0.get(key).and_then(<T as FromLit>::from_lit)
    }
}

fn extract_attrs_inner(attrs: &mut Vec<syn::Attribute>) -> AttributesMap {
    let mut stomps = BTreeMap::new();
    attrs.retain(|attr| {
        if let syn::MetaItem::List(ref ident, ref values) = attr.value {
            if ident != "stomp" || values.len() != 1 {
                panic!("Invalid stomp attribute {}", quote!(#attr).to_string().replace(" ", ""));
            }
            if let syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref name, ref value)) = values[0] {
                stomps.insert(name.to_string(), value.clone());
            } else {
                panic!("Invalid stomp attribute {}", quote!(#attr).to_string().replace(" ", ""));
            }
            false
        } else {
            true
        }
    });
    AttributesMap(stomps)
}

/// Extracts all stomp attributes of the form #[stomp(i = V)]
pub fn extract_attrs(ast: &mut syn::MacroInput) -> Attributes {
    let root_attrs = extract_attrs_inner(&mut ast.attrs);
    let field_attrs = match ast.body {
        syn::Body::Enum(ref mut variants) => {
            variants
                .iter_mut()
                .map(|variant| (variant.ident.clone(), extract_attrs_inner(&mut variant.attrs)))
                .collect()
        }
        syn::Body::Struct(syn::VariantData::Struct(ref mut fields)) => {
            fields
                .iter_mut()
                .map(|field| (field.ident.clone().unwrap(), extract_attrs_inner(&mut field.attrs)))
                .collect()
        }
        syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("TODO: tuple struct unsupported msg")
        }
        syn::Body::Struct(syn::VariantData::Unit) => {
            HashMap::new()
        }
    };
    Attributes {
        root: root_attrs,
        fields: field_attrs,
    }
}
