use std::collections::{ BTreeMap, HashMap };

use syn;

use attr::Attribute;

lazy_static! {
    static ref EMPTY: Attributes = Attributes { summary: "".into(), docs: "".into(), map: BTreeMap::new() };
}

pub struct Attributes {
    pub summary: String,
    pub docs: String,
    map: BTreeMap<String, Attribute>,
}

pub struct FieldAttributes {
    map: HashMap<syn::Ident, Attributes>,
}

impl Attributes {
    pub fn get(&self, key: &str) -> Option<&Attribute> {
        self.map.get(key)
    }

    pub fn get_bool(&self, key: &str) -> bool {
        self.map.get(key).map(|a| a.into()).unwrap_or(false)
    }
}

impl FieldAttributes {
    pub fn get(&self, field: &syn::Field) -> &Attributes {
        self.map.get(field.ident.as_ref().unwrap()).unwrap_or(&*EMPTY)
    }
}

fn extract_attrs_inner(attrs: &mut Vec<syn::Attribute>) -> Attributes {
    let mut stomps = BTreeMap::new();
    attrs.retain(|attr| {
        if let syn::MetaItem::List(ref ident, ref values) = attr.value {
            if ident == "stomp" {
                for value in values {
                    match *value {
                        syn::NestedMetaItem::MetaItem(ref item) => match *item {
                            syn::MetaItem::NameValue(ref name, ref value) => {
                                stomps.insert(name.to_string(), Attribute::new(name.to_string(), value.clone()));
                            }
                            syn::MetaItem::Word(ref name) => {
                                stomps.insert(name.to_string(), Attribute::new(name.to_string(), syn::Lit::Bool(true)));
                            }
                            syn::MetaItem::List(..) => {
                                panic!("Invalid stomp attribute {} unexpected sublist", quote!(#attr).to_string().replace(" ", ""));
                            }
                        },
                        syn::NestedMetaItem::Literal(_) => {
                            panic!("Invalid stomp attribute {} literal value not supported", quote!(#attr).to_string().replace(" ", ""));
                        },
                    }
                }
                false
            } else {
                true
            }
        } else {
            true
        }
    });

    let docs = attrs.iter()
        .filter(|a| a.is_sugared_doc)
        .map(|a| match a.value {
            syn::MetaItem::NameValue(_, syn::Lit::Str(ref doc, _)) => doc,
            _ => unreachable!(),
        })
        .fold(String::new(), |docs, line| docs + line.trim_left_matches('/').trim() + "\n");

    let index = docs.find("\n\n");
    let (summary, docs) = if let Some(index) = index {
        let (summary, docs) = docs.split_at(index);
        let (_, docs) = docs.split_at(2);
        (summary.into(), docs.into())
    } else {
        (docs, "".into())
    };

    Attributes { summary: summary, docs: docs, map: stomps }
}

/// Extracts all stomp attributes of the form #[stomp(i = V)]
pub fn extract_attrs(ast: &mut syn::MacroInput) -> (Attributes, FieldAttributes) {
    let root_attrs = extract_attrs_inner(&mut ast.attrs);
    let field_attrs = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref mut fields)) => {
            fields
                .iter_mut()
                .map(|field| (field.ident.clone().unwrap(), extract_attrs_inner(&mut field.attrs)))
                .collect()
        }
        syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("TODO: tuple struct unsupported msg")
        }
        syn::Body::Struct(syn::VariantData::Unit) | syn::Body::Enum(_) => {
            HashMap::new()
        }
    };
    (root_attrs, FieldAttributes { map: field_attrs })
}
